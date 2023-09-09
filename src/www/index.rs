use rocket::{get, routes, Route};
use rocket_dyn_templates::{context, Template};

#[must_use]
pub fn routes() -> Vec<Route> {
	routes![index]
}

#[get("/")]
fn index() -> Template {
	Template::render("index", context! {})
}
