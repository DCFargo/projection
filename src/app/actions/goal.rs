use crate::app::structures::{error::Error, object_data::GoalObject, program_vars::ProgramVars};
use savefile::prelude::*;
use std::fs::{remove_file, read_dir};
use colored::Colorize;

pub(crate) fn goal(vars: &ProgramVars, mut sub_actions: Vec<String>) -> Result<(), Error> {
    // Syntax check
    sub_actions.remove(0);
    if sub_actions.len() == 0 {
        return Err(Error::SyntaxGoal);
    }
    return match sub_actions.remove(0).as_str() {
        "add" => add(sub_actions, vars),
        "remove" => remove(sub_actions, vars),
        "edit" => edit(sub_actions, vars),
        "complete" => complete(sub_actions, vars),
        "view" => view(sub_actions, vars),
        _ => Err(Error::SyntaxGoal),
    };
}

fn add(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 2 {
        return Err(Error::SyntaxGoalAdd);
    }
    let goal_obj = GoalObject {
        ref_id: args.remove(0),
        display_name: args.remove(0),
        complete: false,
    };
    vars.alert_v("🟡 Creating goal object file...".yellow());
    match save_file(
        format!("projection/goal/{}", goal_obj.ref_id).as_str(),
        0,
        &goal_obj,
    ) {
        Ok(()) => Ok(()),
        Err(_e) => Err(Error::File),
    }
}

fn remove(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::SyntaxGoalRemove);
    }
    let id = args.remove(0);
    vars.alert_v("🟡 Removing goal object file...".yellow());
    match remove_file(format!("projection/goal/{}", id).as_str()) {
        Ok(()) => Ok(()),
        Err(_e) => Err(Error::File),
    }
}

fn edit(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 2 {
        return Err(Error::SyntaxGoalEdit);
    }
    let id = args.remove(0);
    let new_disp_name = args.remove(0);
    vars.alert_v("🟡 Loading goal object file...".yellow());
    let mut obj: GoalObject = match load_file(format!("projection/goal/{}", id).as_str(), 0) {
        Ok(o) => o,
        Err(_e) => return Err(Error::File),
    };
    obj.display_name = new_disp_name;
    vars.alert_v("🟡 Saving new goal object file...".yellow());
    match save_file(format!("projection/goal/{}", id).as_str(), 0, &obj) {
        Ok(()) => Ok(()),
        Err(_e) => Err(Error::File),
    }
}

fn complete(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::SyntaxGoalComplete);
    }
    let id = args.remove(0);
    vars.alert_v("🟡 Loading goal object file...".yellow());
    let mut obj: GoalObject = match load_file(format!("projection/goal/{}", id).as_str(), 0) {
        Ok(o) => o,
        Err(_e) => return Err(Error::File),
    };
    obj.complete = true;
    vars.alert_v("🟡 Saving complete goal object file...".yellow());
    match save_file(format!("projection/goal/{}", id).as_str(), 0, &obj) {
        Ok(()) => Ok(()),
        Err(_e) => Err(Error::File),
    }
}

fn view(args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 0 {
        return Err(Error::SyntaxGoal);
    }
    println!("{}", "[Goal items]".bright_blue());
    for entry in match read_dir("projection/goal") {
        Ok(file) => file,
        Err(_e) => return Err(Error::File),
    } {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => return Err(Error::File)
        };
        let path = entry.path();
        let path = match path.as_path().to_str() {
            Some(p) => p,
            None => return Err(Error::File),
        };
        // Ugly 
        let obj: GoalObject = match load_file(format!("{}", path).as_str(), 0) {
            Ok(o) => o,
            Err(_e) => return Err(Error::File),
        };

        println!("{} {}: {}", match obj.complete {
            false => "🔴",
            true => "🟢",
        }, obj.ref_id.bright_blue(), obj.display_name)
    }
    Ok(())
}
