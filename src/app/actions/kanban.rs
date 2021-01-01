use crate::app::structures::{
    error::Error,
    object_data::{KanbanObject, KanbanStatus},
    program_vars::ProgramVars,
    unit_time::UnitTime,
};
use savefile::prelude::*;
use std::fs::{remove_file, read_dir};
use colored::Colorize;

pub(crate) fn kanban(vars: &ProgramVars, mut sub_actions: Vec<String>) -> Result<(), Error> {
    // Syntax check
    sub_actions.remove(0);
    if sub_actions.len() == 0 {
        return Err(Error::SyntaxKanban);
    }
    match sub_actions.remove(0).as_str() {
        "add" => add(sub_actions, vars),
        "remove" => remove(sub_actions, vars),
        "move" => move_unit(sub_actions, vars),
        "edit" => edit(sub_actions, vars),
        "extend" => extend(sub_actions, vars),
        "view" => view(sub_actions, vars),
        "schedule" => schedule(sub_actions, vars),
        _ => Err(Error::SyntaxKanban),
    }
}

fn add(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 2 {
        return Err(Error::SyntaxKanbanAdd);
    }
    let kanban_obj = KanbanObject {
        ref_id: args.remove(0),
        display_name: args.remove(0),
        status: KanbanStatus::Todo,
        scheduled: false,
        year: 0,
        month: 0,
        day: 0,
        hour: 0,
    };
    vars.alert_v("🟡 Creating kanban object file...".yellow());
    match save_file(
        format!("projection/kanban/{}", kanban_obj.ref_id).as_str(),
        0,
        &kanban_obj,
    ) {
        Ok(()) => Ok(()),
        Err(_e) => Err(Error::File),
    }
}

fn remove(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::SyntaxKanbanRemove);
    }
    let id = args.remove(0);
    vars.alert_v("🟡 Removing kanban object file...".yellow());
    match remove_file(format!("projection/kanban/{}", id).as_str()) {
        Ok(()) => Ok(()),
        Err(_e) => Err(Error::File),
    }
}

fn move_unit(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 2 {
        return Err(Error::SyntaxKanbanMove);
    }
    let id = args.remove(0);
    let designation = match args.remove(0).as_str() {
        "todo" | "t" => KanbanStatus::Todo,
        "progress" | "p" => KanbanStatus::Progress,
        "done" | "d" => KanbanStatus::Done,
        _ => return Err(Error::SyntaxKanbanMove),
    };
    vars.alert_v("🟡 Loading kanban object file...".yellow());
    let mut obj: KanbanObject = match load_file(format!("projection/kanban/{}", id).as_str(), 0) {
        Ok(o) => o,
        Err(_e) => return Err(Error::File),
    };
    obj.status = designation;
    vars.alert_v("🟡 Saving modified kanban file...".yellow());
    match save_file(format!("projection/kanban/{}", id).as_str(), 0, &obj) {
        Ok(()) => Ok(()),
        Err(_e) => Err(Error::File),
    }
}

fn edit(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 2 {
        return Err(Error::SyntaxKanbanEdit);
    }
    let id = args.remove(0);
    let new_disp_name = args.remove(0);
    vars.alert_v("🟡 Loading kanban object file...".yellow());
    let mut obj: KanbanObject = match load_file(format!("projection/kanban/{}", id).as_str(), 0) {
        Ok(o) => o,
        Err(_e) => return Err(Error::File),
    };
    obj.display_name = new_disp_name;
    vars.alert_v("🟡 Saving modified kanban file...".yellow());
    match save_file(format!("projection/kanban/{}", id).as_str(), 0, &obj) {
        Ok(()) => Ok(()),
        Err(_e) => Err(Error::File),
    }
}

