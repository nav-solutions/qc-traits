pub enum Token {
    Equal,
    Colon,
    Semicolon,
    Item(String),
}
pub struct Parser;

impl Parser {
    fn tokenize(s: &str) -> Vec<Token> {
        let mut buffer = String::with_capacity(8);
        let mut tokens = Vec::with_capacity(4);

        let trimmed = s.trim();

        for c in trimmed.chars() {
            match c {
                ':' => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Item(buffer.trim().to_string()));
                    }

                    tokens.push(Token::Colon);
                }
                ';' => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Item(buffer.trim().to_string()));
                    }

                    tokens.push(Token::Semicolon);
                }
                '=' => {
                    if !buffer.is_empty() {
                        tokens.push(Token::Item(buffer.trim().to_string()));
                    }

                    tokens.push(Token::Equal);
                }
                c => buffer.push(c),
            }
        }

        tokens
    }
}

#[cfg(test)]
mod test {

    use super::{Parser, Token};

    #[test]
    fn single_item_parsing() {
        let content = "Gal";
        let mut tokens = Parser::tokenize(content);
    }
}
