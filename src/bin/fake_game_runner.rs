use std::time::Instant;

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

	println!("game	balance	change	elapsed	users");
	for i in 0..game_amt {
		let start = Instant::now();
		let current_status = fake_games.play_game().await?;
		// println!(
		// 	"Game {i}/{game_amt}; currency in play {} changed by a factor of {}x (should be 1).",
		// 	current_status.balance_at_end.round(5),
		// 	current_status.change_in_balance.round(5)
		// );
		println!(
			"{i}	{}	{}	{}	{}",
			current_status.balance_at_end.round(10),
			current_status.change_in_balance.round(10),
			start.elapsed().as_secs_f32(),
			current_status
				.user_balances
				.values()
				.map(|b| format!("{}\t", b.round(0)))
				.collect::<String>()
		);
		// println!("Look at the database, then press enter to continue.");
		// stdin().lines().next();
	}

	// println!("Look at the database, then press enter to clean up.");
	// stdin().lines().next();

	Ok(())
}
