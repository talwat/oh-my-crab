mod color;
mod plugins;
mod prompt;

use color::Color;
use std::{env, path::Path, process::Command};

use clap::{arg, Parser, Subcommand};
use plugins::{git, PLUGINS};
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
        let path = Path::new(&pwd);
        let name = path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("/"));
        name.to_str()?.to_string()
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

    let mut parts = Vec::with_capacity(8);

    if env::var("OMCRAB_SHOW_HOST").is_ok_and(|x| {
        let x = x.trim().to_lowercase();
        x == "1" || x == "true"
    }) {
        // Fix for MacOS.
        let hostname = hostname.trim_end_matches(".local");

        parts.push(Part::single(
            Color::White,
            format!("({}@{})", user, hostname),
        ));
    }

    parts.push(Part::single(
        if exit_code == 0 {
            Color::Green
        } else {
            Color::Red
        },
        "➜ ",
    ));

    parts.push(Part::single(Color::Cyan, working_dir(pwd).unwrap()));

    if let Ok(plugins) = env::var("OMCRAB_PLUGINS") {
        for plugin in plugins.trim().to_lowercase().split(" ") {
            let Some(plugin) = PLUGINS.get(plugin) else {
                continue;
            };

            parts.push(Part::Plugin(*plugin));
        }
    } else {
        parts.push(Part::Plugin(git));
    }

    if is_dirty() {
        parts.push(Part::single(Color::Yellow, "✗"));
    }

    let prompt = ShellPrompt::new(parts);

    prompt.print(&shell);
}
