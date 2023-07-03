extern crate core;

pub mod api;
pub mod config;
pub mod log;
pub mod model;
pub mod ovhapi;
pub mod utils;

pub const BUG: &str = "Unexpected error";
pub const QUEUE_SIZE: usize = 128;
pub const REQUEST_ID: &str = "X-Request-Id";
