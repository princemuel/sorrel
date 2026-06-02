use sorrel::{Action, Args, Error, run};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    match Args::parse()? {
        Action::Help => Args::usage(),
        Action::Run(args) => run(&args)?,
    }

    Ok(())
}
