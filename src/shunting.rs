use std::{error::Error, fmt::Display};

use logos::Logos;


#[derive(Debug)]
pub enum ShuntingError {
    MissingParenthesisError,
    RequiredVariableError,
    SolvingError,
    UnequalOperatorNumberError,
    Other
}

impl Display for ShuntingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shunting error")
    }
}

impl Error for ShuntingError {}

#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("^")]
    Exp,

    #[token("(")]
    Open,

    #[token(")")]
    Close,

    #[token("x")]
    Variable,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i32),

    #[error]
    Error
}

impl Token {
    fn priority(&self) -> Option<i32> {
        match self {
            Token::Add | Token::Sub => Some(1),
            Token::Mul | Token::Div => Some(2),
            Token::Exp => Some(3),
            _ => None
        }
    }
}

pub fn shunt(input: &String) -> Result<Vec<Token>, ShuntingError> {
    // tokenize the input, discarding errors
    let input = Token::lexer(input)
        .filter(|t| t != &Token::Error);

    // needed variables
    let mut opstack: Vec<Token> = Vec::new();
    let mut postfix: Vec<Token> = Vec::new();
    let mut par_count = (0, 0);

    // shunt the yard
    for t in input {
        let priority = t.priority();

        if let Some(pri) = priority {
            loop {
                let last = opstack.last();

                if let Some(la) = last {
                    if la.priority().unwrap_or(0) >= pri {
                        postfix.push(opstack.pop().unwrap())  // safe to unwrap here
                    } else {
                        opstack.push(t);
                        break
                    }
                } else {
                    opstack.push(t);
                    break
                }
            }
        } else {
            match t {
                Token::Close => {
                    par_count.1 += 1;
                    while let Some(x) = opstack.pop() {
                        if x == Token::Open {
                            break
                        }
                        postfix.push(x);
                    }
                },
                Token::Open => {
                    par_count.0 += 1;
                    opstack.push(t)
                },
                Token::Variable => postfix.push(t),
                Token::Number(x) => postfix.push(Token::Number(x)),
                _ => ()
            }
        }
    }
    while let Some(x) = opstack.pop() {
        postfix.push(x)
    };

    if par_count.0 != par_count.1 {
        return Err(ShuntingError::MissingParenthesisError)
    }

    Ok(postfix)
}

pub fn solve_postfix(input: Vec<Token>, variable: Option<f32>) -> Result<f32, ShuntingError> {
    // first one poped is the second number in eq
    let mut numstack: Vec<f32> = Vec::new();

    for t in input {
        match t {
            Token::Variable => if let Some(x) = variable {numstack.push(x)} else {return Err(ShuntingError::SolvingError)},
            Token::Number(x) => numstack.push(x as f32),
            _ => {
                // only the operators should get this far so its safe to pop off
                let b = numstack.pop().ok_or(ShuntingError::SolvingError)?;
                let a = numstack.pop().ok_or(ShuntingError::SolvingError)?;
                match t {
                    Token::Add => numstack.push(a + b),
                    Token::Sub => numstack.push(a - b),
                    Token::Mul => numstack.push(a * b),
                    Token::Div => numstack.push(a / b),
                    Token::Exp => numstack.push(a.powf(b)),
                    _ => ()
                }
            }
        }
    }

    if numstack.len() > 1 || numstack.len() < 1 {return Err(ShuntingError::UnequalOperatorNumberError)}

    Ok(numstack.get(0).unwrap().to_owned())
}

pub fn solve(input: &String, variable: Option<f32>) -> Result<f32, ShuntingError> {
    let postfix = shunt(input)?;
    solve_postfix(postfix, variable)
}