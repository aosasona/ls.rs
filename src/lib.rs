use std::env;

#[derive(Debug)]
pub struct Args {
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

pub fn parse_args() -> Result<Args, String> {
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
