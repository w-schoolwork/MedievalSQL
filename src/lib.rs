#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_panics_doc)]

use std::fmt::Display;

use rocket::{fs::FileServer, http::Status, Build, Rocket};
use rocket_dyn_templates::Template;

pub mod db;
pub mod www;

pub trait RocketExt {
	#[must_use]
	fn medieval_sql_api(self) -> Self;
}

impl RocketExt for Rocket<Build> {
	fn medieval_sql_api(self) -> Self {
		self.attach(Template::fairing())
			.mount("/static", FileServer::from("./static"))
			.mount("/", www::index::routes())
			.mount("/login", www::login::routes())
	}
}
