use html_parser::{Combinator, ParseStream};

fn main() {
    let lex = html_parser::Token::lex("<foo test=\"yes\">").unwrap();

    let mut parse_stream = ParseStream::new(lex);

    let combs = Combinator::LANGLE
        >> Combinator::IDENTIFIER
        >> Combinator::IDENTIFIER
        >> Combinator::EQUALS
        >> Combinator::VALUE
        >> Combinator::RANGLE;

    let res = combs.try_parse(&mut parse_stream);

    println!("{res:#?}");
}
