use clap::{Command, ArgMatches};

pub const COMMAND_NAME: &str = "hello";

pub fn configure() -> Command {
	Command::new(COMMAND_NAME).about("Hello world!")
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
	println!("Hello world!");

	Ok(())
}