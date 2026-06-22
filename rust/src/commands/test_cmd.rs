/// test / [: evaluate expression.
/// Supports `test EXPR` and `[ EXPR ]` forms.
use std::fs;
use std::io::Write;

fn is_file(s: &str) -> bool {
    fs::metadata(s).is_ok()
}

fn is_dir(s: &str) -> bool {
    fs::metadata(s).map(|m| m.is_dir()).unwrap_or(false)
}

fn file_exists(s: &str) -> bool {
    fs::metadata(s).is_ok()
}

fn is_executable(s: &str) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::metadata(s)
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        let _ = s;
        false
    }
}

fn is_writable(s: &str) -> bool {
    fs::metadata(s)
        .map(|m| !m.permissions().readonly())
        .unwrap_or(false)
}

fn is_readable(s: &str) -> bool {
    fs::metadata(s).is_ok()
}

fn parse_num(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    // Allow leading whitespace and optional sign
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    s.parse::<i64>().ok()
}

enum Token {
    Unary(String, String),
    Binary(String, String, String),
    Not,
}

fn tokenize(args: &[String]) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "!" => {
                tokens.push(Token::Not);
                i += 1;
            }
            "-n" | "-z" | "-f" | "-d" | "-e" | "-x" | "-w" | "-r" | "-s" => {
                if i + 1 >= args.len() {
                    return Err(format!("test: missing argument after '{}'", arg));
                }
                tokens.push(Token::Unary(arg.to_string(), args[i + 1].clone()));
                i += 2;
            }
            "=" | "!=" | "-eq" | "-ne" | "-lt" | "-le" | "-gt" | "-ge" => {
                if tokens.is_empty() {
                    return Err(format!("test: missing argument before '{}'", arg));
                }
                // Find the left operand — should be the last string token
                match tokens.pop() {
                    Some(Token::Unary(_, val)) | Some(Token::Binary(val, _, _)) => {
                        if i + 1 >= args.len() {
                            return Err(format!("test: missing argument after '{}'", arg));
                        }
                        let right = args[i + 1].clone();
                        tokens.push(Token::Binary(val, arg.to_string(), right));
                        i += 2;
                    }
                    Some(Token::Not) => {
                        return Err("test: unexpected '!' before binary operator".to_string());
                    }
                    None => {
                        return Err(format!("test: missing argument before '{}'", arg));
                    }
                }
            }
            _ => {
                // Plain string — treat as unary -n
                tokens.push(Token::Unary("-n".to_string(), arg.clone()));
                i += 1;
            }
        }
    }
    Ok(tokens)
}

fn eval_unary(op: &str, val: &str) -> bool {
    match op {
        "-n" => !val.is_empty(),
        "-z" => val.is_empty(),
        "-f" => is_file(val) && !is_dir(val),
        "-d" => is_dir(val),
        "-e" => file_exists(val),
        "-x" => is_executable(val),
        "-w" => is_writable(val),
        "-r" => is_readable(val),
        "-s" => fs::metadata(val).map(|m| m.len() > 0).unwrap_or(false),
        _ => false,
    }
}

fn eval_binary(left: &str, op: &str, right: &str) -> bool {
    match op {
        "=" => left == right,
        "!=" => left != right,
        "-eq" => parse_num(left).zip(parse_num(right)).map_or(false, |(a, b)| a == b),
        "-ne" => parse_num(left).zip(parse_num(right)).map_or(false, |(a, b)| a != b),
        "-lt" => parse_num(left).zip(parse_num(right)).map_or(false, |(a, b)| a < b),
        "-le" => parse_num(left).zip(parse_num(right)).map_or(false, |(a, b)| a <= b),
        "-gt" => parse_num(left).zip(parse_num(right)).map_or(false, |(a, b)| a > b),
        "-ge" => parse_num(left).zip(parse_num(right)).map_or(false, |(a, b)| a >= b),
        _ => false,
    }
}

fn eval_tokens(tokens: &[Token]) -> Result<bool, String> {
    let mut result = true;
    let mut not_next = false;
    let mut i = 0;

    while i < tokens.len() {
        let token = &tokens[i];
        let val = match token {
            Token::Not => {
                not_next = !not_next;
                i += 1;
                continue;
            }
            Token::Unary(op, val) => eval_unary(op, val),
            Token::Binary(left, op, right) => eval_binary(left, op, right),
        };

        let val = if not_next { !val } else { val };
        not_next = false;

        if i == 0 {
            result = val;
        } else {
            result = val; // In the absence of -a/-o, multiple tokens are AND-like
        }
        i += 1;
    }

    Ok(result)
}

