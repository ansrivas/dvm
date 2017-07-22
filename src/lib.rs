extern crate clap;
extern crate chrono;

pub use manager::*;
pub use saver::ImageSaver;

mod manager;
mod utils;
mod saver;
