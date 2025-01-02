use owo_colors::AnsiColors;

use crate::prompt::Segment;

pub struct Output {
    label: (String, AnsiColors),
    value: (String, AnsiColors),
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
