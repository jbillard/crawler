use crate::crawler::Crawler;
use crate::robots::pass::Pass;
use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use reqwest::Client;

#[async_trait]
pub trait Robot: Sized {
    async fn process(&self, client: &Client, pass: &Pass, crawler: Crawler) {
        if let Some(ref extractor) = pass.extractor {
            let sub_crawlers = extractor.parse(client, crawler.clone());
            stream::iter(sub_crawlers.await.into_iter().map(|sub_crawler| async {
                if let Some(ref next_pass) = pass.next_pass {
                    self.process(client, next_pass, sub_crawler).await;
                }
            }))
            .buffer_unordered(100)
            .collect::<Vec<()>>()
            .await;
        }

        if let Some(ref next_pass) = pass.next_pass {
            self.process(client, next_pass, crawler).await;
        }
    }

    async fn run(&self, client: &Client) {
        if let Ok(crawler) = Crawler::new().parse(client, self.get_uri()).await {
            self.process(client, &self.get_pass(), crawler).await;
        }
    }

    fn new() -> Self;
    fn get_pass(&self) -> Pass;
    fn get_uri(&self) -> String;
}
