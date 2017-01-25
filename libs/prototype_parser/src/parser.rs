use ast::Ast;
use std::iter::*;
use token::Token;

#[derive(Debug)]
pub enum Error<'a> {
    Mismatch { expected: &'static str, found: Option<Token<'a>> }
}

fn literal<'a, T>(token_stream: &mut Peekable<T>) -> Result<Ast<'a>, Error<'a>>
    where T: Iterator<Item=Token<'a>>
{
    match token_stream.next() {
        Some(Token::Literal(literal)) => Ok(Ast::Literal(literal)),
        x => Err(Error::Mismatch { expected: "literal", found: x })
    }
}

fn interpolation<'a, T>(token_stream: &mut Peekable<T>) -> Result<Ast<'a>, Error<'a>>
    where T: Iterator<Item=Token<'a>>
{
    match token_stream.next() {
        Some(Token::Interpolation(name)) => Ok(Ast::Interpolation(name)),
        x => Err(Error::Mismatch { expected: "interpolation", found: x })
    }
}

fn section<'a, T>(token_stream: &mut Peekable<T>) -> Result<Ast<'a>, Error<'a>>
    where T: Iterator<Item=Token<'a>>
{
    let name = match token_stream.next() {
        Some(Token::SectionOpener(name)) => Ok(name),
        x => Err(Error::Mismatch { expected: "section opener", found: x })
    }?;

    let nested = sequence(token_stream)?;

    match token_stream.next() {
        Some(Token::SectionCloser(ref close_name)) if close_name == &name
            => Ok(()),
        x => Err(Error::Mismatch { expected: "section closer", found: x })
    }?;

    Ok(Ast::Section { name: name, nested: Box::new(nested) })
}

fn sequence<'a, T>(token_stream: &mut Peekable<T>) -> Result<Ast<'a>, Error<'a>>
    where T: Iterator<Item=Token<'a>>
{
    let mut seq: Vec<Ast> = vec![];

    loop {
        seq.push(
            match token_stream.peek() {
                Some(&Token::Literal(_)) => literal(token_stream),
                Some(&Token::Interpolation(_)) => interpolation(token_stream),
                Some(&Token::SectionOpener(_)) => section(token_stream),
                _ => break
            }?
        )
    }

    Ok(Ast::Sequence(seq))
}

fn parse_impl<'a, T>(mut token_stream: Peekable<T>) -> Result<Ast<'a>, Error<'a>>
    where T: Iterator<Item=Token<'a>>
{
    let seq = sequence(&mut token_stream)?;

    if let Some(x) = token_stream.next() {
        return Err(Error::Mismatch {
            expected: "EOF",
            found: Some(x)
        });
    }

    Ok(seq)
}

pub fn parse<'a, T>(token_stream: T) -> Result<Ast<'a>, Error<'a>>
    where T: IntoIterator<Item=Token<'a>>
{
    parse_impl(token_stream.into_iter().peekable())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Ast::Sequence(vec![
                Ast::Literal("text"),
            ]),
            parse(vec![
                Token::Literal("text")
            ]).unwrap()
        )
    }

    #[test]
    fn simple_section() {
        assert_eq!(
            Ast::Sequence(vec![
                Ast::Literal("text a"),
                Ast::Section {
                    name: "x",
                    nested: Box::new(Ast::Sequence(vec![
                        Ast::Literal("text b"),
                    ]))
                },
                Ast::Literal("text c"),
            ]),
            parse(vec![
                Token::Literal("text a"),
                Token::SectionOpener("x"),
                Token::Literal("text b"),
                Token::SectionCloser("x"),
                Token::Literal("text c"),
            ]).unwrap()
        )
    }

    #[test]
    fn section_closer_mismatch() {
        let res = parse(vec![
            Token::SectionOpener("x"),
            Token::SectionCloser("y"),
        ]);

        assert!(res.is_err())
    }
}
