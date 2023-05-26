#[derive(Debug)]
pub struct AppArgs {
    pub input_file: String,
    pub contract: String,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("nope");
        std::process::exit(0);
    }

    let args = AppArgs {
        input_file: pargs.value_from_str("--inputfile")?,
        contract: pargs.value_from_str("--contract")?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}
