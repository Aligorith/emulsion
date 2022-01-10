use std::cell::RefCell;
use std::collections::HashMap;
use std::process::Command;
use std::rc::Rc;

use crate::configuration::Configuration;
use gelatin::glium::glutin::event::ModifiersState;
use lazy_static::lazy_static;

pub static TOGGLE_FULLSCREEN_NAME: &str = "toggle_fullscreen";
pub static ESCAPE_NAME: &str = "escape";
pub static IMG_BROWSE_NAME: &str = "img_browse";
pub static IMG_NEXT_NAME: &str = "img_next";
pub static IMG_PREV_NAME: &str = "img_prev";
pub static IMG_ORIG_NAME: &str = "img_orig";
pub static IMG_FIT_NAME: &str = "img_fit";
pub static IMG_FIT_BEST_NAME: &str = "img_fit_best";
pub static IMG_FLIP_H_NAME: &str = "img_flip_h";
pub static IMG_FLIP_V_NAME: &str = "img_flip_v";
pub static IMG_ROT_CW_NAME: &str = "img_rot_cw";
pub static IMG_ROT_CCW_NAME: &str = "img_rot_ccw";
pub static IMG_DEL_NAME: &str = "img_del";
pub static IMG_COPY_NAME: &str = "img_copy";
pub static PAN_NAME: &str = "pan";
pub static PLAY_ANIM_NAME: &str = "play_anim";
pub static PLAY_PRESENT_NAME: &str = "play_present";
pub static PLAY_PRESENT_RND_NAME: &str = "play_present_rnd";
pub static TOGGLE_ANTIALIAS_NAME: &str = "toggle_antialias";
pub static SET_AUTOMATIC_ANTIALIAS_NAME: &str = "automatic_antialias";
pub static ZOOM_IN_NAME: &str = "zoom_in";
pub static ZOOM_OUT_NAME: &str = "zoom_out";
pub static PAN_LEFT_NAME: &str = "pan_left";
pub static PAN_RIGHT_NAME: &str = "pan_right";
pub static PAN_UP_NAME: &str = "pan_up";
pub static PAN_DOWN_NAME: &str = "pan_down";

lazy_static! {
	pub static ref DEFAULT_BINDINGS: HashMap<&'static str, Vec<&'static str>> = {
		let mut m = HashMap::new();
		m.insert(TOGGLE_FULLSCREEN_NAME, vec!["F11"]);
		m.insert(ESCAPE_NAME, vec!["Escape"]);
		m.insert(IMG_BROWSE_NAME, vec!["F1", "CmdCtrl+O"]);
		m.insert(IMG_NEXT_NAME, vec!["D", "Right", "PageDown"]);
		m.insert(IMG_PREV_NAME, vec!["A", "Left", "PageUp"]);
		m.insert(IMG_ORIG_NAME, vec!["Q", "1"]);
		m.insert(IMG_FIT_NAME, vec!["F"]);
		m.insert(IMG_FIT_BEST_NAME, vec!["E"]);
		m.insert(IMG_FLIP_H_NAME, vec!["H"]);
		m.insert(IMG_FLIP_V_NAME, vec!["V"]);
		m.insert(IMG_ROT_CCW_NAME, vec!["Shift+Right"]);
		m.insert(IMG_ROT_CW_NAME, vec!["Shift+Left"]);
		m.insert(IMG_DEL_NAME, vec!["Delete"]);
		m.insert(IMG_COPY_NAME, vec!["CmdCtrl+C"]);
		m.insert(ZOOM_IN_NAME, vec!["Add"]);
		m.insert(ZOOM_OUT_NAME, vec!["Subtract"]);
		m.insert(PAN_NAME, vec!["Space"]);
		m.insert(PLAY_ANIM_NAME, vec!["Alt+A", "Alt+V"]);
		m.insert(PLAY_PRESENT_NAME, vec!["P"]);
		m.insert(PLAY_PRESENT_RND_NAME, vec!["Alt+P"]);
		m.insert(TOGGLE_ANTIALIAS_NAME, vec!["S"]);
		m.insert(SET_AUTOMATIC_ANTIALIAS_NAME, vec!["Alt+S"]);
		m
	};
}

