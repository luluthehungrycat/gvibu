/// seq: print a sequence of numbers.

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("seq: usage: seq [FIRST [STEP]] LAST");
        return 1;
    }

    let (first, step, last) = match args.len() {
        1 => {
            match args[0].parse::<i64>() {
                Ok(last) => (1i64, 1i64, last),
                Err(_) => {
                    eprintln!("seq: invalid number");
                    return 1;
                }
            }
        }
        2 => {
            match (args[0].parse::<i64>(), args[1].parse::<i64>()) {
                (Ok(first), Ok(last)) => (first, 1i64, last),
                _ => {
                    eprintln!("seq: invalid number");
                    return 1;
                }
            }
        }
        3 => {
            match (args[0].parse::<i64>(), args[1].parse::<i64>(), args[2].parse::<i64>()) {
                (Ok(first), Ok(step), Ok(last)) => (first, step, last),
                _ => {
                    eprintln!("seq: invalid number");
                    return 1;
                }
            }
        }
        _ => {
            eprintln!("seq: too many arguments");
            return 1;
        }
    };

    if step == 0 {
        eprintln!("seq: step cannot be zero");
        return 1;
    }

    if step > 0 && first > last {
        return 0;
    }
    if step < 0 && first < last {
        return 0;
    }

    let mut i = first;
    if step > 0 {
        while i <= last {
            println!("{}", i);
            i += step;
        }
    } else {
        while i >= last {
            println!("{}", i);
            i += step;
        }
    }

    0
}
