use std::{iter::Peekable, str::Chars};
#[derive(Debug, PartialEq)]
enum Token {
    Number(i32),
    Operator(Op),
    Parenthesis(char),
    Unary,
}

#[derive(Debug, PartialEq)]
enum Op {
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
            Self::Add | Self::Sub => 0,
            Self::Mul | Self::Div | Self::Mod | Self::Shl | Self::Shr => 1,
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

pub fn eval(expression: &str) -> i32 {
    to_expression_tree(tokenize(expression.to_string())).evaluate()
}

#[derive(Debug)]
enum Item {
    Number(i32),
    Operator(Op),
}

#[derive(Debug)]
struct BinaryExpressionTree {
    root: Item,
    left: Option<Box<BinaryExpressionTree>>,
    right: Option<Box<BinaryExpressionTree>>,
}

impl BinaryExpressionTree {
    pub fn new(val: Item) -> Self {
        Self {
            root: val,
            left: None,
            right: None,
        }
    }

    pub fn from(val: Item, l: Self, r: Self) -> Self {
        Self {
            root: val,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }

    pub fn evaluate(&self) -> i32 {
        if let Item::Number(c) = &self.root {
            *c
        } else if let Item::Operator(op) = &self.root {
            op.apply(
                self.left.as_ref().unwrap().evaluate(),
                self.right.as_ref().unwrap().evaluate(),
            )
        } else {
            0
        }
    }
}

fn tokenize(expr: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();
    'outer: while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Token::Operator(Op::Add)),
            '-' => {
                if tokens.len() == 0 {
                    tokens.push(Token::Unary);
                    continue;
                }
                match &tokens[tokens.len() - 1] {
                    Token::Number(_) => tokens.push(Token::Operator(Op::Sub)),
                    _ => tokens.push(Token::Unary),
                };
            }
            '*' => tokens.push(Token::Operator(Op::Mul)),
            '/' => tokens.push(Token::Operator(Op::Div)),
            'X' if consume(&mut chars, "OR") => {
                tokens.push(Token::Operator(Op::Xor));
            }
            'O' if consume(&mut chars, "R") => {
                tokens.push(Token::Operator(Op::Xor));
            }
            'A' if consume(&mut chars, "ND") => tokens.push(Token::Operator(Op::And)),
            'S' if consume(&mut chars, "H") => {
                if let Some('L') = chars.next() {
                    tokens.push(Token::Operator(Op::Shl));
                } else if let Some('R') = chars.next() {
                    tokens.push(Token::Operator(Op::Shr));
                }
            }
            '(' | ')' => tokens.push(Token::Parenthesis(c)),
            '0'..='9' => {
                let mut num_str = String::from(c);
                while let Some(d) = chars.peek() {
                    match d {
                        '0'..='9' | 'a'..='f' => {
                            num_str.push(*d);
                            chars.next();
                        }
                        'H' => {
                            tokens.push(Token::Number(i32::from_str_radix(&num_str, 16).unwrap()));
                            chars.next();
                            continue 'outer;
                        }
                        'B' => {
                            tokens.push(Token::Number(i32::from_str_radix(&num_str, 2).unwrap()));
                            chars.next();
                            continue 'outer;
                        }
                        'O' => {
                            tokens.push(Token::Number(i32::from_str_radix(&num_str, 8).unwrap()));
                            chars.next();
                            continue 'outer;
                        }
                        _ => {
                            tokens.push(Token::Number(num_str.parse::<i32>().unwrap()));
                            continue 'outer;
                        }
                    }
                }
                tokens.push(Token::Number(num_str.parse::<i32>().unwrap()));
            }
            _ => (),
        }
    }
    tokens
}

fn consume(chars: &mut Peekable<impl Iterator<Item = char>>, expected: &str) -> bool {
    for c in expected.chars() {
        if chars.next_if(|&x| x == c).is_some() {
            chars.next();
        } else {
            return false;
        }
    }
    true
}

struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(input_str: &'a str) -> Self {
        Self {
            chars: input_str.chars().peekable(),
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
            match c {
                '+' => Some(Token::Operator(Op::Add)),
                '-' => Some(Token::Operator(Op::Sub)),
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
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    let mut num_str = String::from(c);
                    while let Some(digit) = self.chars.next_if(|&x| x.is_ascii_hexdigit()) {
                        num_str.push(digit);
                    }
                    if let Some(post) = self.chars.peek() {
                        return match post {
                            'H' => {
                                self.chars.next();
                                Some(Token::Number(i32::from_str_radix(&num_str, 16).unwrap()))
                            },
                            'O' | 'Q' => {
                                self.chars.next();
                                Some(Token::Number(i32::from_str_radix(&num_str, 8).unwrap()))
                            },
                            _ => Some(Token::Number(i32::from_str_radix(&num_str, 10).unwrap())),
                        }
                    }
                    match num_str.chars().last().unwrap() {
                        'B' => Some(Token::Number(i32::from_str_radix(&num_str[..num_str.len()-1], 2).unwrap())),
                        'D' => Some(Token::Number(i32::from_str_radix(&num_str[..num_str.len()-1], 10).unwrap())),
                        _ => Some(Token::Number(i32::from_str_radix(&num_str, 10).unwrap())),
                    }
                },
                c if c.is_whitespace() => self.next(),
                _ => panic!(),
            }
        } else {
            None
        }
    }
}

/*
* Convert Token vector to binary expression tree using the shunning yard algorithm
* RANGIERBAHNHOF
 */
fn to_expression_tree(tokens: Vec<Token>) -> BinaryExpressionTree {
    let mut stack: Vec<Token> = Vec::new();
    let mut trees: Vec<BinaryExpressionTree> = Vec::new();
    for t in tokens {
        match t {
            Token::Number(v) => trees.push(BinaryExpressionTree::new(Item::Number(v))),
            Token::Unary => {
                stack.push(t);
            }
            Token::Operator(ref c) => {
                // If precedence of t is lower than the top of the stack
                // Pop stack until t has higher precedence than top
                while stack.len() > 0 {
                    if let Token::Parenthesis(_) = stack[stack.len() - 1] {
                        break;
                    }
                    if let Token::Unary = stack[stack.len() - 1] {
                        let t1 = trees.pop().unwrap();
                        trees.push(BinaryExpressionTree::from(
                            Item::Operator(Op::Sub),
                            BinaryExpressionTree::new(Item::Number(0)),
                            t1,
                        ));
                        stack.pop();
                    } else if let Token::Operator(ref op) = stack[stack.len() - 1] {
                        if op.precedence() >= c.precedence() {
                            let t2 = trees.pop().unwrap();
                            let t1 = trees.pop().unwrap();
                            if let Token::Operator(top) = stack.pop().unwrap() {
                                trees.push(BinaryExpressionTree::from(Item::Operator(top), t1, t2));
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
                        if let Token::Unary = stack[stack.len() - 1] {
                            let t1 = trees.pop().unwrap();
                            trees.push(BinaryExpressionTree::from(
                                Item::Operator(Op::Sub),
                                BinaryExpressionTree::new(Item::Number(0)),
                                t1,
                            ));
                            stack.pop();
                        } else {
                            let t2 = trees.pop().unwrap();
                            let t1 = trees.pop().unwrap();
                            if let Token::Operator(op) = stack.pop().unwrap() {
                                trees.push(BinaryExpressionTree::from(Item::Operator(op), t1, t2));
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
        if let Token::Unary = stack[stack.len() - 1] {
            let t1 = trees.pop().unwrap();
            trees.push(BinaryExpressionTree::from(
                Item::Operator(Op::Sub),
                BinaryExpressionTree::new(Item::Number(0)),
                t1,
            ));
            stack.pop();
        } else if let Token::Operator(op) = stack.pop().unwrap() {
            let t2 = trees.pop().unwrap();
            let t1 = trees.pop().unwrap();
            trees.push(BinaryExpressionTree::from(Item::Operator(op), t1, t2));
        }
    }
    trees.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        ];
        for (expr, res) in expressions {
            assert_eq!(
                to_expression_tree(tokenize(String::from(expr))).evaluate(),
                res
            );
        }
    }

    #[test]
    fn tokenizer() {
        for x in 0..0x1235 {
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
