use dotenv::dotenv;
use fi_cli::cli::Cli;
use fi_cli::run;
use std::env;
use std::error::Error;
use structopt::StructOpt;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading database url");

    let args = Cli::from_args();

    handle_error(run(&database_url.as_str(), args));
}

fn handle_error<T>(res: Result<T, Box<dyn Error>>) -> T {
    match res {
        Ok(x) => x,
        Err(e) => print_error_and_exit(&*e),
    }
}

fn print_error_and_exit(err: &dyn Error) -> ! {
    use std::process::exit;
    eprintln!("An unexpected error occurred: {}", err);
    exit(1);
}