pub fn char_to_input_key(ch: char) -> String {
	let mut input_key = String::with_capacity(8);
	if ch == ' ' {
		input_key.push_str("space");
	} else if ch == '+' {
		input_key.push_str("add");
	} else if ch == '-' {
		input_key.push_str("subtract");
	} else {
		input_key.push(ch);
	}
	input_key
}

fn substitute_command_parameters(string: &str, var_map: &HashMap<&str, &str>) -> String {
	let mut result = String::from(string);
	for (&var_name, &substitute) in var_map.iter() {
		result = result.replace(var_name, substitute);
	}
	result
}

/// Execute all custom commands that were triggered by the input key and modifier set.
/// Note: img_path and folder_path both have to be str instead of Path because we
/// wouldn't be able to construct a command from them if they cannot be converted to
/// valid UTF-8.
pub fn execute_triggered_commands(
	config: Rc<RefCell<Configuration>>,
	input_key: &str,
	modifiers: ModifiersState,
	img_path: &str,
	folder_path: &str,
) {
	let config = config.borrow();
	if let Some(ref commands) = config.commands {
		let mut var_map = HashMap::with_capacity(2);
		var_map.insert("${img}", img_path);
		var_map.insert("${folder}", folder_path);
		for command in commands.iter() {
			if keys_triggered(&command.input, input_key, modifiers) {
				let mut cmd = Command::new(&command.program);
				if let Some(ref args) = command.args {
					cmd.args(args.iter().map(|arg| substitute_command_parameters(arg, &var_map)));
				}
				if let Some(ref envs) = command.envs {
					cmd.envs(
						envs.iter().map(|env_var| (env_var.name.as_str(), env_var.value.as_str())),
					);
				}
				if let Err(e) = cmd.status() {
					eprintln!("Error while executing the following user command. See the error below.\n{:?}\nError: {:?}", command, e);
				}
			}
		}
	}
}

pub fn keys_triggered<S: AsRef<str>>(
	keys: &[S],
	input_key: &str,
	modifiers: ModifiersState,
) -> bool {
	for key in keys {
		let complex_key = key.as_ref();
		let parts = complex_key.split('+').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>();
		if parts.is_empty() {
			continue;
		}
		let key = parts.last().unwrap();
		if input_key != *key {
			continue;
		}
		let mut has_alt = false;
		let mut has_ctrl = false;
		let mut has_logo = false;
		for mod_str in parts.iter().take(parts.len() - 1) {
			match mod_str.as_ref() {
				"alt" => has_alt = true,
				"ctrl" => has_ctrl = true,
				"logo" => has_logo = true,
				"cmdctrl" => {
					if cfg!(target_os = "macos") {
						has_logo = true;
					} else {
						has_ctrl = true;
					}
				}
				_ => (),
			}
		}
		if has_alt == modifiers.alt()
			&& has_ctrl == modifiers.ctrl()
			&& has_logo == modifiers.logo()
		{
			return true;
		}
	}
	false
}

pub fn action_triggered(
	config: &Rc<RefCell<Configuration>>,
	action_name: &str,
	input_key: &str,
	modifiers: ModifiersState,
) -> bool {
	let config = config.borrow();
	let bindings = config.bindings.as_ref();
	if let Some(Some(keys)) = bindings.map(|b| b.get(action_name)) {
		keys_triggered(keys.as_slice(), input_key, modifiers)
	} else {
		let empty = Vec::new();
		let keys = DEFAULT_BINDINGS.get(action_name).unwrap_or(&empty);
		keys_triggered(keys.as_slice(), input_key, modifiers)
	}
}
