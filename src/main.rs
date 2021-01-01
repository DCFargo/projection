// Welcome to Projection, project management software from your terminal.
// This software is licensed under the Apache License, see LICENSE for details.
// Thanks to all the testers and contributors who made this project possible.
mod app;

use crate::structures::error::{handle, Error};
use app::actions::*;
use app::*;
use std::env;
use colored::Colorize;

#[macro_use]
extern crate savefile_derive;

fn main() {
    // Program variables
    let mut vars = structures::program_vars::init_vars();

    // Options / Actions parser
    let args: Vec<String> = env::args().collect();
    let mut args = parsers::arg_parser(args);
    vars.exe_path = args.actions[0].clone();
    args.actions.remove(0);

    // Set flags
    for option in args.options.iter() {
        match option.as_str() {
            "--verbose" | "-v" => vars.verbose = true,
            "--help" | "-h" => vars.help = true,
            _ => handle(Error::Syntax),
        }
    }

    // Run actions
    vars.alert_v("Projection v0.1.0 | (c) 2020 under Apache 2.0".white());
    if args.actions.len() == 0 {
        handle(Error::Syntax)
    }
    match {
        match args.actions[0].as_str() {
            "init" => init::init(&vars, args.actions),
            "goal" => goal::goal(&vars, args.actions),
            "kanban" => kanban::kanban(&vars, args.actions),
            _ => Err(Error::Syntax),
        }
    } {
        Ok(()) => (),
        Err(e) => handle(e),
    }

    println!("{}", "\n🟢 Complete!".bright_green())
}
