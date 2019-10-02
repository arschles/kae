mod cli;
mod cmd;
use cmd::result::Res;
mod kubernetes;

fn main() -> Res {
    let app = cli::app();
    let matches = app.clone().get_matches();
    match cli::route(matches) {
        Ok(()) => {
            let res: Res = Ok(());
            res
        }
        Err(s) => {
            print!("{}!\n\n", s);
            let res = app.clone().print_long_help();
            let ret: Res = res
                .map(|_| ())
                .map_err(|_| String::from("Couldn't print help text"));
            ret
        }
    }
}
