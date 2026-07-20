/// expr: evaluate expressions.
use std::io::Write;
use crate::pwriteln;

#[derive(Debug, Clone)]
enum Token {
    Number(i64),
    String(String),
    Op(String),
    LParen,
    RParen,
    Keyword(String), // length, substr, index, match
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn expect(&mut self, msg: &str) -> Result<Token, String> {
        self.next().ok_or_else(|| format!("expr: {}", msg))
    }

    /// Parse the full expression.
    fn parse(&mut self) -> Result<String, String> {
        let result = self.parse_or()?;
        Ok(result)
    }

    // | operator (lowest precedence)
    fn parse_or(&mut self) -> Result<String, String> {
        let mut left = self.parse_and()?;
        while self.peek().map_or(false, |t| matches!(t, Token::Op(s) if s == "|")) {
            self.next();
            let right = self.parse_and()?;
            // | returns left if non-zero/non-null, else right
            if is_non_zero_non_null(&left) {
                // keep left
            } else {
                left = right;
            }
        }
        Ok(left)
    }

    // & operator
    fn parse_and(&mut self) -> Result<String, String> {
        let mut left = self.parse_cmp()?;
        while self.peek().map_or(false, |t| matches!(t, Token::Op(s) if s == "&")) {
            self.next();
            let right = self.parse_cmp()?;
            // & returns left if both non-zero/non-null, else 0
            if is_non_zero_non_null(&left) && is_non_zero_non_null(&right) {
                // keep left
            } else {
                left = "0".to_string();
            }
        }
        Ok(left)
    }

    // Comparison operators: = != < <= > >=
    fn parse_cmp(&mut self) -> Result<String, String> {
        let left = self.parse_arith()?;
        if let Some(Token::Op(op)) = self.peek().cloned() {
            match op.as_str() {
                "=" | "!=" | "<" | "<=" | ">" | ">=" => {
                    self.next();
                    let right = self.parse_arith()?;
                    return Ok(compare_str(&left, &op, &right));
                }
                _ => {}
            }
        }
        Ok(left)
    }

    // Addition/subtraction
    fn parse_arith(&mut self) -> Result<String, String> {
        let mut left = self.parse_term()?;
        while let Some(Token::Op(op)) = self.peek().cloned() {
            match op.as_str() {
                "+" | "-" => {
                    self.next();
                    let right = self.parse_term()?;
                    left = arith_op(&left, &op, &right)?;
                }
                _ => break,
            }
        }
        Ok(left)
    }

    // Multiplication/division/modulo (and match operator ':')
    fn parse_term(&mut self) -> Result<String, String> {
        let mut left = self.parse_factor()?;
        loop {
            match self.peek().cloned() {
                Some(Token::Op(op)) if matches!(op.as_str(), "*" | "/" | "%") => {
                    self.next();
                    let right = self.parse_factor()?;
                    left = arith_op(&left, &op, &right)?;
                }
                Some(Token::Op(op)) if op == ":" => {
                    self.next();
                    let right = self.parse_factor()?;
                    left = regex_match(&left, &right);
                }
                _ => break,
            }
        }
        Ok(left)
    }

