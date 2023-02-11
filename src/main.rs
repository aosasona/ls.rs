use rustls::{parse_args, print_dir_content, Args};
use std::process;

fn main() {
    let args: Args;
    match parse_args() {
        Ok(result) => args = result,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }

    if let Err(err) = print_dir_content(args) {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}
