use reqwest::Client;
use std::string::String;

#[derive(Debug, Clone)]
pub struct Crawler {
    pub content: Option<String>,
    pub uri: Option<String>,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            content: None,
            uri: None,
        }
    }
    pub fn set_uri(mut self, uri: &str) -> Self {
        self.uri = Some(uri.to_owned());
        self
    }

    pub fn set_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_owned());
        self
    }

    pub fn get_content(&self) -> String {
        let mut result: String = String::new();
        if let Some(ref content) = self.content {
            result = content.to_string();
        }
        result
    }

    pub fn get_uri(&self) -> String {
        let mut result: String = String::new();
        if let Some(ref uri) = self.uri {
            result = uri.to_string();
        }
        result
    }

    pub async fn parse_uri(client: &Client, uri: &str) -> Result<String, reqwest::Error> {
        client.get(uri).send().await?.text().await
    }

    pub async fn parse(self, client: &Client, uri: String) -> Result<Self, reqwest::Error> {
        Ok(self
            .set_uri(&uri)
            .set_content(client.get(&uri).send().await?.text().await?.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::Crawler;

    #[test]
    fn it_create_crawler() {
        let crawler = Crawler::new();
        assert!(crawler.content.is_none());
        assert!(crawler.uri.is_none());
    }

    #[test]
    fn it_set_uri() {
        let crawler = Crawler::new();
        let new_crawler = crawler.set_uri("test");
        assert_eq!(new_crawler.uri, Some("test".to_string()));
    }

    #[test]
    fn it_set_content() {
        let crawler = Crawler::new();
        let new_crawler = crawler.set_content("test");
        assert_eq!(new_crawler.content, Some("test".to_string()));
    }

    #[test]
    fn it_get_content() {
        let crawler = Crawler::new();
        assert_eq!(crawler.get_content(), "".to_string());
    }

    #[test]
    fn it_get_uri() {
        let crawler = Crawler::new();
        assert_eq!(crawler.get_uri(), "".to_string());
    }
}
