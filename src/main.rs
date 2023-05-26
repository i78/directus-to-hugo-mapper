use std::{fs};
use log::{info, warn, error};

pub mod cli;
pub mod parser;
pub mod mapper;
pub mod config;


fn main() {
    env_logger::init();

    info!("Starting Hugo mapper");

    let args = match cli::parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let mapping_config = match config::resolve_config(&args.contract) {
        Ok(v) => v,
        Err(e) => {
            error!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let contents = match fs::read_to_string(args.input_file.to_string()) {
        Ok(v) => v,
        Err(e) => {
            error!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let contents2 = match parser::parse_digitus_json(&contents) {
        Ok(v) => v,
        Err(e) => {
            error!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let results = mapper::export_to_hugo(mapping_config, &contents2);

    results.iter().for_each(|k| match k {
        Ok(slug) => info!("slug={:?} result=success", slug),
        Err(slug) => warn!("slug={:?} result=fail", slug),
    });
}
