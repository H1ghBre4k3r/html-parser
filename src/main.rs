fn main() {
    let lex = html_parser::Token::lex("<foo test=\"bar baz foo-bar\">");

    println!("{:#?}", lex.unwrap());
}
