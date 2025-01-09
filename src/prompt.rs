use crate::{color::Color, plugins};

pub struct Segment {
    color: Color,
    text: String,
}

impl Segment {
    fn spacer() -> Self {
        Self {
            color: Color::Reset,
            text: " ".to_string(),
        }
    }

    pub fn new(color: Color, text: impl ToString) -> Self {
        Self {
            color,
            text: text.to_string(),
        }
    }

    pub fn printable(&self, shell: &str) -> String {
        if self.text.trim().is_empty() {
            String::from(" ")
        } else {
            format!(
                "{}{}{}",
                self.color.shell_aware_ansi(shell),
                self.text,
                Color::Reset.shell_aware_ansi(shell)
            )
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

    pub fn single(color: Color, text: impl ToString) -> Self {
        Self::Simple(vec![Segment::new(color, text)])
    }
}

pub struct ShellPrompt {
    parts: Vec<Part>,
}

impl ShellPrompt {
    pub fn print(self, shell: &str) {
        let segments: Vec<Segment> = self
            .parts
            .into_iter()
            .filter_map(|x| x.evaluate())
            .flatten()
            .collect();

        let output: String = segments.into_iter().map(|x| x.printable(shell)).collect();

        print!("{output}")
    }

    pub fn new(parts: Vec<Part>) -> Self {
        Self { parts }
    }
}
