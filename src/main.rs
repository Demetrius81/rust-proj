use clap::Command;
use rust_proj::commands;

pub fn main() -> anyhow::Result<()> {
    let mut command = Command::new("Sample CLI App");
    command = commands::configure(command);

    let matches = command.get_matches();
    commands::handle(&matches)?;

    Ok(())
}
