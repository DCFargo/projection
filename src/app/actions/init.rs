use crate::app::structures::{error::Error, program_vars::ProgramVars};
use std::fs::create_dir;
use colored::Colorize;

pub(crate) fn init(vars: &ProgramVars, sub_actions: Vec<String>) -> Result<(), Error> {
    // Syntax check
    if sub_actions.len() != 1 {
        return Err(Error::Syntax);
    }
    vars.alert_v("Initializing Projection...".yellow());
    match create_dir("projection") {
        Ok(()) => (),
        Err(_e) => return Err(Error::Directory),
    }
    match create_dir("projection/kanban") {
        Ok(()) => (),
        Err(_e) => return Err(Error::Directory),
    }
    match create_dir("projection/goal") {
        Ok(()) => (),
        Err(_e) => return Err(Error::Directory),
    }

    vars.alert_v("Initialization complete!".yellow());
    Ok(())
}
