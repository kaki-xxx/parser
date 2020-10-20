use super::parser;
use super::parser::tokenizer;

fn parse(input: &str) -> Option<i32> {
    let mut p = parser::Parser::new(input.to_string());
    p.parse()
}

#[test]
fn num() {
    assert_eq!(parse("1"), Some(1));
    assert_eq!(parse("3"), Some(3));
    assert_eq!(parse("0"), Some(0));
}

#[test]
fn add() {
    assert_eq!(parse("1 + 1"), Some(2));
    assert_eq!(parse("1 + 2 + 3 + 4"), Some(10));
    assert_eq!(parse("1+2+3+4"), Some(10));
}

#[test]
fn sub() {
    assert_eq!(parse("2 - 1"), Some(1));
    assert_eq!(parse("1 - 3"), Some(-2));
    assert_eq!(parse("10 - 3 - 2 - 1"), Some(4));
    assert_eq!(parse("10-3-2-1"), Some(4));
}

#[test]
fn mul() {
    assert_eq!(parse("1 * 2"), Some(2));
    assert_eq!(parse("2 * 3 * 4"), Some(24));
    assert_eq!(parse("2*3*4"), Some(24));
}

#[test]
fn div() {
    assert_eq!(parse("4 / 2"), Some(2));
    assert_eq!(parse("3 / 2"), Some(1));
    assert_eq!(parse("8 / 2 / 2"), Some(2));
    assert_eq!(parse("8/2/2"), Some(2));
}

#[test]
fn add_and_mul() {
    assert_eq!(parse("1 * 2 + 3"), Some(5));
    assert_eq!(parse("1 + 2 * 3"), Some(7));
    assert_eq!(parse("1+2*3"), Some(7));
}

#[test]
fn sub_and_mul() {
    assert_eq!(parse("1 * 2 - 3"), Some(-1));
    assert_eq!(parse("1 - 2 * 3"), Some(-5));
    assert_eq!(parse("1-2*3"), Some(-5));
}

#[test]
fn with_parentheses() {
    assert_eq!(parse("( 1 + 2 ) * 3"), Some(9));
    assert_eq!(parse("4 * ( 2 + 3 )"), Some(20));
    assert_eq!(parse("(1*(2+3*4))/2"), Some(7));
}

#[test]
fn error() {
    assert_eq!(parse("1 +"), None);
    assert_eq!(parse("1 1"), None);
}

fn tokenize(input: &str) -> tokenizer::Tokenizer {
    use tokenizer::*;
    Tokenizer::new(input.to_string())
}

#[test]
fn tokenize_add() {
    use tokenizer::*;
    let mut t = tokenize("10+2+3");
    assert_eq!(t.comsume(TokenKind::Num), Some(Token::new_num(10)));
    assert_eq!(t.comsume(TokenKind::Plus), Some(Token::new(TokenKind::Plus)));
    assert_eq!(t.comsume(TokenKind::Num), Some(Token::new_num(2)));
    assert_eq!(t.comsume(TokenKind::Plus), Some(Token::new(TokenKind::Plus)));
    assert_eq!(t.comsume(TokenKind::Num), Some(Token::new_num(3)));
}

#[test]
fn tokenize_zero() {
    use tokenizer::*;
    let mut t = tokenize("1+0+2");
    assert_eq!(t.comsume(TokenKind::Num), Some(Token::new_num(1)));
    assert_eq!(t.comsume(TokenKind::Plus), Some(Token::new(TokenKind::Plus)));
    assert_eq!(t.comsume(TokenKind::Num), Some(Token::new_num(0)));
    assert_eq!(t.comsume(TokenKind::Plus), Some(Token::new(TokenKind::Plus)));
    assert_eq!(t.comsume(TokenKind::Num), Some(Token::new_num(2)));
}
