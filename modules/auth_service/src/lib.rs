mod database; // maps module to filename database
mod auth_utils; // maps module to folder auth_utils

// re-export Credentials
pub use auth_utils::models::Credentials;
use database::Status;

// public API
pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_database() {
        auth_utils::login(creds)
    }
}



