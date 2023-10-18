use crate::{db::Pool, www::login::MakeAccount};
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::BTreeMap;
use totp_rs::TOTP;
use uuid::Uuid;

pub struct FakeGame {
	users: Vec<Uuid>,
	games: Vec<Uuid>,
	pool: Pool,
}

pub struct GameSummary {
	pub change_in_balance: i128,
	pub user_balances: BTreeMap<Uuid, u64>,
}

impl FakeGame {
	pub fn new(pool: Pool) -> Self {
		Self {
			pool,
			users: Vec::new(),
			games: Vec::new(),
		}
	}

	pub async fn add_users(&mut self, amt: usize, balance: i64) -> Result<(), sqlx::Error> {
		for _ in 0..amt {
			let uuid = Uuid::new_v4();
			let totp = TOTP::default();
			let mk_user = MakeAccount {
				user_id: uuid,
				email: format!("fake_user_{uuid}@example.com"),
				totp_secret: totp,
			};
			self.pool.register_user(&mk_user).await?;
			self.users.push(uuid);
			self.pool.magic_money(uuid, balance).await?;
		}

		Ok(())
	}

	pub async fn play_game(&mut self) -> Result<GameSummary, sqlx::Error> {
		let mut rng = thread_rng();
		let current_balance = self.pool.total_balance().await?;
		// Choose which players will play and which will gamble
		let master = self.users.choose(&mut rng).unwrap();
		let players: Vec<_> = self
			.users
			.choose_multiple(&mut rng, self.users.len() / 10)
			.filter(|u| u != &master)
			.collect();
		let gamblers: Vec<_> = self
			.users
			.iter()
			.filter(|u| !players.contains(u))
			.filter(|u| u != &master)
			.collect();

		// Set up the game
		let game = self
			.pool
			.create_game(*master, String::from("Fake Game"))
			.await?;
		self.games.push(game);
		for player in &players {
			self.pool.register_for_game(game, **player).await?;
		}
		for gambler in &gamblers {
			let balance = self.pool.balance_of(**gambler).await?;
			let choice = **players.choose(&mut rng).unwrap();
			self.pool
				.wager_on_game(game, **gambler, choice, rng.gen_range(0..balance))
				.await?;
		}
		for player in &players {
			self.pool.score_on(game, **player, rng.gen()).await?;
		}
		self.pool.end_game(game).await?;
		// Output game summary
		let balance_after_game = self.pool.total_balance().await?;
		let change_in_balance = (current_balance as i128) - (balance_after_game as i128);
		let mut user_balances = BTreeMap::new();
		for user in &self.users {
			let balance = self.pool.balance_of(*user).await?;
			user_balances.insert(*user, balance);
		}
		Ok(GameSummary {
			change_in_balance,
			user_balances,
		})
	}

	pub async fn cleanup(&mut self) {
		for id in &self.users {
			if let Err(e) = self.pool.delete_user(*id).await {
				eprintln!("Error deleting user: {e}");
			}
		}
		for id in &self.games {
			if let Err(e) = self.pool.delete_game(*id).await {
				eprintln!("Error deleting game: {e}");
			}
		}
		self.games.clear();
		self.users.clear();
	}
}

impl Drop for FakeGame {
	fn drop(&mut self) {
		if !self.users.is_empty() || !self.games.is_empty() {
			panic!("You need to call cleanup()!");
		}
	}
}
