use morselang::lexer::Lexer;

fn main() {
    let src = r#"
    ... y == 7;
    .. z < 100;
    if abc123 > 5 loop
    .-.. i .-. j
    ..-. add(5,6);
    "hello world 123"
    + - * /
    .---- ..--- ...-- ....- ..... -.... --... ---.. ----. -----
    -....--...
    --
    set x = 42
    ..-.--.
    "#;
    let mut l = Lexer::new(src);
    while let Some(tok) = l.next_token() {
        println!("{tok:?}");
    }
}
