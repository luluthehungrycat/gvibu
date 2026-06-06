pub fn run(args: &[String]) -> i32 {
    let mut newline = true;
    let mut start_idx = 0;

    if let Some(first) = args.first() {
        if first == "-n" {
            newline = false;
            start_idx = 1;
        }
    }

    for (i, arg) in args.iter().enumerate().skip(start_idx) {
        if i > start_idx {
            print!(" ");
        }
        print!("{}", arg);
    }

    if newline {
        println!();
    }

    0
}
