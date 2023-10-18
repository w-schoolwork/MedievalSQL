use clap::Parser;
use medieval_sql::{db::Pool, fake_games::FakeGame};

#[derive(Parser)]
pub struct Args {
	pub user_amt: usize,
	pub starting_balance: i64,
	pub game_amt: usize,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;
	let url = std::env::var("DATABASE_URL")?;
	let pool = Pool::from_url(&url).await?;
	let Args {
		user_amt,
		starting_balance,
		game_amt,
	} = Args::parse();

	let mut fake_games = FakeGame::new(pool.clone());
	fake_games.add_users(user_amt, starting_balance).await?;

	for i in 0..game_amt {
		let current_status = fake_games.play_game().await?;
		println!(
			"Game {i}/{game_amt}; currency in play changed by {} (should be 0).",
			current_status.change_in_balance
		);
	}

	Ok(())
}
