// Strava API module

pub(crate) mod oauth;
pub use self::oauth::auth;
pub mod athlete;
pub mod activities;
pub mod gear;
pub mod helpers;
pub use self::helpers::handle_api_error;

// Returns the URL for the Strava API
pub fn strava_v3(path: String) -> String {
    format!("https://www.strava.com/api/v3/{}", path)
}
