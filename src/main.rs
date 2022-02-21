use std::{io::{self, BufRead, Write}, str::Chars, iter::Peekable};

#[derive(Debug)]
enum Token {
    LPAREN,
    RPAREN,
    DASH, STAR,
    PLUS, SLASH,
    NUMBER(f64),
    ERROR(String)
}

/* 
enum Operator {
    ADDITION,
    DIVISION,
    SUBTRACTION,
    MULTIPLICATION,
}

enum Expression {
    Binary(Operator, Box<Expression>, Box<Expression>),
    Number(f64)
}

fn lex(program: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::new();

    let mut skip_until: u8 = 0;
    let mut error_token = String::new();

    'char_loop: for (column, character) in program.chars().enumerate() {
        match character {
            ' ' | '\n' => continue,
            '(' => tokens.push(Token::LPAREN),
            '.' => {
                match buffer.parse::<f64>() {
                    Ok(_) => {
                        if !buffer.contains('.') {
                            buffer.push(character);
                        } else {
                            error_token = buffer.clone();
                            let mut current_character = character;
                            let mut i = column + 1;
                            while current_character.is_digit(10) || current_character == '.' {
                                current_character = match program.chars().nth(i) {
                                    Some(lookahead) => {
                                        i += 1;
                                        error_token.push(current_character);
                                        lookahead
                                    },
                                    None => {
                                        break;
                                    }
                                }
                            }

                            tokens.push(Token::ERROR(format!("Unrecognized keyword '{}' found from columns {} through {}. Did you mean {}?", error_token, column - buffer.len() + 1, i - 1, buffer)));
                            buffer.clear();
                        }
                    },
                    Err(_) => {
                        buffer.push(character);
                        tokens.push(Token::ERROR(format!("Unrecognized keyword '{}' found from columns {} through {}", buffer, column - buffer.len() + 1, column)));
                        buffer.clear();
                    }
                }
            },
            // TODO: use lookahead for buffer value
            ')' => tokens.push(Token::RPAREN),
            character if character.is_ascii_alphanumeric() => {
                match program.chars().nth(column + 1) {
                    Some(lookahead) => {
                        if lookahead.is_digit(10) || lookahead == '.' {
                            buffer.push(character);
                        } else {
                            match buffer.parse() {
                                Ok(float) => {
                                    tokens.push(Token::NUMBER(float));
                                    buffer.clear();
                                },
                                Err(_) => {
                                    tokens.push(Token::ERROR(format!("Unrecognized keyword '{}' found from columns {} through {}", buffer, column - buffer.len() + 1, column)));
                                    buffer.clear();
                                }
                            }
                        }
                    },
                    None => {
                        match buffer.parse() {
                            Ok(float) => {
                                tokens.push(Token::NUMBER(float));
                                buffer.clear();
                            },
                            Err(_) => {
                                tokens.push(Token::ERROR(format!("Unrecognized keyword '{}' found from columns {} through {}", buffer, column - buffer.len() + 1, column)));
                                buffer.clear();
                            }
                        }
                    }
                }
                buffer.push(character);
            },
            '*' => tokens.push(Token::STAR),
            '/' => tokens.push(Token::SLASH),
            '+' => tokens.push(Token::PLUS),
            '-' => {
                match program.chars().nth(column + 1) {
                    Some(lookahead) => {
                        if lookahead.is_digit(10) {
                            buffer.push(character);
                        } else {
                            tokens.push(Token::DASH);
                        }
                    },
                    None => {
                        tokens.push(Token::DASH);
                    }
                }
            },
            _ => {
                tokens.push(Token::ERROR(format!("Unrecognized token '{}' found at column {}", character, column + 1)));
            }
        }
    }

    tokens
}

struct BinaryTree {
    value: String,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    fn new(value: String) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None
        }
    }

    fn left(mut self, node: BinaryTree) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    fn right(mut self, node: BinaryTree) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}
*/

fn consume_number(c: char, iterator: &mut Peekable<Chars>) -> u64 {
    let mut number: u64 = c.to_string().parse().expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iterator.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        iterator.next();
    }
    number
}
// TODO: Fix implementation and make safer
fn lex(program: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iterator = program.chars().peekable();
    let mut column: u8 = 0;
    while let Some(&c) = iterator.peek() {
        column += 1;
        match c {
            '0'..='9' => {
                iterator.next();
                let before = consume_number(c, &mut iterator);
                println!("peek {:?}", iterator.peek());
                if Some(&'.') == iterator.peek() {
                    iterator.next();
                    if let Some(&d) = iterator.peek() {
                        if d.is_digit(10) {
                            iterator.next();
                            let after = consume_number(d, &mut iterator) as f64;
                            let number = before as f64 + after / 10_f64.powi(after.log10().ceil() as i32);
                            tokens.push(Token::NUMBER(number));
                        }
                    }
                } else {
                    tokens.push(Token::NUMBER(before as f64));
                }
            },
            '+' => {
                tokens.push(Token::PLUS);
                iterator.next();
            },
            '-' => {
                tokens.push(Token::DASH);
                iterator.next();
            },
            '*' => {
                tokens.push(Token::STAR);
                iterator.next();
            },
            '/' => {
                tokens.push(Token::SLASH);
                iterator.next();
            },
            '(' => {
                tokens.push(Token::LPAREN);
                iterator.next();
            },
            ')' => {
                tokens.push(Token::RPAREN);
                iterator.next();
            },
            ' ' | '\n' => {
                iterator.next();
            }, 
            _ => {
                tokens.push(Token::ERROR(format!("Error: Unrecognized token '{}' at column {}.", c, column)));
                iterator.next();
            }
        }

    }

    tokens
}


fn generate_ast(tokens: Vec<Token>) -> Vec<String> {
    let mut ast = Vec::new();
    let mut level: u8 = 0;
    for token in tokens {
        match token {
            Token::LPAREN => {
                ast.push(format!("{}(", "    ".repeat(level.into())));
                level += 1;
            },
            Token::RPAREN => {
                level -= 1;
                ast.push(format!("{})", "    ".repeat(level.into())));
            },
            Token::NUMBER(n) => {
                ast.push(format!("{}{}", "    ".repeat(level.into()), n));
            },
            Token::DASH => {
                ast.push(format!("{}-", "    ".repeat(level.into())));
            },
            Token::STAR => {
                ast.push(format!("{}*", "    ".repeat(level.into())));
            },
            Token::PLUS => {
                ast.push(format!("{}+", "    ".repeat(level.into())));
            },
            Token::SLASH => {
                ast.push(format!("{}/", "    ".repeat(level.into())));
            },
            Token::ERROR(e) => {
                ast.push(format!("{}Error: {}", "    ".repeat(level.into()), e));
            }
        }
    }
    ast
}

fn main() {
    let stdin = io::stdin();
    loop {
        print!("fun> ");
        io::stdout().flush().expect("Error flushing stdout");
        let mut input = String::new();
        stdin.lock().read_line(&mut input).expect("Error reading from stdin");
        if input == ":quit\n".to_string() {
            println!("Quitting...");
            break;
        } else {
            let tokens = lex(input);
            println!("{:?}", tokens);
            let ast = generate_ast(tokens);
            for string in ast { println!("{}", string) }
        }
    }
}
