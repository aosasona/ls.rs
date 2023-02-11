use std::{env, fs, io};

/*
* Oh dear, this is painful to look at but I couldn't find any other way to do this without a
* package like clap and I am definitely not even a 2/10 when it comes to rust lmao; first actual
* rust code/experiment?
*/

#[derive(Debug)]
pub struct Args {
    pub show_hidden: bool,
    pub path: String,
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

pub fn print_dir_content(args: Args) -> Result<(), io::Error> {
    for content in fs::read_dir(args.path)? {
        let content = content?;
        let file = content.path();

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

    Ok(())
}