pub fn run(_w: &mut dyn Write, args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("test: missing operand");
        return 1;
    }

    let test_args: &[String] = if args[0] == "[" {
        // [ EXPR ] form — need closing ]
        if args.len() < 2 || args[args.len() - 1] != "]" {
            eprintln!("test: missing ']'");
            return 1;
        }
        &args[1..args.len() - 1]
    } else {
        args
    };

    if test_args.is_empty() {
        // test with no arguments returns false (exit 1)
        return 1;
    }

    match tokenize(test_args) {
        Ok(tokens) => {
            match eval_tokens(&tokens) {
                Ok(true) => 0,
                Ok(false) => 1,
                Err(e) => {
                    eprintln!("{}", e);
                    return 1;
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_unary_n() {
        assert!(eval_unary("-n", "hello"));
        assert!(!eval_unary("-n", ""));
    }

    #[test]
    fn test_eval_unary_z() {
        assert!(eval_unary("-z", ""));
        assert!(!eval_unary("-z", "x"));
    }

    #[test]
    fn test_eval_binary_eq() {
        assert!(eval_binary("a", "=", "a"));
        assert!(!eval_binary("a", "=", "b"));
    }

    #[test]
    fn test_eval_binary_neq() {
        assert!(eval_binary("a", "!=", "b"));
        assert!(!eval_binary("a", "!=", "a"));
    }

    #[test]
    fn test_eval_binary_num() {
        assert!(eval_binary("5", "-eq", "5"));
        assert!(eval_binary("3", "-lt", "5"));
        assert!(eval_binary("7", "-gt", "2"));
        assert!(eval_binary("3", "-le", "3"));
        assert!(eval_binary("5", "-ge", "5"));
        assert!(!eval_binary("5", "-eq", "6"));
    }

    #[test]
    fn test_tokenize_unary() {
        let args = vec!["-n".to_string(), "hello".to_string()];
        let tokens = tokenize(&args).unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Unary(op, val) => {
                assert_eq!(op.as_str(), "-n");
                assert_eq!(val, "hello");
            }
            _ => panic!("expected Unary"),
        }
    }

    #[test]
    fn test_tokenize_binary() {
        let args = vec!["a".to_string(), "=".to_string(), "b".to_string()];
        let tokens = tokenize(&args).unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Binary(left, op, right) => {
                assert_eq!(left, "a");
                assert_eq!(op.as_str(), "=");
                assert_eq!(right, "b");
            }
            _ => panic!("expected Binary"),
        }
    }

    #[test]
    fn test_test_no_args() {
        assert_eq!(run(&mut std::io::sink(), &[]), 1);
    }

    #[test]
    fn test_test_string() {
        assert_eq!(run(&mut std::io::sink(), &["hello".into()]), 0);
    }

    #[test]
    fn test_test_empty_string() {
        assert_eq!(run(&mut std::io::sink(), &["''".into()]), 0);
    }

    #[test]
    fn test_test_n_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-n".into(), "hello".into()]), 0);
    }

    #[test]
    fn test_test_z_flag() {
        assert_eq!(run(&mut std::io::sink(), &["-z".into(), "".into()]), 0);
    }

    #[test]
    fn test_test_eq() {
        assert_eq!(run(&mut std::io::sink(), &["a".into(), "=".into(), "a".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["a".into(), "=".into(), "b".into()]), 1);
    }

    #[test]
    fn test_test_neq() {
        assert_eq!(run(&mut std::io::sink(), &["a".into(), "!=".into(), "b".into()]), 0);
    }

    #[test]
    fn test_test_num_eq() {
        assert_eq!(run(&mut std::io::sink(), &["5".into(), "-eq".into(), "5".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["5".into(), "-eq".into(), "6".into()]), 1);
    }

    #[test]
    fn test_test_num_lt() {
        assert_eq!(run(&mut std::io::sink(), &["3".into(), "-lt".into(), "5".into()]), 0);
        assert_eq!(run(&mut std::io::sink(), &["5".into(), "-lt".into(), "3".into()]), 1);
    }

    #[test]
    fn test_test_not() {
        assert_eq!(run(&mut std::io::sink(), &["!".into(), "".into()]), 0);
    }

    #[test]
    fn test_test_bracket_form() {
        assert_eq!(run(&mut std::io::sink(), &["[".into(), "hello".into(), "]".into()]), 0);
    }

    #[test]
    fn test_test_bracket_form_false() {
        assert_eq!(run(&mut std::io::sink(), &["[".into(), "".into(), "]".into()]), 1);
    }

    #[test]
    fn test_test_bracket_missing_close() {
        assert_eq!(run(&mut std::io::sink(), &["[".into(), "hello".into()]), 1);
    }

    #[test]
    fn test_test_dev_null_exists() {
        assert_eq!(run(&mut std::io::sink(), &["-e".into(), "/dev/null".into()]), 0);
    }

    #[test]
    fn test_test_dev_null_is_file() {
        assert_eq!(run(&mut std::io::sink(), &["-f".into(), "/dev/null".into()]), 0);
    }
}
