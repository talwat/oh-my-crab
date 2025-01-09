use std::process::Command;

use crate::{color::Color, prompt::Segment};

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

pub fn git() -> Option<Output> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .ok()?;

    if !output.status.success() || output.stdout.is_empty() {
        return None;
    }

    return Some(Output {
        value: (
            String::from_utf8(output.stdout).ok()?.trim().to_string(),
            Color::Red,
        ),
        label: (String::from("git"), Color::Blue),
    });
}
