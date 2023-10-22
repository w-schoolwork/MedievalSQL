use rocket::{
	async_trait,
	form::Form,
	get,
	http::{Cookie, CookieJar, Status},
	post,
	request::{FromRequest, Outcome},
	response::Redirect,
	routes, FromForm, Request, Route, State,
};
use rocket_dyn_templates::{context, Template};
use totp_rs::{Rfc6238, Secret, TOTP};
use uuid::Uuid;

use crate::db::Pool;

#[must_use]
pub fn routes() -> Vec<Route> {
	routes![
		challenge,
		make_account,
		try_login,
		try_make_account,
		check_logged_in,
		logout
	]
}

#[get("/check")]
#[must_use]
pub async fn check_logged_in(logged_in: LoggedInAs, pool: &State<Pool>) -> String {
	let email = pool.email_of(logged_in.0).await.ok().flatten();
	if let Some(email) = email {
		format!("OK, logged in as {email}.")
	} else {
		"Somehow logged in as a nonexistent user?".to_string()
	}
}

#[get("/logout")]
#[must_use]
pub async fn logout(pool: &State<Pool>, cookies: &CookieJar<'_>) -> &'static str {
	if let Some(cookie) = cookies.get("Authorization") {
		let secret = cookie.value().to_string();
		cookies.remove(cookie.clone());
		if let Err(e) = pool.clobber_session(&secret).await {
			eprintln!("{e}");
			return "Something happened?";
		}
		return "OK";
	};
	"You don't have a login cookie."
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
			Ok(Redirect::to("/"))
		}
		LoginStatus::BadTOTP => Err(Status::BadRequest),
		LoginStatus::BadUser => Err(Status::Unauthorized),
		LoginStatus::Whoopsie => Err(Status::InternalServerError),
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
	pub user_id: Uuid,
	pub email: String,
	pub totp_secret: TOTP,
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
	let user_id = Uuid::parse_str(&form.uuid).map_err(|_| Status::BadRequest)?;
	let totp_secret = TOTP::from_url(&form.totp).map_err(|_| Status::BadRequest)?;

	let check = totp_secret.check_current(&form.totp_confirm).unwrap();

	if !check {
		return Ok(Redirect::to("."));
	}

	let mk_acct = MakeAccount {
		user_id,
		totp_secret,
		email: form.email.clone(),
	};

	let secret = pool.register_user(&mk_acct).await.unwrap();

	cookies.add(Cookie::new("Authorization", secret));
	Ok(Redirect::to("/"))
}

pub struct LoggedInAs(pub Uuid);

#[async_trait]
impl<'a> FromRequest<'a> for LoggedInAs {
	type Error = &'static str;

	async fn from_request<'b: 'a>(request: &'a Request<'b>) -> Outcome<Self, Self::Error> {
		let secret = request.cookies().get("Authorization");
		if let Some(secret) = secret {
			let pool: &State<Pool> = match request.guard().await {
				Outcome::Success(p) => p,
				_ => {
					return Outcome::Failure((
						Status::InternalServerError,
						"No database connection?",
					))
				}
			};
			if let Some(id) = pool.check_session(secret.value()).await.ok().flatten() {
				return Outcome::Success(LoggedInAs(id));
			}
			return Outcome::Failure((
				Status::Unauthorized,
				"Secret doesn't correspond to a session",
			));
		}
		Outcome::Failure((Status::Unauthorized, "No secret available"))
	}
}
