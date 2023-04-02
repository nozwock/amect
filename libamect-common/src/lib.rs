pub mod cache;
pub mod defines;
mod misc;
mod user;
pub mod utils;
mod winutils;

pub mod windows {
    pub use super::misc::*;
    pub use super::user::*;
    pub use super::winutils::is_admin;
}
