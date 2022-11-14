use crate::crawler::Crawler;
use crate::extractor::Extractor;
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;

pub struct Books {}

#[async_trait]
impl Extractor for Books {
    async fn parse(&self, _client: &Client, crawler: Crawler) -> Vec<Crawler> {
        let uri: String = if let Some(matches) = Regex::new(r#"^(?P<host>.*)/browse"#)
            .expect("Unable to parse regular expression for host")
            .captures(&crawler.get_uri())
        {
            matches["host"].to_owned()
        } else {
            crawler.get_uri()
        };

        let mut crawlers = Vec::new();

        if let Some(content) = &crawler.content {
            for cap in Regex::new(r#"<li\s*class="pgdbetext">(?P<bookLink>.*)</li>"#)
                .expect("Unable to parse regular expression for content")
                .captures_iter(content)
            {
                crawlers.push(Crawler::new().set_uri(&uri).set_content(&cap["bookLink"]));
            }
        }

        crawlers
    }
}

impl Books {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::Books;

    #[test]
    fn it_create_extractor() {
        Books::new();
    }
}
