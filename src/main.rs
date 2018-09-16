#![allow(proc_macro_derive_resolution_fallback)]
extern crate rustyline;
extern crate env_logger;
extern crate rand;
extern crate colored;
#[macro_use] extern crate failure;
extern crate shellwords;
extern crate dirs;
extern crate publicsuffix;
extern crate chrootable_https;
extern crate url;
extern crate hlua_badtouch as hlua;
#[macro_use] extern crate log;
#[macro_use] extern crate nom;
#[macro_use] extern crate structopt;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

pub mod args;
pub mod cmd;
pub mod db;
pub mod errors;
pub mod engine;
pub mod migrations;
pub mod models;
pub mod paths;
pub mod schema;
pub mod shell;
pub mod runtime;
pub mod term;
pub mod worker;
pub mod psl;
pub mod utils;

use args::{Args, SubCommand};
use errors::*;
use structopt::StructOpt;


fn run() -> Result<()> {
    let args = Args::from_args();
    match args.subcommand {
        Some(SubCommand::Sandbox) => unimplemented!(),
        None => shell::run(&args),
    }
}

fn main() {
    env_logger::init();

    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        for cause in err.iter_chain().skip(1) {
            eprintln!("Because: {}", cause);
        }
        std::process::exit(1);
    }
}