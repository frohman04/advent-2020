use std::fmt::Debug;

fn main() {
    let exprs = std::fs::read_to_string("src/bin/day18.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| Term::from_str(val))
                .collect::<Vec<Term>>()
        })
        .expect("Unable to open file");
    println!("{:?}", exprs.iter().map(|expr| expr.eval()).sum::<u64>());
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Lit(u64),
    Plus,
    Star,
    LParen,
    RParen,
}

impl Token {
    pub fn from_raw(char: char) -> Option<Token> {
        match char {
            '+' => Some(Token::Plus),
            '*' => Some(Token::Star),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '0'..='9' => Some(Token::Lit(
                char.to_string()
                    .parse::<u64>()
                    .expect("Unable to parse number"),
            )),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Term {
    Lit(u64),
    Add(Box<Term>, Box<Term>),
    Mul(Box<Term>, Box<Term>),
}

impl Term {
    pub fn from_str(line: &str) -> Term {
        let tokens = Term::parse(line);
        Term::lex(tokens)
    }

    fn parse(line: &str) -> Vec<Token> {
        let mut merged_tokens: Vec<Token> = Vec::new();
        for token in line.chars().filter_map(|c| Token::from_raw(c)) {
            let last_token = merged_tokens.last().cloned();
            match (last_token, token) {
                (Some(Token::Lit(lhs)), Token::Lit(rhs)) => {
                    merged_tokens.remove(merged_tokens.len() - 1);
                    merged_tokens.push(Token::Lit(lhs * 10 + rhs));
                }
                (_, t) => merged_tokens.push(t),
            }
        }
        merged_tokens
    }

    fn lex(tokens: Vec<Token>) -> Term {
        let mut state = LexState::new();
        let mut stack: Vec<LexState> = Vec::new();
        for token in tokens.iter() {
            let curr_state = state.clone();
            match token {
                Token::Lit(val) => match (&curr_state.lhs, &curr_state.op) {
                    (None, _) => state.lhs = Some(Term::Lit(*val)),
                    (Some(lhs), Some(Token::Plus)) => {
                        state.reset(Term::Add(Box::new(lhs.clone()), Box::new(Term::Lit(*val))))
                    }
                    (Some(lhs), Some(Token::Star)) => {
                        state.reset(Term::Mul(Box::new(lhs.clone()), Box::new(Term::Lit(*val))))
                    }
                    _ => panic!(
                        "Invalid state encountered while parsing expression {:?}",
                        tokens
                    ),
                },
                Token::Plus => state.op = Some(Token::Plus),
                Token::Star => state.op = Some(Token::Star),
                Token::LParen => {
                    stack.push(state.clone());
                    state = LexState::new();
                }
                Token::RParen => match (&curr_state.lhs, &curr_state.op) {
                    (_, Some(_)) => panic!("Invalid state when closing parens {:?}", tokens),
                    (Some(lhs), None) => {
                        let prev_state = stack.pop().expect(&format!(
                            "No prev state available when closing parens {:?}",
                            tokens
                        ));
                        match (&prev_state.lhs, &prev_state.op) {
                            (None, None) => state.reset(lhs.clone()),
                            (Some(prev_lhs), Some(Token::Plus)) => state.reset(Term::Add(
                                Box::new(prev_lhs.clone()),
                                Box::new(lhs.clone()),
                            )),
                            (Some(prev_lhs), Some(Token::Star)) => state.reset(Term::Mul(
                                Box::new(prev_lhs.clone()),
                                Box::new(lhs.clone()),
                            )),
                            _ => panic!(
                                "Invalid state encountered while parsing expression {:?}",
                                tokens
                            ),
                        }
                    }
                    _ => panic!("Invalid state when closing parens {:?}", tokens),
                },
            }
        }
        if !stack.is_empty() || state.op.is_some() {
            panic!("Invalid state encountered before return {:?}", tokens);
        }
        state
            .lhs
            .expect(&format!("No lhs found at end of lexing {:?}", tokens))
    }

    pub fn eval(&self) -> u64 {
        match self {
            Term::Lit(val) => *val,
            Term::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
            Term::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
        }
    }
}

#[derive(Debug, Clone)]
struct LexState {
    pub lhs: Option<Term>,
    pub op: Option<Token>,
}

impl LexState {
    pub fn new() -> LexState {
        LexState {
            lhs: None,
            op: None,
        }
    }

    pub fn reset(&mut self, new_lhs: Term) -> () {
        self.lhs = Some(new_lhs);
        self.op = None;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_term_lit_eval() {
        assert_eq!(Term::Lit(42).eval(), 42)
    }

    #[test]
    fn test_term_add_eval() {
        assert_eq!(
            Term::Add(Box::new(Term::Lit(40)), Box::new(Term::Lit(2))).eval(),
            42
        )
    }

    #[test]
    fn test_term_mul_eval() {
        assert_eq!(
            Term::Mul(Box::new(Term::Lit(21)), Box::new(Term::Lit(2))).eval(),
            42
        )
    }

    #[test]
    fn test_term_parse_single_lit() {
        assert_eq!(Term::parse("1"), vec![Token::Lit(1)])
    }

    #[test]
    fn test_term_parse_consecutive_lit() {
        assert_eq!(Term::parse("1234"), vec![Token::Lit(1234)])
    }

    #[test]
    fn test_term_parse_add() {
        assert_eq!(Term::parse("+"), vec![Token::Plus])
    }

    #[test]
    fn test_term_parse_mul() {
        assert_eq!(Term::parse("*"), vec![Token::Star])
    }

    #[test]
    fn test_term_parse_lparen() {
        assert_eq!(Term::parse("("), vec![Token::LParen])
    }

    #[test]
    fn test_term_parse_rparen() {
        assert_eq!(Term::parse(")"), vec![Token::RParen])
    }

    #[test]
    fn test_term_parse_mixed() {
        assert_eq!(
            Term::parse("1 + (23 * 2)"),
            vec![
                Token::Lit(1),
                Token::Plus,
                Token::LParen,
                Token::Lit(23),
                Token::Star,
                Token::Lit(2),
                Token::RParen
            ]
        )
    }

    #[test]
    fn test_parse_expr_lit() {
        assert_eq!(Term::from_str("42"), Term::Lit(42))
    }

    #[test]
    fn test_parse_expr_add() {
        assert_eq!(
            Term::from_str("40 + 2"),
            Term::Add(Box::new(Term::Lit(40)), Box::new(Term::Lit(2)))
        )
    }

    #[test]
    fn test_parse_expr_mul() {
        assert_eq!(
            Term::from_str("21 * 2"),
            Term::Mul(Box::new(Term::Lit(21)), Box::new(Term::Lit(2)))
        )
    }

    #[test]
    fn test_parse_expr_chain() {
        assert_eq!(
            Term::from_str("10 + 11 * 2"),
            Term::Mul(
                Box::new(Term::Add(Box::new(Term::Lit(10)), Box::new(Term::Lit(11)))),
                Box::new(Term::Lit(2))
            )
        )
    }

    #[test]
    fn test_parse_expr_paren() {
        assert_eq!(
            Term::from_str("10 + (11 * 2)"),
            Term::Add(
                Box::new(Term::Lit(10)),
                Box::new(Term::Mul(Box::new(Term::Lit(11)), Box::new(Term::Lit(2))))
            )
        )
    }

    #[test]
    fn test_parse_expr_paren_noop() {
        assert_eq!(Term::from_str("(42)"), Term::Lit(42))
    }

    #[test]
    fn test_parse_expr_paren_nested() {
        assert_eq!(
            Term::from_str("10 + (11 * (2 + 4))"),
            Term::Add(
                Box::new(Term::Lit(10)),
                Box::new(Term::Mul(
                    Box::new(Term::Lit(11)),
                    Box::new(Term::Add(Box::new(Term::Lit(2)), Box::new(Term::Lit(4))))
                ))
            )
        )
    }

    #[test]
    fn test_parse_expr_paren_double_open() {
        assert_eq!(
            Term::from_str("10 + ((11 + 2) * (2 + 4))"),
            Term::Add(
                Box::new(Term::Lit(10)),
                Box::new(Term::Mul(
                    Box::new(Term::Add(Box::new(Term::Lit(11)), Box::new(Term::Lit(2)))),
                    Box::new(Term::Add(Box::new(Term::Lit(2)), Box::new(Term::Lit(4))))
                ))
            )
        )
    }
}
