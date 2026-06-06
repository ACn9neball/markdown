mod database;
mod display;
mod input;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about= None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(visible_alias = "-a")]
    Add,
    #[command(visible_alias = "-d")]
    Display { name: String },
    #[command(visible_alias = "-l")]
    List,
    #[command(visible_alias = "-e")]
    Edit { name: String },
}

fn main() {
    let path = "markdown.db";
    database::db_setup(path).expect("!Create");
    let args = Args::parse();
    match &args.command {
        Commands::Add => input::start().expect(""),
        Commands::Display { name } => {}
        Commands::List => display::display().expect(""),
        Commands::Edit { name } => {}
    }
}
