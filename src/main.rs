use morselang::{lexer::Lexer, parser::Parser};

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
    let src = "a = 3 + 5 * 10";
    let lexer = Lexer::new(src).peekable();

    // while let Some(tok) = lexer.next() {
    //     // if let Some(tok) = lexer.peek() {
    //     dbg!(tok);
    //     // }
    // }
    let exp = Parser::new(lexer).parse();
    dbg!(&exp);
    dbg!(exp.to_string());
}
