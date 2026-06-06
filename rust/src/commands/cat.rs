/// cat: concatenate files and print to stdout.
use std::fs::File;
use std::io;  // io::copy uses Read/Write impls internally

pub fn run(args: &[String]) -> i32 {
    if args.is_empty() {
        let mut stdin = io::stdin().lock();
        let mut stdout = io::stdout().lock();
        if let Err(e) = io::copy(&mut stdin, &mut stdout) {
            eprintln!("cat: {}", e);
            return 1;
        }
        return 0;
    }

    let mut exit_code = 0;
    for filename in args {
        if filename == "-" {
            let mut stdin = io::stdin().lock();
            let mut stdout = io::stdout().lock();
            if let Err(e) = io::copy(&mut stdin, &mut stdout) {
                eprintln!("cat: {}", e);
                exit_code = 1;
            }
        } else {
            match File::open(filename) {
                Ok(mut file) => {
                    let mut stdout = io::stdout().lock();
                    if let Err(e) = io::copy(&mut file, &mut stdout) {
                        eprintln!("cat: {}: {}", filename, e);
                        exit_code = 1;
                    }
                }
                Err(e) => {
                    eprintln!("cat: {}: {}", filename, e);
                    exit_code = 1;
                }
            }
        }
    }

    exit_code
}
