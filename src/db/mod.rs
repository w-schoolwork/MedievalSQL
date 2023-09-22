use std::ops::Deref;

use sqlx::PgPool;
use totp_rs::TOTP;
use uuid::Uuid;

use crate::www::login::{LoginAttempt, LoginStatus, MakeAccount};

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("An error occurred when communicating with the database.")]
	DB(#[from] sqlx::Error),
	#[error("An unexpected data condition was encountered.")]
	BrokenContract,
}

#[derive(Clone)]
pub struct Pool(PgPool);

#[allow(clippy::missing_errors_doc)]
impl Pool {
	pub async fn from_url(url: &str) -> Result<Self, Error> {
		let pool = PgPool::connect(url).await?;
		sqlx::migrate!().run(&pool).await?;
		Ok(Self(pool))
	}

	pub async fn login(&self, attempt: &LoginAttempt) -> Result<LoginStatus, Error> {
		struct UserInfo {
			pub user_id: Uuid,
			pub email: String,
			pub totp_secret: String,
		}
		let Some(UserInfo {user_id, email, totp_secret}) = sqlx::query_as!(
			UserInfo,
			"SELECT user_id, email, totp_secret FROM users WHERE users.email = $1",
			attempt.email.clone()
		).fetch_optional(&self.0).await? else {
			return Ok(LoginStatus::BadUser)
		};
		let Ok(totp_secret) = TOTP::from_url(totp_secret) else {
			tracing::error!("user {email} somehow got a bad TOTP stored?");
			return Err()
		};
		let Ok(ok) = totp_secret.check_current(&attempt.totp) else {
			tracing::error!("couldn't check TOTP for {email} because of bad system time?");
			return Err()
		}
		if  {
			Ok(LoginStatus::Success(self.mk_session(user_id).await?))
		} else {
			Ok(LoginStatus::BadTOTP)
		}
	}

	async fn mk_session(&self, id: Uuid) -> Result<String, sqlx::Error> {
		unimplemented!()
	}

	pub async fn register(&self, account: &MakeAccount) -> Result<String, sqlx::Error> {
		unimplemented!()
	}
}

impl std::ops::Deref for Pool {
	type Target = PgPool;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
