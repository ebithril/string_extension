pub trait StringExtension {
    fn find_all(&self, in_char: char) -> Vec<usize>;
}

impl StringExtension for String {
    fn find_all(&self, in_char: char) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for (i, c) in self.chars().enumerate() {
            if c == in_char {
                result.push(i);
            }
        }

        result
    }
}

impl StringExtension for str {
    fn find_all(&self, in_char: char) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for (i, c) in self.chars().enumerate() {
            if c == in_char {
                result.push(i);
            }
        }

        result
    }
}
