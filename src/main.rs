mod plugins;
mod prompt;

use std::{path::Path, process::Command};

use clap::{arg, Parser, Subcommand};
use owo_colors::AnsiColors as Color;
use plugins::git;
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

fn is_dirty() -> bool {
    let Ok(output) = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
    else {
        return false;
    };

    return output.status.success() && !output.stdout.is_empty();
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

    let mut parts = vec![
        Part::single(
            if exit_code == 0 {
                Color::BrightGreen
            } else {
                Color::BrightRed
            },
            "➜ ",
        ),
        Part::single(Color::BrightCyan, working_dir(pwd).unwrap()),
    ];

    parts.push(Part::Plugin(git));

    if is_dirty() {
        parts.push(Part::single(Color::BrightYellow, "✗"));
    }

    let prompt = ShellPrompt::new(parts);

    prompt.print();
}
