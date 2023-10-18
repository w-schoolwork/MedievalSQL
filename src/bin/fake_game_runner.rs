use clap::Parser;
use medieval_sql::{db::Pool, fake_games::FakeGame};

#[derive(Parser)]
pub struct Args {
	pub user_amt: usize,
	pub starting_balance: i64,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;
	let url = std::env::var("DATABASE_URL")?;
	let pool = Pool::from_url(&url).await?;
	let Args {
		user_amt,
		starting_balance,
	} = Args::parse();

	let mut fake_games = FakeGame::new(pool.clone());
	fake_games.add_users(user_amt, starting_balance).await?;

	fake_games.cleanup().await;
	Ok(())
}
