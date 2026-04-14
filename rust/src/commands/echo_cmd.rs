pub fn run(args: &[String]) -> i32 {
    let mut newline = true;
    let mut args_iter = args.iter();

    if let Some(first) = args_iter.next() {
        if first == "-n" {
            newline = false;
        } else {
            print!("{}", first);
            for arg in args_iter {
                print!(" {}", arg);
            }
            if newline {
                println!();
            }
            return 0;
        }
    }

    let args_vec: Vec<&str> = args_iter.map(|s| s.as_str()).collect();
    for (i, arg) in args_vec.iter().enumerate() {
        print!("{}", arg);
        if i < args_vec.len() - 1 {
            print!(" ");
        }
    }

    if newline {
        println!();
    }

    0
}
