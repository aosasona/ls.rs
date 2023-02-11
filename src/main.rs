use rustls::{parse_args, Args};
use std::{fs, process};

fn main() {
    let args: Args;
    match parse_args() {
        Ok(result) => args = result,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }

    for content in fs::read_dir(args.path).unwrap() {
        let file = content.unwrap().path();

        /*
         * apparently it is also difficult to get terminal colors in rust without an external
         * package because for some reason, it simply prints out the CYAN and RESET strings unlike
         * the Golang package
         */
        let file_name = file.file_name().unwrap().to_str().unwrap();

        if file_name.starts_with(".") && !args.show_hidden {
            continue;
        }

        println!("{}", file_name)
    }
}
