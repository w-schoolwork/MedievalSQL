use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};

#[must_use]
pub fn routes() -> Vec<Route> {
	routes![index, practice]
}

#[get("/")]
fn index() -> Template {
	Template::render("index", context! {})
}

#[get("/practice")]
fn practice() -> Template {
	Template::render("game", context! {})
}