    // Primary expressions: number, string, parenthesized, keyword functions
    fn parse_factor(&mut self) -> Result<String, String> {
        match self.peek().cloned() {
            Some(Token::Keyword(k)) if k == "length" => {
                self.next();
                let arg = self.parse_factor()?;
                Ok(arg.len().to_string())
            }
            Some(Token::Keyword(k)) if k == "substr" => {
                self.next();
                let s = self.expect("expected string after substr")?;
                let pos_str = self.expect("expected position after substr")?;
                let len_str = self.expect("expected length after substr")?;
                let s_val = token_value(&s);
                let pos = token_value(&pos_str).parse::<usize>().unwrap_or(1);
                let len = token_value(&len_str).parse::<usize>().unwrap_or(0);
                if pos < 1 || pos > s_val.len() || len == 0 {
                    return Ok("".to_string());
                }
                let start = pos - 1;
                let end = std::cmp::min(start + len, s_val.len());
                Ok(s_val[start..end].to_string())
            }
            Some(Token::Keyword(k)) if k == "index" => {
                self.next();
                let s = self.expect("expected string after index")?;
                let chars = self.expect("expected chars after index")?;
                let s_val = token_value(&s);
                let chars_val = token_value(&chars);
                match s_val.find(|c| chars_val.contains(c)) {
                    Some(pos) => Ok((pos + 1).to_string()),
                    None => Ok("0".to_string()),
                }
            }
            Some(Token::Keyword(k)) if k == "match" => {
                self.next();
                let s = self.expect("expected string after match")?;
                let regex = self.expect("expected regex after match")?;
                Ok(regex_match(&token_value(&s), &token_value(&regex)))
            }
            Some(Token::LParen) => {
                self.next();
                let result = self.parse_or()?;
                match self.peek() {
                    Some(Token::RParen) => {
                        self.next();
                        Ok(result)
                    }
                    _ => Err("missing closing parenthesis".to_string()),
                }
            }
            Some(Token::Number(n)) => {
                self.next();
                Ok(n.to_string())
            }
            Some(Token::String(s)) => {
                self.next();
                Ok(s)
            }
            Some(Token::RParen) => Err("unexpected closing parenthesis".to_string()),
            Some(Token::Op(o)) => Err(format!("unexpected operator '{}'", o)),
            Some(Token::Keyword(k)) => Err(format!("unknown keyword '{}'", k)),
            None => Err("missing operand".to_string()),
        }
    }
}

fn is_non_zero_non_null(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // If it's a number, check != 0
    if let Ok(n) = s.parse::<i64>() {
        return n != 0;
    }
    // Non-empty non-numeric string is truthy
    !s.is_empty()
}

fn compare_str(left: &str, op: &str, right: &str) -> String {
    // Try numeric comparison first
    if let (Ok(l), Ok(r)) = (left.parse::<i64>(), right.parse::<i64>()) {
        let result = match op {
            "=" => l == r,
            "!=" => l != r,
            "<" => l < r,
            "<=" => l <= r,
            ">" => l > r,
            ">=" => l >= r,
            _ => false,
        };
        return if result { "1".to_string() } else { "0".to_string() };
    }

    // String comparison
    let result = match op {
        "=" => left == right,
        "!=" => left != right,
        "<" => left < right,
        "<=" => left <= right,
        ">" => left > right,
        ">=" => left >= right,
        _ => false,
    };
    if result { "1".to_string() } else { "0".to_string() }
}

fn arith_op(left: &str, op: &str, right: &str) -> Result<String, String> {
    let l = left.parse::<i64>().map_err(|_| format!("non-numeric argument: {}", left))?;
    let r = right.parse::<i64>().map_err(|_| format!("non-numeric argument: {}", right))?;
    let result = match op {
        "+" => l.wrapping_add(r),
        "-" => l.wrapping_sub(r),
        "*" => l.wrapping_mul(r),
        "/" => {
            if r == 0 {
                return Err("division by zero".to_string());
            }
            l / r
        }
        "%" => {
            if r == 0 {
                return Err("division by zero".to_string());
            }
            l % r
        }
        _ => unreachable!(),
    };
    Ok(result.to_string())
}

fn regex_match(text: &str, pattern: &str) -> String {
    // Basic regex match implementation without regex crate dependency
    // Supports basic . and * patterns
    if let Ok(re) = regex_lite(pattern) {
        if let Some(m) = re.find(text) {
            return m.to_string();
        }
    }
    "0".to_string()
}

/// Minimal regex engine supporting literal text, ., and *
struct RegexLite {
    pattern: Vec<PatternPiece>,
}

enum PatternPiece {
    Literal(char),
    Dot,
    Star(char),
    DotStar,
}

