use crate::crawler::Crawler;
use crate::extractor::Extractor;
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::path::Path;

const OUTPUT_DIR: &str = "./output/gutenberg";

pub struct Book {}

#[async_trait]
impl Extractor for Book {
    async fn parse(&self, client: &Client, crawler: Crawler) -> Vec<Crawler> {
        if let Some(matches) = Regex::new(r#"<a\s*href="(?P<bookLink>/ebooks.*)">"#)
            .expect("Unable to parse regular expression for book link")
            .captures(&crawler.get_content())
        {
            if let Ok(content) = Crawler::parse_uri(
                client,
                &format!("{}{}", &crawler.get_uri(), &matches["bookLink"]),
            )
            .await
            {
                match Regex::new(r#"<a\s*href="(?P<fileLink>.*?)"[^>]*>Plain\s+Text\s+UTF-8"#)
                    .expect("Unable to parse regular expression for file link")
                    .captures(&content)
                {
                    Some(matches) => self
                        .process(
                            client,
                            &format!("{}{}", &crawler.get_uri(), &matches["fileLink"]),
                        )
                        .await
                        .expect("Unable to process Gutenberg file extractor"),
                    None => println!("No UTF-8 book for {}", &matches["bookLink"]),
                }
            }
        }
        [].to_vec()
    }
}

impl Book {
    pub fn new() -> Self {
        Self {}
    }

    async fn process(&self, client: &Client, uri: &str) -> std::io::Result<()> {
        println!("{}", uri);
        DirBuilder::new()
            .recursive(true)
            .create(OUTPUT_DIR)
            .unwrap();
        if let Ok(content) = Crawler::parse_uri(client, uri).await {
            if let Some(file_name) = Path::new(&uri).file_name() {
                if let Some(f) = file_name.to_str() {
                    println!("{}/{}", OUTPUT_DIR, f);
                    File::create(format!("{}/{}", OUTPUT_DIR, f))?
                        .write_all(&content.into_bytes())?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Book;

    #[test]
    fn it_create_extractor() {
        Book::new();
    }
}
