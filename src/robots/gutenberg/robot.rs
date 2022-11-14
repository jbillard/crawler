use crate::robots::gutenberg::extractors::{book::Book, books::Books};
use crate::robots::pass::Pass;
use crate::robots::robot::Robot;

const URI: &str = "http://www.gutenberg.org/browse/languages/fr";

pub struct Gutenberg {}

impl Robot for Gutenberg {
    fn new() -> Self {
        Gutenberg {}
    }

    fn get_pass(&self) -> Pass {
        Pass::new("books")
            .extract_with(Books::new())
            .continue_with(self.pass_book())
    }

    fn get_uri(&self) -> String {
        URI.to_string()
    }
}

impl Gutenberg {
    fn pass_book(&self) -> Pass {
        Pass::new("book").extract_with(Book::new())
        //.continue_with(self.pass_files())
    }

    /*fn pass_files(&self) -> Pass {
         Pass::new("files").extract_with(FilesExtractor::new())
    }*/
}

#[cfg(test)]
mod tests {
    use super::Gutenberg;
    use crate::robots::{gutenberg::robot::URI, robot::Robot};

    #[test]
    fn it_create_robot() {
        Gutenberg::new();
    }

    #[test]
    fn it_get_pass() {
        let robot = Gutenberg::new();
        let pass = robot.get_pass();
        assert_eq!(pass.name, "books");
    }

    #[test]
    fn it_get_uri() {
        let robot = Gutenberg::new();
        assert_eq!(robot.get_uri(), URI);
    }
}
