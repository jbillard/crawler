#![warn(clippy::all, clippy::pedantic)]

mod crawler;
mod extractor;
mod robots;

use crate::robots::gutenberg::robot::Gutenberg;
use crate::robots::robot::Robot;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Gutenberg::new().run(&Client::new()).await;
    Ok(())
}
