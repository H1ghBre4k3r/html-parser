use html_parser::{Combinator, ParseStream};

fn main() {
    let lex = html_parser::Token::lex(r#"<foo test="bar" baz="true" foobar>"#).unwrap();

    let mut parse_stream = ParseStream::new(lex);

    let combs = Combinator::LANGLE
        >> Combinator::IDENTIFIER
        >> Combinator::ATTRIBUTE
        >> Combinator::ATTRIBUTE
        >> Combinator::ATTRIBUTE
        >> Combinator::RANGLE;

    let res = combs.try_parse(&mut parse_stream);

    println!("{res:#?}");
}
