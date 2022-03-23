use core::fmt;
use std::{iter::Peekable, str::Chars, fmt::{Display, Formatter}, array::IntoIter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(i32),
    Operator(Op),
    Parenthesis(char),
    Unary(UnOp),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOp {
    Minus,
    Not
}

impl UnOp {
    fn apply(&self, arg1: i32) -> i32 {
        match self {
            Self::Minus => -arg1,
            Self::Not => !arg1
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus => f.write_str("-")?,
            Self::Not => f.write_str("NOT")?
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shr,
    Shl,
}

impl Op {
    fn precedence(&self) -> i32 {
        match self {
            Self::Or | Self::Xor => -2,
            Self::And => -1,
            Self::Add | Self::Sub => 1,
            Self::Mul | Self::Div | Self::Mod | Self::Shl | Self::Shr => 2,
        }
    }

    fn apply(&self, arg1: i32, arg2: i32) -> i32 {
        match self {
            Self::Add => arg1 + arg2,
            Self::Sub => arg1 - arg2,
            Self::Mul => arg1 * arg2,
            Self::Div => arg1 / arg2,
            Self::Mod => arg1 % arg2,
            Self::And => arg1 & arg2,
            Self::Or => arg1 | arg2,
            Self::Xor => arg1 ^ arg2,
            Self::Shr => arg1 >> arg2,
            Self::Shl => arg1 << arg2,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => f.write_str("+")?,
            Self::Sub => f.write_str("-")?,
            Self::Mul => f.write_str("*")?,
            Self::Div => f.write_str("/")?,
            Self::Mod => f.write_str("MOD")?,
            Self::And => f.write_str("AND")?,
            Self::Or => f.write_str("OR")?,
            Self::Xor => f.write_str("XOR")?,
            Self::Shr => f.write_str("SHR")?,
            Self::Shl => f.write_str("SHL")?,
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum Item {
    Number(i32),
    Operator(Op),
}

pub fn eval(expression: &str) -> i32 {
    eval_tokens(Tokenizer::new(expression)).expect("")
}


struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    previous: Option<Token>
}

impl<'a> Tokenizer<'a> {
    fn new(input_str: &'a str) -> Self {
        Self {
            chars: input_str.chars().peekable(), previous: None
        }
    }

    fn consume(&mut self, expected: &str) -> bool {
        for c in expected.chars() {
            if self.chars.next_if(|&x| x == c).is_none() {
                return false;
            }
        }
        true
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.chars.next() {
            self.previous = match c {
                '(' => Some(Token::Parenthesis('(')),
                ')' => Some(Token::Parenthesis(')')),
                '+' => Some(Token::Operator(Op::Add)),
                '-' => {
                    match &self.previous {
                        Some(token) => {
                            match token {
                                Token::Operator(_) => Some(Token::Unary(UnOp::Minus)),
                                Token::Parenthesis('(') => Some(Token::Unary(UnOp::Minus)),
                                _ => Some(Token::Operator(Op::Sub))
                            }
                        }
                        None => Some(Token::Unary(UnOp::Minus))
                    }
                }
                '*' => Some(Token::Operator(Op::Mul)),
                '/' => Some(Token::Operator(Op::Div)),
                'X' if self.consume("OR") => Some(Token::Operator(Op::Xor)),
                'A' if self.consume("ND") => Some(Token::Operator(Op::And)),
                'O' if self.consume("R") => Some(Token::Operator(Op::Or)),
                'S' if self.consume("H") => {
                    if self.consume("L") {
                        Some(Token::Operator(Op::Shl))
                    } else if self.consume("R") {
                        Some(Token::Operator(Op::Shr))
                    } else {
                        panic!();
                    }
                }
                'M' if self.consume("OD") => Some(Token::Operator(Op::Mod)),
                'N' if self.consume("OT") => Some(Token::Unary(UnOp::Not)),
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    let mut num_str = String::from(c);
                    while let Some(digit) = self.chars.next_if(|&x| x.is_ascii_hexdigit()) {
                        num_str.push(digit);
                    }
                    if let Some(post) = self.chars.peek() {
                        match post {
                            'H' => {
                                self.chars.next();
                                Some(Token::Number(i32::from_str_radix(&num_str, 16).unwrap()))
                            }
                            'O' | 'Q' => {
                                self.chars.next();
                                Some(Token::Number(i32::from_str_radix(&num_str, 8).unwrap()))
                            }
                            _ => Some(Token::Number(i32::from_str_radix(&num_str, 10).unwrap())),
                        }
                    } else {
                        match num_str.chars().last().unwrap() {
                            'B' => Some(Token::Number(i32::from_str_radix(&num_str[..num_str.len()-1], 2).unwrap())),
                            'D' => Some(Token::Number(i32::from_str_radix(&num_str[..num_str.len()-1], 10).unwrap())),
                            _ => Some(Token::Number(i32::from_str_radix(&num_str, 10).unwrap())),
                        }
                    }
                }
                c if c.is_whitespace() => self.next(),
                _ => panic!()
            };
            self.previous
        } else {
            None
        }
    }
}

pub fn eval_tokens<I>(tokens: I) -> Result<i32, String>
where
    I: Iterator<Item = Token>
{
    let mut stack: Vec<Token> = Vec::new();
    let mut args = Vec::new();
    for t in tokens {
        match t {
            Token::Number(v) => {
                args.push(v);
            }
            Token::Unary(_) => {
                stack.push(t);
            }
            Token::Operator(ref c) => {
                // If precedence of t is lower than the top of the stack
                // Pop stack until t has higher precedence than top
                while stack.len() > 0 {
                    if let Token::Parenthesis(_) = stack[stack.len() - 1] {
                        break;
                    }
                    if let Token::Unary(op) = stack[stack.len() - 1] {
                        let t1 = args.pop().ok_or(format!("Not enough arguments for unary operator: {}", &op))?;
                        args.push(op.apply(t1));
                        stack.pop();
                    } else if let Token::Operator(ref op) = stack[stack.len() - 1] {
                        if op.precedence() >= c.precedence() {
                            let t1 = args.pop().ok_or(format!("Not enough arguments for operator: {}", &op))?;
                            let t2 = args.pop().ok_or(format!("Not enough arguments for operator: {}", &op))?;
                            if let Token::Operator(top) = stack.pop().unwrap() {
                                args.push(top.apply(t1, t2));
                            }
                        } else {
                            break;
                        }
                    }
                }
                stack.push(t);
            }
            Token::Parenthesis(c) => match c {
                '(' => stack.push(t),
                ')' => {
                    while stack.len() > 0 {
                        if let Token::Parenthesis(_) = stack[stack.len() - 1] {
                            stack.pop();
                            break;
                        }
                        if let Token::Unary(op) = stack[stack.len() - 1] {
                            let t1 = args.pop().ok_or(format!("Not enough arguments for unary operator: {}", &op))?;
                            args.push(op.apply(t1));
                            stack.pop();
                        } else {
                            if let Token::Operator(op) = stack.pop().unwrap() {
                                let t2 = args.pop().ok_or(format!("Not enough arguments for operator: {}", &op))?;
                                let t1 = args.pop().ok_or(format!("Not enough arguments for operator: {}", &op))?;
                                args.push(op.apply(t1, t2));
                            }
                        }
                    }
                }
                _ => (),
            },
        }
    }
    // No more Tokens in input -> process the remaining operators on the stack
    while stack.len() > 0 {
        if let Token::Parenthesis(_) = stack[stack.len() - 1] {
            panic!("Parenthesis in stack after traversing all tokens");
        }
        if let Token::Unary(op) = stack[stack.len() - 1] {
            let t1 = args.pop().ok_or(format!("Not enough arguments for unary operator: {}", &op))?;
            args.push(op.apply(t1));
            stack.pop();
        } else if let Token::Operator(op) = stack.pop().unwrap() {
            let t2 = args.pop().ok_or(format!("Not enough arguments for operator: {}", &op))?;
            let t1 = args.pop().ok_or(format!("Not enough arguments for operator: {}", &op))?;
            args.push(op.apply(t1, t2));
        }
    }
    Ok(args.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        // Just a bunch of expressions I hope it covers enough cases
        let expressions = vec![
            ("3", 3),
            ("3 + 4", 7),
            ("3 - 4", -1),
            ("3 * 4", 12),
            ("3 / 3", 1),
            ("6 / 3", 2),
            ("3 * (4 + 2)", 18),
            ("3 + 4 * (4 + 2)", 27),
            ("(3) * (4 + 2)", 18),
            ("(((3)))", 3),
            //("3^2*3", 27),
            //("3^(2*3)", (3 as i32).pow(6)),
            //("3^2+3", 12),
            //("3*3^2+3", 30),
            ("-3", -3),
            ("3 + -4", -1),
            ("3*-(4+2)", -18),
            ("27 XOR 9", 27 ^ 9),
            ("6 AND 6", 6),
            ("200 OR 1", 201),
            ("15 MOD 5", 0),
            ("4 MOD 3", 1),
            ("8 MOD 9", 8),
            ("NOT 9", !9)
        ];
        for (expr, res) in expressions {
            let tokens = Tokenizer::new(expr);
            assert_eq!(
                eval_tokens(tokens).expect(""),
                res
            );
        }
    }

    #[test]
    fn erroneous_expressions() {
        let expressions = vec![
            ("4 +", "Not enough arguments for operator: +"),
            ("-", "Not enough arguments for unary operator: -"),
            ("NOT", "Not enough arguments for unary operator: NOT")
        ];
        for (expr, err) in expressions {
            let tokens = Tokenizer::new(expr);
            assert_eq!(eval_tokens(tokens), Err(String::from(err)));
        }
    }

    #[test]
    fn tokenizer() {
        for x in 0..1000 {
            let hex: &str = &format!("{:x}H", x);
            let oct: &str = &format!("{:o}O", x);
            let bin: &str = &format!("{:b}B", x);
            let dec: &str = &format!("{}D", x);
            let dec2: &str = &format!("{}", x);
            let mut t1 = Tokenizer::new(hex);
            let mut t2 = Tokenizer::new(oct);
            let mut t3 = Tokenizer::new(bin);
            let mut t4 = Tokenizer::new(dec);
            let mut t5 = Tokenizer::new(dec2);
            assert_eq!(t1.next(), Some(Token::Number(x)));
            assert_eq!(t2.next(), Some(Token::Number(x)));
            assert_eq!(t3.next(), Some(Token::Number(x)));
            assert_eq!(t4.next(), Some(Token::Number(x)));
            assert_eq!(t5.next(), Some(Token::Number(x)));
        }
    }
}
