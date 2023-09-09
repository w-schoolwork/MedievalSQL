use medieval_sql::{db::Pool, RocketExt};
use rocket::Rocket;

#[rocket::main]
async fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;
	// the async main macro is a bit wack so move into another function asap
	inner().await
}

async fn inner() -> color_eyre::Result<()> {
	// Connect to the database
	let url = std::env::var("DATABASE_URL")?;
	// Connecting using the crate abstraction ensures that migrations always run.
	let pool = Pool::from_url(&url).await?;
	// Start the webserver
	Rocket::build()
		.manage(pool.clone())
		.medieval_sql_api()
		.launch()
		.await?;

	Ok(())
}
