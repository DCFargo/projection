use std::process::exit;
use colored::Colorize;
const SYNTAX_MSG: &str = "🔴 Failed! | See proj(1) man page for proper usage. | ";

pub(crate) enum Error {
    Syntax,

    SyntaxGoal,
    SyntaxGoalAdd,
    SyntaxGoalRemove,
    SyntaxGoalEdit,
    SyntaxGoalComplete,

    SyntaxKanban,
    SyntaxKanbanAdd,
    SyntaxKanbanRemove,
    SyntaxKanbanMove,
    SyntaxKanbanEdit,
    SyntaxKanbanExtend,
    SyntaxKanbanSchedule,

    File,
    Directory,
}

pub(crate) fn handle(e_type: Error) {
    // Todo: write individual error messages
    let msg: &str = match e_type {
        Error::Syntax => "'proj [goal, init, kanban] ...'",

        Error::SyntaxGoal => "'proj goal [add, remove, edit, view] ...'",
        Error::SyntaxGoalAdd => "'proj goal add (ID) (DISP_NAME)'",
        Error::SyntaxGoalRemove => "'proj goal remove (ID/DISP_NAME)'",
        Error::SyntaxGoalEdit => "'proj goal edit (ID) (NEW_DISP_NAME)'",
        Error::SyntaxGoalComplete => "'proj goal complete (ID/DISP_NAME)'",

        Error::SyntaxKanban => {
            "'proj kanban [add, remove, move, edit, extend, view, schedule] ...'"
        }
        Error::SyntaxKanbanAdd => "'proj kanban add (ID) (DISP_NAME)'",
        Error::SyntaxKanbanRemove => "'proj kanban remove (ID/DISP_NAME)'",
        Error::SyntaxKanbanEdit => "'proj kanban edit (ID) (NEW_DISP_NAME)'",
        Error::SyntaxKanbanMove => "'proj kanban move (ID) (todo, progress, done)'",
        Error::SyntaxKanbanExtend => {
            "'proj kanban extend (ID) [hour (h), day (d), week (w)] (AMOUNT)"
        }
        Error::SyntaxKanbanSchedule => {
            "'proj kanban schedule (ID/DISP_NAME) (YEAR) (MONTH) (DAY) (opt: TIME)'"
        }

        Error::Directory => {
            "Error creating directory for init function, please check write permissions."
        }
        Error::File => {
            "Error saving/opening proj file, please run 'proj init' if you haven't already."
        }
    };
    let msg = format!("{}{}", SYNTAX_MSG, msg);
    println!("\n{}", msg.red());
    exit(1);
}
