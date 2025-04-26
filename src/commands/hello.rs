use clap::{Command, ArgMatches};

use crate::settings::Settings;

pub const COMMAND_NAME: &str = "hello";

pub fn configure() -> Command {
	Command::new(COMMAND_NAME).about("Hello world!")
}

// #[allow(unused)]
pub fn handle(_matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
	println!("Hello world!");

	Ok(())
}