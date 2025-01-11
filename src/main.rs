use html_parser::{Combinator, ParseStream};

fn main() {
    let lex = html_parser::Token::lex(
        r#"
    </foo < test="$foo.test()" baz="true" foobar>"#,
    )
    .unwrap();

    let mut parse_stream = ParseStream::new(lex);

    let combs = Combinator::LANGLE
        >> !Combinator::SLASH
        >> Combinator::IDENTIFIER
        >> Combinator::ATTRIBUTE
        >> Combinator::ATTRIBUTE
        >> Combinator::ATTRIBUTE
        >> Combinator::RANGLE;

    let res = combs.try_parse(&mut parse_stream);

    match res {
        Ok(res) => println!("{res:#?}"),
        Err(e) => eprintln!("{e}"),
    }
}