fn regex_lite(pattern: &str) -> Result<RegexLite, ()> {
    let mut pieces = Vec::new();
    let mut chars = pattern.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '.' => {
                if chars.peek() == Some(&'*') {
                    chars.next();
                    pieces.push(PatternPiece::DotStar);
                } else {
                    pieces.push(PatternPiece::Dot);
                }
            }
            '*' => {
                // Star after a literal or standalone
                if let Some(last) = pieces.last_mut() {
                    match last {
                        PatternPiece::Literal(ch) => {
                            let ch = *ch;
                            pieces.pop();
                            pieces.push(PatternPiece::Star(ch));
                        }
                        _ => {}
                    }
                }
            }
            '\\' => {
                if let Some(next) = chars.next() {
                    pieces.push(PatternPiece::Literal(next));
                } else {
                    pieces.push(PatternPiece::Literal(c));
                }
            }
            _ => {
                if chars.peek() == Some(&'*') {
                    chars.next();
                    pieces.push(PatternPiece::Star(c));
                } else {
                    pieces.push(PatternPiece::Literal(c));
                }
            }
        }
    }
    Ok(RegexLite { pattern: pieces })
}

impl RegexLite {
    fn find<'a>(&self, text: &'a str) -> Option<&'a str> {
        let text_bytes = text.as_bytes();
        let text_len = text_bytes.len();

        // Try matching at each position
        for start in 0..=text_len {
            if let Some(end) = self.match_at(text_bytes, start) {
                return Some(&text[start..end]);
            }
        }
        None
    }

    fn match_at(&self, text: &[u8], start: usize) -> Option<usize> {
        let mut pos = start;
        for piece in &self.pattern {
            match piece {
                PatternPiece::Literal(c) => {
                    if pos >= text.len() || text[pos] as char != *c {
                        return None;
                    }
                    pos += 1;
                }
                PatternPiece::Dot => {
                    if pos >= text.len() {
                        return None;
                    }
                    pos += 1;
                }
                PatternPiece::Star(c) => {
                    if *c == '\0' {
                        continue; // * at start = match anything
                    }
                    // Skip all matching chars
                    while pos < text.len() && text[pos] as char == *c {
                        pos += 1;
                    }
                }
                PatternPiece::DotStar => {
                    // Try to match the rest of the pattern after the dot-star
                    // Invariant: we are inside the DotStar match arm, so the pattern
                    // is guaranteed to contain at least one PatternPiece::DotStar.
                    let dotstar_pos = self
                        .pattern
                        .iter()
                        .position(|p| matches!(p, PatternPiece::DotStar))
                        .unwrap_or_else(|| unreachable!("DotStar arm matched but no DotStar piece found in pattern"));
                    let remaining = &self.pattern[dotstar_pos + 1..];
                    if remaining.is_empty() {
                        pos = text.len();
                    } else {
                        // Greedy: start from end and work backwards
                        let mut best_match = None;
                        'search: for end_pos in (pos..=text.len()).rev() {
                            let mut test_pos = end_pos;
                            for rp in remaining {
                                match rp {
                                    PatternPiece::Literal(c) => {
                                        if test_pos >= text.len() || text[test_pos] as char != *c {
                                            continue 'search;
                                        }
                                        test_pos += 1;
                                    }
                                    PatternPiece::Dot => {
                                        if test_pos >= text.len() {
                                            continue 'search;
                                        }
                                        test_pos += 1;
                                    }
                                    _ => {
                                        // Nested star - skip for now
                                        break;
                                    }
                                }
                            }
                            best_match = Some(end_pos);
                            break;
                        }
                        pos = best_match?;
                    }
                }
            }
        }
        Some(pos)
    }
}

fn token_value(t: &Token) -> String {
    match t {
        Token::Number(n) => n.to_string(),
        Token::String(s) => s.clone(),
        _ => String::new(),
    }
}