fn extend(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 3 {
        return Err(Error::SyntaxKanbanExtend);
    }
    let id = args.remove(0);
    let unit = match args.remove(0).as_str() {
        "hour" | "h" => UnitTime::Hour,
        "day" | "d" => UnitTime::Day,
        "week" | "w" => UnitTime::Week,
        _ => return Err(Error::SyntaxKanbanExtend),
    };
    vars.alert_v("🟡 Parsing values for edit...".yellow());
    let value = match args.remove(0).parse::<u32>() {
        Ok(num) => num,
        Err(_e) => return Err(Error::SyntaxKanbanExtend),
    };
    vars.alert_v("🟡 Loading kanban object file...".yellow());
    let mut obj: KanbanObject = match load_file(format!("projection/kanban/{}", id).as_str(), 0) {
        Ok(o) => o,
        Err(_e) => return Err(Error::File),
    };
    match unit {
        UnitTime::Hour => {
            obj.hour = obj.hour + value as u8;
        },
        UnitTime::Day => {
            obj.day = obj.day + value;
        },
        UnitTime::Week => {
            obj.day = obj.day + (value * 7);
        },
    }
    // Ugly chained ifs but afaik this is best practice
    vars.alert_v("🟡 Checking for invalid values...".yellow());
    if obj.hour > 24 {
        obj.day = obj.day + (obj.hour / 24) as u32;
        obj.hour = obj.hour % 24;
    }
    let mut day_num = match obj.month {
        1 | 3 | 5 | 7 | 8 | 10 => 31,
        4 | 6 | 9 | 11 | 12 => 30,
        2 => 28,
        _ => return Err(Error::SyntaxKanbanExtend)
    };
    if day_num == 28 && obj.year % 4 == 0 {
        day_num = 29;
    }
    if obj.day > day_num {
        obj.month = obj.month + (obj.day / day_num) as u8;
        obj.day = obj.day % day_num;
    }
    if obj.month > 12 {
        obj.year = obj.year + (obj.month / 12) as u32;
        obj.month = obj.month % 12;
    }
    vars.alert_v("🟡 Saving modified kanban file...".yellow());
    match save_file(format!("projection/kanban/{}", id).as_str(), 0, &obj) {
        Ok(()) => Ok(()),
        Err(_e) => return Err(Error::File)
    }
}

// If this was perfect id be using blackbox funcs
// But its not worth Ok(String)
fn view(args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() != 0 {
        return Err(Error::SyntaxKanban);
    }
    println!("{}", "[Kanban items]".bright_blue());
    for entry in match read_dir("projection/kanban") {
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
        vars.alert_v("🟡 Loading kanban object file...".yellow());
        let obj: KanbanObject = match load_file(format!("{}", path).as_str(), 0) {
            Ok(o) => o,
            Err(_e) => return Err(Error::File),
        };
        if obj.year != 0 {
            println!("{} {}: {}, {} {} {} {}", match obj.status {
                KanbanStatus::Todo => "🔴",
                KanbanStatus::Progress => "🟡",
                KanbanStatus::Done => "🟢",
            }, obj.ref_id.bright_blue(), obj.display_name, obj.year, obj.month, obj.day, obj.hour);
        } else {
            println!("{} {}: {}", match obj.status {
                KanbanStatus::Todo => "🔴",
                KanbanStatus::Progress => "🟡",
                KanbanStatus::Done => "🟢",
            }, obj.ref_id.bright_blue(), obj.display_name)
        }
    }
    Ok(())
}

fn schedule(mut args: Vec<String>, vars: &ProgramVars) -> Result<(), Error> {
    if args.len() == 0 {
        return Err(Error::SyntaxKanbanSchedule);
    }
    let id = args.remove(0);
    let time: (String, String, String, String);
    match args.len() {
        3 => {
            time = (
                args.remove(0),
                args.remove(0),
                args.remove(0),
                String::from("0"),
            )
        }
        4 => {
            time = (
                args.remove(0),
                args.remove(0),
                args.remove(0),
                args.remove(0),
            )
        }
        _ => return Err(Error::SyntaxKanbanSchedule),
    }
    vars.alert_v("🟡 Loading kanban object file...".yellow());
    let mut obj: KanbanObject = match load_file(format!("projection/kanban/{}", id).as_str(), 0) {
        Ok(o) => o,
        Err(_e) => return Err(Error::File)
    };
    vars.alert_v("🟡 Parsing arguments for schedule...".yellow());
    obj.year = match (time.0).parse::<u32>() {
        Ok(num) => num,
        Err(_e) => return Err(Error::SyntaxKanbanSchedule),
    };
    obj.month = match (time.1).parse::<u8>() {
        Ok(num) => num,
        Err(_e) => return Err(Error::SyntaxKanbanSchedule),
    };
    obj.day = match (time.2).parse::<u32>() {
        Ok(num) => num,
        Err(_e) => return Err(Error::SyntaxKanbanSchedule),
    };
    obj.hour = match (time.3).parse::<u8>() {
        Ok(num) => num,
        Err(_e) => return Err(Error::SyntaxKanbanSchedule),
    };

    vars.alert_v("🟡 Saving modified kanban file...".yellow());
    match save_file(format!("projection/kanban/{}", id).as_str(), 0, &obj) {
        Ok(()) => Ok(()),
        Err(_e) => Err(Error::File),
    }
}