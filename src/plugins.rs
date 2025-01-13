use std::process::Command;

use phf::phf_map;

use crate::{color::Color, prompt::Segment};

/// Defines the list of available plugins
pub const PLUGINS: phf::Map<&str, fn() -> Option<Output>> = phf_map! {
    "git" => git,
    "node" => node,
    "python" => python,
    "ruby" => ruby,
    "rust" => rust,
};

pub struct Output {
    label: (String, Color),
    value: (String, Color),
}

impl Output {
    pub fn evaluate(self) -> Vec<Segment> {
        vec![
            Segment::new(self.label.1, format!("{}:(", self.label.0)),
            Segment::new(self.value.1, self.value.0),
            Segment::new(self.label.1, ")"),
        ]
    }
}

fn simple(
    command: &mut Command,
    label: (String, Color),
    value: (impl Fn(String) -> Option<String>, Color),
) -> Option<Output> {
    let output = command.output().ok()?;
    if !output.status.success() || output.stdout.is_empty() {
        return None;
    }

    let output = String::from_utf8(output.stdout).ok()?.trim().to_string();

    return Some(Output {
        value: (value.0(output)?, value.1),
        label,
    });
}

pub fn git() -> Option<Output> {
    simple(
        Command::new("git").arg("branch").arg("--show-current"),
        (String::from("git"), Color::Blue),
        (|x| Some(x), Color::Red),
    )
}

pub fn node() -> Option<Output> {
    simple(
        Command::new("node").arg("--version"),
        (String::from("node"), Color::Blue),
        (|x| Some(x), Color::Green),
    )
}

pub fn python() -> Option<Output> {
    simple(
        Command::new("python3").arg("--version"),
        (String::from("python"), Color::Blue),
        (|x| Some(x.split(" ").last()?.to_string()), Color::Yellow),
    )
}

pub fn ruby() -> Option<Output> {
    simple(
        Command::new("ruby").arg("--version"),
        (String::from("ruby"), Color::Red),
        (|x| Some(x.split(" ").nth(1)?.to_string()), Color::Magenta),
    )
}

pub fn rust() -> Option<Output> {
    simple(
        Command::new("rustc").arg("--version"),
        (String::from("rustc"), Color::Red),
        (|x| Some(x.split(" ").nth(1)?.to_string()), Color::Yellow),
    )
}
