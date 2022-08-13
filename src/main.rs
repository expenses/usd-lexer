fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let string = std::fs::read_to_string(&filename).unwrap();
    let mut lex = Token::lexer(&string);

    let tokens: Vec<_> = lex.collect();
    //dbg!(&tokens);

    //let mut defines = Vec::new();

    let mut lex = Token::lexer(&string);

    while let Some(token) = lex.next() {
        match token {
            Token::Identifier(ident) => {
                if lex.next() == Some(Token::OpenBracket) {
                    assert_eq!(lex.next(), Some(Token::CloseBracket));
                    let name = match lex.next() {
                        Some(Token::Identifier(name)) => name,
                        _ => panic!(),
                    };
                    assert_eq!(lex.next(), Some(Token::Equals));

                    if ident == "int" {
                        let values = parse_integer_list(&mut lex);

                        let max = values.iter().max();

                        dbg!(&name, &values.len(), &max);
                    } else if ident == "point3f" {
                        let values = parse_vec3_list(&mut lex);

                        dbg!(&name, values.len());
                    } else if ident == "texCoord2f" {
                        let values = parse_vec2_list(&mut lex);

                        dbg!(&name, values.len());
                    }
                }
            }
            _ => {}
        }
    }

    //dbg!(&defines);
}

fn parse_integer_list<'a>(lexer: &mut logos::Lexer<'a, Token<'a>>) -> Vec<u32> {
    assert_eq!(lexer.next(), Some(Token::OpenBracket));

    let mut values = Vec::new();

    loop {
        match lexer.next() {
            Some(Token::Integer(int)) => values.push(int as u32),
            Some(Token::CloseBracket) => break,
            Some(Token::Comma) => {}
            _ => panic!(),
        }
    }

    values
}

fn parse_vec3_list<'a>(lexer: &mut logos::Lexer<'a, Token<'a>>) -> Vec<[f32; 3]> {
    assert_eq!(lexer.next(), Some(Token::OpenBracket));

    let mut values = Vec::new();

    loop {
        match lexer.next() {
            Some(Token::OpenParen) => {}
            Some(Token::CloseBracket) => break,
            _ => panic!(),
        }

        let a = parse_float(lexer);
        assert_eq!(lexer.next(), Some(Token::Comma));
        let b = parse_float(lexer);
        assert_eq!(lexer.next(), Some(Token::Comma));
        let c = parse_float(lexer);
        assert_eq!(lexer.next(), Some(Token::CloseParen));

        values.push([a, b, c]);
        match lexer.next() {
            Some(Token::CloseBracket) => break,
            Some(Token::Comma) => {}
            _ => panic!(),
        }
    }

    values
}


fn parse_vec2_list<'a>(lexer: &mut logos::Lexer<'a, Token<'a>>) -> Vec<[f32; 2]> {
    assert_eq!(lexer.next(), Some(Token::OpenBracket));

    let mut values = Vec::new();

    loop {
        match lexer.next() {
            Some(Token::OpenParen) => {}
            Some(Token::CloseBracket) => break,
            _ => panic!(),
        }

        let a = parse_float(lexer);
        assert_eq!(lexer.next(), Some(Token::Comma));
        let b = parse_float(lexer);
        assert_eq!(lexer.next(), Some(Token::CloseParen));

        values.push([a, b]);
        match lexer.next() {
            Some(Token::CloseBracket) => break,
            Some(Token::Comma) => {}
            _ => panic!(),
        }
    }

    values
}


fn parse_float<'a>(lexer: &mut logos::Lexer<'a, Token<'a>>) -> f32 {
    match lexer.next() {
        Some(Token::Float(float)) => float,
        Some(Token::Integer(int)) => int as f32,
        _ => panic!(),
    }
}

