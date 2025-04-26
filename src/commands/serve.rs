use clap::{Command, ArgMatches};

pub const COMMAND_NAME: &str = "serve";

pub fn configure() -> Command {
	Command::new(COMMAND_NAME).about("Start the HTTP server")
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
	println!("TBD: Start the HTTP server on port ???");

	Ok(())
}