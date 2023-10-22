use std::time::SystemTimeError;

use crate::www::login::{LoginAttempt, LoginStatus, MakeAccount};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use sqlx::{migrate::MigrateError, types::chrono::{DateTime, Local}, PgPool};
use totp_rs::{TotpUrlError, TOTP};
use uuid::Uuid;

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
	ClockError(#[from] SystemTimeError),
	#[error("Problem decoding base64")]
	Base64DecError(#[from] base64::DecodeError),
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
			pub totp_secret: String,
		}
		let Some(UserInfo {
			user_id,
			totp_secret,
		}) = sqlx::query_as!(
			UserInfo,
			"SELECT user_id, totp_secret FROM users WHERE users.email = $1",
			attempt.email.clone()
		)
		.fetch_optional(&self.0)
		.await?
		else {
			return Ok(LoginStatus::BadUser);
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
		let data = (0..128).map(|_| rand::random()).collect::<Vec<u8>>();
		let text = STANDARD_NO_PAD.encode(&data);
		sqlx::query!("INSERT INTO session VALUES ($1, $2);", id, text.clone())
			.execute(&self.0)
			.await?;
		Ok(text)
	}

	pub async fn check_session(&self, secret: &str) -> Result<Option<Uuid>, Error> {
		struct Session {
			pub user_id: Uuid,
		}
		let session: Option<Session> = sqlx::query_as!(
			Session,
			"SELECT user_id FROM session
			WHERE secret = $1",
			secret
		)
		.fetch_optional(&self.0)
		.await?;
		Ok(session.map(|s| s.user_id))
	}

	pub async fn clobber_session(&self, secret: &str) -> Result<(), sqlx::Error> {
		sqlx::query!("DELETE FROM session WHERE secret = $1", secret)
			.execute(&self.0)
			.await?;
		Ok(())
	}

	pub async fn email_of(&self, id: Uuid) -> Result<Option<String>, Error> {
		struct Out {
			email: String,
		}
		let out = sqlx::query_as!(
			Out,
			"SELECT email FROM users
			WHERE user_id = $1",
			id
		)
		.fetch_optional(&self.0)
		.await?;
		Ok(out.map(|o| o.email))
	}

	pub async fn register_user(&self, account: &MakeAccount) -> Result<String, sqlx::Error> {
		sqlx::query!(
			"INSERT INTO users VALUES ($1, $2, $3)",
			account.user_id,
			account.email,
			account.totp_secret.get_url()
		)
		.execute(&self.0)
		.await?;
		self.mk_session(account.user_id).await
	}

	pub async fn delete_user(&self, user: Uuid) -> Result<(), sqlx::Error> {
		sqlx::query!("DELETE FROM users WHERE user_id = $1;", user)
			.execute(&self.0)
			.await?;
		Ok(())
	}

	pub async fn total_balance(&self) -> Result<u64, sqlx::Error> {
		unimplemented!()
	}

	pub async fn magic_money(&self, user: Uuid, amount: i64) -> Result<(), sqlx::Error> {
		sqlx::query!("INSERT INTO deposit VALUES ($1, $2);", user, amount)
			.execute(&self.0)
			.await?;
		Ok(())
	}

	pub async fn create_game(
		&self,
		master: Uuid,
		name: String,
		flavor: String,
		date: DateTime<Local>,
	) -> Result<Uuid, sqlx::Error> {
		let id = Uuid::new_v4();
		sqlx::query!(
			"INSERT INTO events VALUES ($1, $3, $5, $4, $2, false)",
			id,
			master,
			name,
			flavor,
			date
		)
		.execute(&self.0)
		.await?;
		Ok(id)
	}

	pub async fn delete_game(&self, game: Uuid) -> Result<(), sqlx::Error> {
		sqlx::query!("DELETE FROM events WHERE event_id = $1", game)
			.execute(&self.0)
			.await?;
		Ok(())
	}

	pub async fn register_for_game(&self, game: Uuid, player: Uuid) -> Result<(), sqlx::Error> {
		self.wager_on_game(game, player, player, 10).await?;
		unimplemented!()
	}

	pub async fn wager_on_game(
		&self,
		_game: Uuid,
		_user: Uuid,
		_winner: Uuid,
		_amt: u64,
	) -> Result<(), sqlx::Error> {
		unimplemented!()
	}

	pub async fn balance_of(&self, _user: Uuid) -> Result<u64, sqlx::Error> {
		unimplemented!()
	}

	pub async fn score_on(&self, _game: Uuid, _user: Uuid, _score: u64) -> Result<(), sqlx::Error> {
		unimplemented!()
	}

	pub async fn end_game(&self, _game: Uuid) -> Result<(), sqlx::Error> {
		unimplemented!()
	}
}

impl std::ops::Deref for Pool {
	type Target = PgPool;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
