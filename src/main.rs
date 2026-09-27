use morselang::{lexer::Lexer, parser::Parser, tokens::Token};

fn main() {
    // let src = r#"
    // ... y == 7;
    // .. z < 100;
    // if abc123 > 5 loop
    // .-.. i .-. j
    // ..-. add(5,6);
    // "hello world 123"
    // + - * /
    // .---- ..--- ...-- ....- ..... -.... --... ---.. ----. -----
    // -....--...
    // --
    // set x = 42
    // ..-.--.
    // "#;
    let src = r#"
    F foo() {
    S a = 3 + 5 * 10;
    }
    "#;
    let mut lexer = Lexer::new(src).peekable();

    // while let Some(tok) = lexer.next() {
    //     if matches!(tok, Token::Eof) {
    //         break;
    //     }
    //     dbg!(tok);
    // }
    let exp = Parser::new(lexer).parse();
    dbg!(&exp);
    dbg!(exp.to_string());
}
