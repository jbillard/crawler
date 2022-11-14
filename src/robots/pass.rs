use crate::extractor::Extractor;

pub struct Pass {
    pub extractor: Option<Box<dyn Extractor + Send + Sync>>,
    pub name: String,
    pub next_pass: Option<Box<Self>>,
}

impl Pass {
    pub fn new(pass: &str) -> Self {
        Pass {
            extractor: None,
            name: pass.to_owned(),
            next_pass: None,
        }
    }

    pub fn continue_with(mut self, next_pass: Self) -> Self {
        self.next_pass = Some(Box::new(next_pass));
        self
    }

    pub fn extract_with<E>(mut self, extractor: E) -> Self
    where
        E: Extractor + 'static + Send + Sync,
    {
        self.extractor = Some(Box::new(extractor));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::robots::gutenberg::extractors::books::Books;

    use super::Pass;

    #[test]
    fn it_create_pass() {
        let pass = Pass::new("test");
        assert!(pass.extractor.is_none());
        assert_eq!(pass.name, "test".to_string());
        assert!(pass.next_pass.is_none());
    }

    #[test]
    fn it_continue_with() {
        let pass = Pass::new("test");
        let new_pass = pass.continue_with(Pass::new("test2"));
        assert!(new_pass.next_pass.is_some());
        assert_eq!(new_pass.next_pass.unwrap().name, "test2");
    }

    #[test]
    fn it_extract_with() {
        let pass = Pass::new("test");
        let new_pass = pass.extract_with(Books::new());
        assert!(new_pass.extractor.is_some());
    }
}
