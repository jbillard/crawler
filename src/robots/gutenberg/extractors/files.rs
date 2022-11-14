use crate::crawler::Crawler;
use crate::extractor::Extractor;
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::path::Path;

const OUTPUT_DIR: &str = "./output/gutenberg";

pub struct Files {}

#[async_trait]
impl Extractor for Files {
    async fn parse(&self, client: &Client, crawler: Crawler) -> Vec<Crawler> {
        if let Some(matches) = Regex::new(r#"<a\s*href="(?P<fileLink>[^"]*\.txt)">"#)
            .expect("Unable to parse regular expression for file link")
            .captures(&crawler.get_content())
        {
            self.process(
                client,
                &format!("{}/{}", &crawler.get_uri(), &matches["fileLink"]),
            )
            .await
            .expect("Unable to process Gutenberg files extractor");
        }
        [].to_vec()
    }
}

impl Files {
    /*pub fn new() -> Self {
        Self {}
    }*/

    async fn process(&self, client: &Client, uri: &str) -> std::io::Result<()> {
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
