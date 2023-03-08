#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
pub extern crate sea_orm;

pub mod prelude;

pub mod db;
pub mod todos;
