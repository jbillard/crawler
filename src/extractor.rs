use crate::crawler::Crawler;
use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait Extractor {
    async fn parse(&self, client: &Client, crawler: Crawler) -> Vec<Crawler>;
}
