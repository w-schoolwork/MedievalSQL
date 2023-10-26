use rocket::{get, routes, Route, State};
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

use crate::{db::Pool, www::login::LoggedInAs};

#[must_use]
pub fn routes() -> Vec<Route> {
	routes![index, practice]
}

#[get("/")]
async fn index(login: Option<LoggedInAs>, pool: &State<Pool>) -> Template {
	#[derive(Serialize)]
	struct User {
		email: String,
	}
	let user = if let Some(LoggedInAs(id)) = login {
		sqlx::query_as!(User, "SELECT email FROM users WHERE user_id = $1", id)
			.fetch_one(&***pool)
			.await
			.ok()
	} else {
		None
	};
	Template::render("index", context! { user })
}

#[get("/practice")]
fn practice() -> Template {
	Template::render("game", context! {})
}
