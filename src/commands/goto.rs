extern crate i3_ipc;

use i3_ipc::{Connect, I3};

use crate::common::{
	constraint::{Constraint, Constraints},
	outputs, polybar, this_command, workspaces,
};

use crate::{CommandFn, Commands, DEFAULT_CMD, HELP_CMD, WILD_CMD};
use std::collections::HashMap;

const ON_LAST_CREATE: &str = "create";
const ON_LAST_NOOP: &str = "noop";

lazy_static! {
	pub static ref CMD: String = "goto".to_string();
	pub static ref SUBCMDS: Commands = {
		let mut cmds = HashMap::new();
		cmds.insert(WILD_CMD, exec as CommandFn);
		cmds.insert(DEFAULT_CMD, help as CommandFn);
		cmds.insert(HELP_CMD, help as CommandFn);
		cmds
	};
}

pub fn help(_: Vec<String>) {
	println!(
		"{} {} <nth> [{}|{}]",
		this_command(),
		CMD.as_str(),
		ON_LAST_CREATE,
		ON_LAST_NOOP
	);
	println!(
		"    Focuses on the <nth> workspace, where <nth> is the position of the workspace\n\r"
	);
	println!(
		"    {} {} <nth> {}",
		this_command(),
		CMD.as_str(),
		ON_LAST_CREATE
	);
	println!("        If workspace doesn't exist, creates a new workspace.\n\r");
	println!(
		"    {} {} <nth> {}",
		this_command(),
		CMD.as_str(),
		ON_LAST_NOOP
	);
	println!("        If workspace doesn't exist, do nothing.\n\r");
}

pub fn exec(mut args: Vec<String>) {
	let nth_try = args.remove(0).parse::<usize>();
	if nth_try.is_err() {
		help(vec![]);
		return;
	}

	let nth = nth_try.unwrap();
	if nth < 1 {
		help(vec![]);
		return;
	}

	let last_action: String = if args.len() == 0 {
		"".to_string()
	} else {
		match args[0].as_str() {
			ON_LAST_CREATE => args.remove(0),
			ON_LAST_NOOP => args.remove(0),
			_ => "".to_string(),
		}
	};

	let mut constraints = Constraints::new();
	constraints.add(Constraint::Output);
	constraints.add(Constraint::Group);
	constraints.add(Constraint::NoGroup);
	constraints.add(Constraint::AllowUrgent);
	constraints.output = outputs::focused();

	let workspaces = workspaces::get(constraints.clone(), false);

	if workspaces.len() < nth {
		match last_action.as_str() {
			ON_LAST_CREATE => {
				let last_ws = workspaces::last(constraints);
				let mut i3 = I3::connect().unwrap();
				let cmd = format!("workspace {}", last_ws.name);
				i3.run_command(cmd).ok();
				crate::commands::adjacent::exec(vec!["right".to_owned()]);
			}
			ON_LAST_NOOP => {}
			_ => {}
		};
	} else {
		let target_ws = workspaces.get(nth - 1).unwrap();
		let mut i3 = I3::connect().unwrap();
		let cmd = format!("workspace {}", target_ws.name);
		i3.run_command(cmd).ok();
	}

	polybar::update();
}
