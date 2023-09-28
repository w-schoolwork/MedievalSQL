use rocket::{
	form::Form,
	get,
	http::{Cookie, CookieJar, Status},
	post,
	response::Redirect,
	routes, FromForm, Route, State,
};
use rocket_dyn_templates::{context, Template};
use totp_rs::{Rfc6238, Secret, TOTP};
use uuid::Uuid;

use crate::db::Pool;

#[must_use]
pub fn routes() -> Vec<Route> {
	routes![challenge, make_account, try_login, try_make_account]
}

#[get("/")]
#[must_use]
pub fn challenge() -> Template {
	Template::render(
		"login",
		context! {
			business_name: std::env::var("BUSINESS_NAME").unwrap_or_else(|_| String::from("Businessy McBusinessface"))
		},
	)
}

#[derive(FromForm)]
pub struct LoginAttempt {
	pub email: String,
	pub totp: String,
}

pub enum LoginStatus {
	/// The login is successful, and this is the session secret.
	Success(String),
	/// The login failed because the TOTP challenge failed.
	BadTOTP,
	/// The login failed because this user doesn't exist.
	BadUser,
	/// The login failed because something is wrong
	Whoopsie,
}

/// # Errors
/// * HTTP 400 when the given TOTP code is incorrect.
/// * HTTP 401 when the user does not exist.
/// * HTTP 500 when the database call fails.
#[post("/", data = "<login>")]
pub async fn try_login(
	login: Form<LoginAttempt>,
	cookies: &CookieJar<'_>,
	pool: &State<Pool>,
) -> Result<Redirect, Status> {
	let login_status: LoginStatus = pool.login(&login).await.expect("Database call");

	match login_status {
		LoginStatus::Success(secret) => {
			cookies.add(Cookie::new("Authorization", secret));
			Ok(Redirect::temporary("/"))
		}
		LoginStatus::BadTOTP => Err(Status::BadRequest),
		LoginStatus::BadUser => Err(Status::Unauthorized),
		LoginStatus::Whoopsie => Err(Status::InternalServerError)
	}
}

#[must_use]
#[get("/new")]
pub fn make_account() -> Template {
	let uuid = Uuid::new_v4();
	let secret = Secret::generate_secret();
	let totp = Rfc6238::new(
		6,
		secret.to_bytes().unwrap(),
		Some("vbmd".to_string()),
		uuid.to_string(),
	)
	.unwrap();
	let totp = TOTP::from_rfc6238(totp).unwrap();
	let qr = "data:image/png;base64,".to_string() + &totp.get_qr().unwrap();

	Template::render(
		"mk-acct",
		context! {
			totp_url: totp.get_url(),
			totp_qr: qr,
			uuid: uuid.to_string()
		},
	)
}

#[derive(FromForm)]
pub struct MakeAccountAttempt {
	email: String,
	uuid: String,
	totp: String,
	totp_confirm: String,
}

pub struct MakeAccount {
	pub uuid: Uuid,
	pub email: String,
	pub totp: TOTP,
}

/// # Errors
/// * HTTP 400 when the UUID or TOTP secret are malformed
/// * HTTP 500 if the database query fails or the totp check can't get the system time for some reason
#[post("/new", data = "<form>")]
pub async fn try_make_account(
	cookies: &CookieJar<'_>,
	pool: &State<Pool>,
	form: Form<MakeAccountAttempt>,
) -> Result<Redirect, Status> {
	let uuid = Uuid::parse_str(&form.uuid).map_err(|_| Status::BadRequest)?;
	let totp = TOTP::from_url(&form.totp).map_err(|_| Status::BadRequest)?;

	let check = totp.check_current(&form.totp_confirm).unwrap();

	if !check {
		return Ok(Redirect::temporary("."));
	}

	let mk_acct = MakeAccount {
		uuid,
		totp,
		email: form.email.clone(),
	};

	let secret = pool.register(&mk_acct).await.unwrap();

	cookies.add(Cookie::new("Authorization", secret));
	Ok(Redirect::temporary("/"))
}
