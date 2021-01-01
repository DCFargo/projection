use colored::ColoredString;

pub struct ProgramVars {
    pub(crate) debug: bool,
    pub(crate) verbose: bool,
    pub(crate) exe_path: String,
    pub(crate) help: bool,
}

impl ProgramVars {
    pub(crate) fn alert_v(&self, message: ColoredString) {
        if self.verbose | self.debug {
            println!("{}", message)
        }
    }
}

// Default values
pub fn init_vars() -> ProgramVars {
    ProgramVars {
        debug: false,
        verbose: false,
        exe_path: String::new(),
        help: false,
    }
}
