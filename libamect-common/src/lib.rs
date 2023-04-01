pub mod cache;
pub mod defines;
mod misc;
mod user;
mod utils;

pub mod windows {
    pub use super::misc::*;
    pub use super::user::*;
    pub use super::utils::is_admin;
}
