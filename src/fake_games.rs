use crate::{db::Pool, www::login::MakeAccount};
use num_traits::cast::ToPrimitive;
use rand::{seq::SliceRandom, thread_rng, Rng};
use sqlx::types::{chrono::Local, BigDecimal};
use std::{collections::BTreeMap, ops::Deref, time::Duration};
use totp_rs::TOTP;
use uuid::Uuid;

pub struct FakeGame {
	users: Vec<Uuid>,
	games: Vec<Uuid>,
	pool: Pool,
}

pub struct GameSummary {
	pub change_in_balance: BigDecimal,
	pub user_balances: BTreeMap<Uuid, BigDecimal>,
}

#[allow(clippy::missing_errors_doc)]
impl FakeGame {
	#[must_use]
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
			self.pool
				.magic_money(uuid, BigDecimal::from(balance))
				.await?;
		}

		Ok(())
	}

	pub async fn play_game(&mut self) -> Result<GameSummary, crate::db::Error> {
		let mut rng = thread_rng();
		let balance_at_start = self.pool.total_balance().await?;
		// Choose which players will play and which will gamble
		let master = self.users.choose(&mut rng).unwrap();
		let players: Vec<_> = self
			.users
			.choose_multiple(&mut rng, (self.users.len() / 5).max(3))
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
			.create_game(
				*master,
				String::from("Fake Game"),
				String::new(),
				Local::now() - Duration::from_secs(30),
			)
			.await?;
		self.games.push(game);
		for player in &players {
			self.pool.register_for_game(game, **player).await?;
		}
		for gambler in &gamblers {
			let balance = self
				.pool
				.balance_of(**gambler)
				.await?
				.round(0)
				.to_i64()
				.unwrap_or(0);
			if balance <= 0 {
				println!("{gambler} went bankrupt");
				continue;
			}
			let choice = **players.choose(&mut rng).unwrap();
			self.pool
				.wager_on_game(
					game,
					**gambler,
					choice,
					BigDecimal::from(rng.gen_range(0..balance)),
				)
				.await?;
		}
		for player in &players {
			self.pool
				.score_on(game, **player, rng.gen_range(0..=i64::MAX))
				.await?;
		}
		self.pool.end_game(game).await?;
		// Output game summary
		let balance_at_end = self.pool.total_balance().await?;
		let change_in_balance = balance_at_end / balance_at_start;
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
}

impl Drop for FakeGame {
	fn drop(&mut self) {
		let users = self.users.clone();
		let games = self.games.clone();
		let pool = self.pool.clone();
		let job = tokio::spawn(async move {
			let pool = pool;
			for id in &users {
				if let Err(e) = pool.delete_user(*id).await {
					eprintln!("Error deleting user: {e}");
				}
			}
			for id in &games {
				if let Err(e) = pool.delete_game(*id).await {
					eprintln!("Error deleting game: {e}");
				}
			}
		});
		while !job.is_finished() {
			std::thread::yield_now();
		}
	}
}

impl Deref for FakeGame {
	type Target = Pool;

	fn deref(&self) -> &Self::Target {
		&self.pool
	}
}
