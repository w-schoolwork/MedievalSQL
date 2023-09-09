use sqlx::PgPool;

use crate::www::login::{LoginAttempt, LoginStatus, MakeAccount};

#[derive(Clone)]
pub struct Pool(PgPool);

#[allow(clippy::missing_errors_doc)]
impl Pool {
	pub async fn from_url(url: &str) -> Result<Self, sqlx::Error> {
		let pool = PgPool::connect(url).await?;
		sqlx::migrate!().run(&pool).await?;
		Ok(Self(pool))
	}

	pub async fn login(&self, attempt: &LoginAttempt) -> Result<LoginStatus, sqlx::Error> {
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