fn tokenize(args: &[String]) -> Vec<Token> {
    let mut tokens = Vec::new();
    for arg in args {
        if arg == "(" {
            tokens.push(Token::LParen);
        } else if arg == ")" {
            tokens.push(Token::RParen);
        } else if arg == "|" || arg == "&" {
            tokens.push(Token::Op(arg.to_string()));
        } else if arg == "=" || arg == "!=" {
            tokens.push(Token::Op(arg.to_string()));
        } else if arg == "<" || arg == "<=" || arg == ">" || arg == ">=" {
            tokens.push(Token::Op(arg.to_string()));
        } else if arg == "+" || arg == "-" || arg == "*" || arg == "/" || arg == "%" {
            tokens.push(Token::Op(arg.to_string()));
        } else if arg == ":" {
            tokens.push(Token::Op(":".to_string()));
        } else if arg == "length" {
            tokens.push(Token::Keyword("length".to_string()));
        } else if arg == "substr" {
            tokens.push(Token::Keyword("substr".to_string()));
        } else if arg == "index" {
            tokens.push(Token::Keyword("index".to_string()));
        } else if arg == "match" {
            tokens.push(Token::Keyword("match".to_string()));
        } else if let Ok(n) = arg.parse::<i64>() {
            tokens.push(Token::Number(n));
        } else {
            tokens.push(Token::String(arg.clone()));
        }
    }
    tokens
}

pub fn run(w: &mut dyn Write, args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("expr: missing operand");
        return 1;
    }

    let tokens = tokenize(args);
    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(result) => {
            pwriteln!(w, "{}", result);
            // Exit code: 0 if result is non-zero/non-null, 1 if zero/null
            if is_non_zero_non_null(&result) {
                0
            } else {
                1
            }
        }
        Err(e) => {
            eprintln!("expr: {}", e);
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_expr_number() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["42".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "42");
    }

    #[test]
    fn test_expr_addition() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["1".into(), "+".into(), "2".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "3");
    }

    #[test]
    fn test_expr_subtraction() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["10".into(), "-".into(), "3".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "7");
    }

    #[test]
    fn test_expr_multiplication() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["3".into(), "*".into(), "4".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "12");
    }

    #[test]
    fn test_expr_division() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["10".into(), "/".into(), "3".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "3");
    }

    #[test]
    fn test_expr_modulo() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["10".into(), "%".into(), "3".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "1");
    }

    #[test]
    fn test_expr_equality() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["5".into(), "=".into(), "5".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "1");
    }

    #[test]
    fn test_expr_inequality() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["5".into(), "=".into(), "6".into()]), 1);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "0");
    }

    #[test]
    fn test_expr_not_equal() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["5".into(), "!=".into(), "6".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "1");
    }

    #[test]
    fn test_expr_less_than() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["3".into(), "<".into(), "5".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "1");
    }

    #[test]
    fn test_expr_greater_than() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["5".into(), ">".into(), "3".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "1");
    }

    #[test]
    fn test_expr_parentheses() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["(".into(), "1".into(), "+".into(), "2".into(), ")".into(), "*".into(), "3".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "9");
    }

    #[test]
    fn test_expr_logical_or() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["0".into(), "|".into(), "5".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "5");
    }

    #[test]
    fn test_expr_logical_and() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["5".into(), "&".into(), "3".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "5");
    }

    #[test]
    fn test_expr_length() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["length".into(), "hello".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "5");
    }

    #[test]
    fn test_expr_substr() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["substr".into(), "hello".into(), "2".into(), "3".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "ell");
    }

    #[test]
    fn test_expr_index() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["index".into(), "hello".into(), "l".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "3");
    }

    #[test]
    fn test_expr_string_compare() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["abc".into(), "=".into(), "abc".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "1");
    }

    #[test]
    fn test_expr_precedence() {
        let mut buf = Vec::new();
        // 2 + 3 * 4 = 14 (not 20)
        assert_eq!(run(&mut buf, &["2".into(), "+".into(), "3".into(), "*".into(), "4".into()]), 0);
        assert_eq!(String::from_utf8_lossy(&buf).trim(), "14");
    }

    #[test]
    fn test_expr_division_by_zero() {
        let mut buf = Vec::new();
        assert_eq!(run(&mut buf, &["1".into(), "/".into(), "0".into()]), 2);
    }

    #[test]
    fn test_regex_lite() {
        let re = regex_lite("hello").unwrap();
        assert_eq!(re.find("hello world"), Some("hello"));

        let re = regex_lite("h.*o").unwrap();
        assert_eq!(re.find("hello"), Some("hello"));

        let re = regex_lite("h.llo").unwrap();
        assert_eq!(re.find("hello"), Some("hello"));
    }
}