fn parse_define<'a>(lexer: &mut logos::Lexer<'a, Token<'a>>) -> Define {
    let mut next = lexer.next();

    let ty = match next {
        Some(Token::Identifier("Material")) => {
            next = lexer.next();
            Some(DefineType::Material)
        }
        Some(Token::Identifier("Shader")) => {
            next = lexer.next();
            Some(DefineType::Shader)
        }
        Some(Token::Identifier("Scope")) => {
            next = lexer.next();
            Some(DefineType::Scope)
        }
        Some(Token::Identifier("Xform")) => {
            next = lexer.next();
            Some(DefineType::Xform)
        }
        Some(Token::Identifier("Sphere")) => {
            next = lexer.next();
            Some(DefineType::Sphere)
        }
        Some(Token::Identifier("Mesh")) => {
            next = lexer.next();
            Some(DefineType::Mesh)
        }
        Some(Token::Name(_)) => None,
        other => panic!("{:?}", other),
    };

    let name = match next {
        Some(Token::Name(name)) => name.to_string(),
        other => panic!("{:?}", other),
    };

    let mut children = Vec::new();

    let mut kind = None;
    let mut references = None;

    match lexer.next() {
        Some(Token::OpenBrace) => {}
        Some(Token::OpenParen) => {
            while let Some(token) = lexer.next() {
                dbg!(&token);
                match token {
                    Token::Kind => {
                        assert_eq!(lexer.next(), Some(Token::Equals));
                        kind = match lexer.next() {
                            Some(Token::Name(name)) => Some(name.to_string()),
                            other => panic!("{:?}", other),
                        };
                    }
                    Token::References => {
                        assert_eq!(lexer.next(), Some(Token::Equals));
                        references = match lexer.next() {
                            Some(Token::FilePath(path)) => Some(path.to_string()),
                            other => panic!("{:?}", other),
                        };
                    }
                    Token::CloseParen => break,
                    _ => {}
                }
            }

            assert_eq!(lexer.next(), Some(Token::OpenBrace));
        }
        other => panic!("{:?}", other),
    };

    while let Some(token) = lexer.next() {
        match token {
            Token::Def => {
                children.push(parse_define(lexer));
            }
            Token::CloseBrace => {
                return Define {
                    ty,
                    name,
                    children,
                    kind,
                    references,
                }
            }
            _ => {}
        }
    }

    panic!()
}

#[derive(Debug)]
struct Define {
    ty: Option<DefineType>,
    name: String,
    kind: Option<String>,
    references: Option<String>,
    children: Vec<Define>,
}

#[derive(Debug)]
enum DefineType {
    Xform,
    Sphere,
    Mesh,
    Scope,
    Material,
    Shader,
}

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token<'a> {
    // Tokens can be literal strings, of any length.
    #[regex("#[^\n]+", |lexer| lexer.slice().trim_left_matches('#').trim())]
    Comment(&'a str),

    #[regex("\"[^\"]+\"", |lexer| lexer.slice().trim_matches('\"'))]
    Name(&'a str),

    #[token("def")]
    Def,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("add")]
    Add,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token("=")]
    Equals,

    #[regex("-?[0-9]*\\.[0-9]+(e[-+][0-9]+)?", |lex| lex.slice().parse())]
    Float(f32),

    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("references")]
    References,

    #[regex("<[^>]+>", |lex| lex.slice().trim_left_matches('<').trim_right_matches('>'))]
    ScenePath(&'a str),

    #[regex("@[^@]+@", |lex| lex.slice().trim_matches('@'))]
    FilePath(&'a str),

    #[regex(r"[a-zA-Z]+[a-zA-Z0-9\.:]*")]
    Identifier(&'a str),

    #[token("true")]
    True,

    #[token("false")]
    False,

    // Logos requires one token varia#[token("visibility")]
    Visibility,

    #[token("variantSets")]
    VariantSets,

    #[token("variantSet")]
    VariantSet,

    #[token("delete")]
    Delete,

    #[token("rel")]
    Rel,

    #[token("class")]
    Class,

    #[token("over")]
    Over,

    #[token("instanceable")]
    Instanceable,

    #[token("inherits")]
    Inherits,

    #[token("None")]
    None,

    #[token("Prepend")]
    Prepend,

    #[token("kind")]
    Kind,

    #[token("specializes")]
    Specializes,

    //nt to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}
