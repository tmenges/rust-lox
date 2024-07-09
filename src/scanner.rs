
struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
}

fn init_scanner(source: String) -> Box<Scanner> {
    Box::new(Scanner::new(source))
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn is_at_end(&self) -> bool { self.current == self.source.len() }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn peek(&self) -> char {
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }

        self.source[self.current + 1]
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }

        if self.source[self.current] != expected {
            return false
        }

        self.current += 1;

        return true
    }

    fn skip_whitespace(&mut self) -> () {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                '/' =>
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    },
                _ => return,
            }
        }
    }
}

fn is_alpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') ||
           (c >= 'A' && c <= 'Z') ||
            c == '_'
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_ws() {
        let source = "skip<          >";
        let mut scanner = Scanner::new(source.to_string());
        while is_alpha(scanner.peek()) {
            scanner.advance();
        }
        assert_eq!(scanner.advance(), '<');
        scanner.skip_whitespace();
        assert_eq!(scanner.source[scanner.current], '>');
    }
}
