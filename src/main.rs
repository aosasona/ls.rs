use rustls::{parse_args, Args};
use std::process;

// Oh dear, this is painful to look at but I couldn't find any other way to do this without a
// package like clap and I am definitely not even a 2/10 when it comes to rust lmao; first actual
// rust code/experiment?

fn main() {
    let args: Args;
    match parse_args() {
        Ok(result) => args = result,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }

    print!("{:?}", args);
}
