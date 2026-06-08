mod database;
mod display;
mod input;
mod list;
mod readme;
mod search;

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
    Display { id: i64 },
    #[command(visible_alias = "-l")]
    List,
    #[command(visible_alias = "-e")]
    Edit { id: i64 },
    #[command(visible_alias = "-s")]
    Search { name: String },
}

fn main() {
    let path = "markdown.db";
    database::db_setup(path).expect("!Create");
    let args = Args::parse();
    match &args.command {
        Commands::Add => input::start(0, 0).expect(""),
        Commands::Display { id } => display::view(id.clone()).expect(""),
        Commands::List => list::display().expect(""),
        Commands::Edit { id } => input::start(1, id.clone()).expect(""),
        Commands::Search { name } => search::display(name.clone()).expect(""),
    }
}
