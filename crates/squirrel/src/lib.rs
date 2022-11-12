//! Bill lines
#![allow(dead_code)]

mod application;

mod cli;
mod db;
mod error;
mod utils;

mod models;
pub mod ui_actions;

pub use error::Error;
pub use models::squirrel::Squirrel;

pub use cli::Cli;
