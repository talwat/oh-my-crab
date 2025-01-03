use owo_colors::{AnsiColors, OwoColorize};

use crate::plugins;

pub struct Segment {
    color: AnsiColors,
    text: String,
}

impl Segment {
    fn spacer() -> Self {
        Self {
            color: AnsiColors::Default,
            text: " ".to_string(),
        }
    }

    pub fn new(color: AnsiColors, text: impl ToString) -> Self {
        Self {
            color,
            text: text.to_string(),
        }
    }
}

pub enum Part {
    Plugin(fn() -> Option<plugins::Output>),
    Simple(Vec<Segment>),
}

impl Part {
    fn evaluate(self) -> Option<Vec<Segment>> {
        let mut segments = match self {
            Self::Plugin(x) => x()?.evaluate(),
            Self::Simple(segment) => segment,
        };

        segments.push(Segment::spacer());

        Some(segments)
    }

    pub fn single(color: AnsiColors, text: impl ToString) -> Self {
        Self::Simple(vec![Segment::new(color, text)])
    }
}

pub struct ShellPrompt {
    parts: Vec<Part>,
}

impl ShellPrompt {
    pub fn print(self) {
        let segments: Vec<Segment> = self
            .parts
            .into_iter()
            .filter_map(|x| x.evaluate())
            .flatten()
            .collect();

        let output: String = segments
            .into_iter()
            .map(|x| x.text.color(x.color).to_string())
            .collect();

        print!("{output}\n")
    }

    pub fn new(parts: Vec<Part>) -> Self {
        Self { parts }
    }
}
