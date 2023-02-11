use std::{env, process};

#[derive(Debug)]
struct Args {
    show_hidden: bool,
    path: String,
}

impl Args {
    fn new() -> Self {
        Args {
            show_hidden: false,
            path: String::from("."),
        }
    }
}

// Oh dear, this is painful to look at but I couldn't find any other way to do this without a
// package like clap and I am definitely not even a 2/10 when it comes to rust lmao; first actual
// rust code/experiment?

fn main() {
    let args: Args;
    match get_ls_args() {
        Ok(result) => args = result,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }

    print!("{:?}", args);
}
fn get_ls_args() -> Result<Args, String> {
    let raw_args: Vec<String> = env::args().collect();
    let mut ls_args = Args::new();
    let raw_args_length = raw_args.len();

    if raw_args_length == 3 {
        ls_args.path = if !raw_args[2].starts_with("-") {
            raw_args[2].clone()
        } else {
            return Err(format!("invalid argument {} provided", raw_args[2]));
        };

        ls_args.show_hidden = if raw_args[1] == "-a" {
            true
        } else {
            return Err(format!("invalid flag {} provided", raw_args[1]));
        };
    } else if raw_args_length == 2 {
        if !raw_args[1].starts_with("-") {
            ls_args.path = raw_args[1].clone()
        } else if raw_args[1] == "-a" {
            ls_args.show_hidden = true;
        } else {
            return Err(format!("invalid argument {} provided", raw_args[1]));
        };
    } else if raw_args_length > 3 {
        return Err(String::from("invalid arguments provided!"));
    };

    Ok(ls_args)
}
