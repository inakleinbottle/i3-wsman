use crate::common::{polybar, this_command};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD};
use std::collections::HashMap;

lazy_static! {
	pub static ref CMD: String = "poke-poly".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(DEFAULT_CMD, exec as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!("{} {}", this_command(), CMD.as_str());
	println!("    Pokes polybar to update\n\r");
}

pub fn exec(_: Vec<String>) {
	polybar::update();
}
