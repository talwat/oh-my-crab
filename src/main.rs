mod plugins;
mod prompt;

use std::{
    env::home_dir,
    path::{Path, PathBuf},
};

use clap::{arg, Parser, Subcommand};
use owo_colors::AnsiColors as Color;
use prompt::{Part, ShellPrompt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Prompt {
        #[arg(long)]
        shell: String,

        #[arg(long)]
        pwd: String,

        #[arg(long)]
        user: String,

        #[arg(long)]
        hostname: String,

        #[arg(long = "exitcode")]
        exit_code: i16,
    },
}

fn working_dir(pwd: String) -> Option<String> {
    let home = dirs::home_dir()?;
    let home = home.to_str()?;

    return Some(if pwd != home {
        Path::new(&pwd).file_name()?.to_str()?.to_string()
    } else {
        "~".to_string()
    });
}

fn main() {
    let cli = Cli::parse();

    let Commands::Prompt {
        shell,
        pwd,
        user,
        hostname,
        exit_code,
    } = cli.command;

    let parts = vec![
        Part::single(Color::Red, "➜ "),
        Part::single(Color::Cyan, working_dir(pwd).unwrap()),
    ];

    let prompt = ShellPrompt::new(parts);

    prompt.print();
}
