use std::{ops::Deref, time::SystemTimeError};

use sqlx::{PgPool, migrate::MigrateError};
use totp_rs::{TOTP, TotpUrlError};
use uuid::Uuid;

use crate::www::login::{LoginAttempt, LoginStatus, MakeAccount};

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("An error occurred when communicating with the database.")]
	DB(#[from] sqlx::Error),
	#[error("An unexpected data condition was encountered.")]
	BrokenContract,
	#[error("Problem working with TOTP URLs")]
	TOTPError(#[from] TotpUrlError),
	#[error("Problem running migrations")]
	MigrateError(#[from] MigrateError),
	#[error("Problem with clock")]
	ClockError(#[from] SystemTimeError)
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
		let totp_secret = TOTP::from_url(totp_secret)?;
		let ok = totp_secret.check_current(&attempt.totp)?;
		if ok {
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
