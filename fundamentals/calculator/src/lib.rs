enum Expr {
    Number(i64),
    Square(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>)
}

#[derive(Debug)]
pub enum ParseError {
    NotEnoughNumbers,
    EvaluationError,
    InvalidInput
}

fn pop_and_push(stack: &mut Vec<Expr>, ctor: fn(Box<Expr>, Box<Expr>) -> Expr) -> Result<(), ParseError> {
    let b = stack.pop().ok_or(ParseError::NotEnoughNumbers)?;
    let a = stack.pop().ok_or(ParseError::NotEnoughNumbers)?;
    let e = ctor(Box::new(a), Box::new(b));
    stack.push(e);
    Ok(())
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();

    for word in input.split_ascii_whitespace() {
        
        match word {
            "+" => pop_and_push(&mut stack, Expr::Add)?,
            "-" => pop_and_push(&mut stack, Expr::Sub)?,
            "*" => pop_and_push(&mut stack, Expr::Mul)?,
            "/" => pop_and_push(&mut stack, Expr::Div)?,
            "sqr" => {
                let a = stack.pop().ok_or(ParseError::NotEnoughNumbers)?;
                let e = Expr::Square(Box::new(a));
                stack.push(e);
            },
            _ => {
                let Ok(n) = word.parse::<i64>() else {
                    return Err(ParseError::InvalidInput);
                };
                stack.push(Expr::Number(n));
            },
        };
    };

    assert_eq!(stack.len(), 1);

    Ok(stack.pop().unwrap())
}

#[derive(Debug)]
pub enum EvalError {
    Unsupported
}

pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Add(a, b) => Ok(eval(a).unwrap() + eval(b).unwrap()),
        Expr::Sub(a, b) => Ok(eval(a).unwrap() - eval(b).unwrap()),
        Expr::Mul(a, b) => Ok(eval(a).unwrap() * eval(b).unwrap()),
        Expr::Div(a, b) => Ok(eval(a).unwrap() / eval(b).unwrap()),
        Expr::Square(a) => {
            let value = eval(a).unwrap();
            Ok(value * value)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let input = "42";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn smoke_test() {
        let input = "3 sqr 4 sqr + 5 sqr -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 0);
    }

    #[test]
    fn debug_test() {
        let input = "3 4 + -5 +";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 2);
    }
}