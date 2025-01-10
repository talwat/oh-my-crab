#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Color {
    Reset,
    Bold,
    Dim,
    Italic,
    URL,
    Blink,
    Blink2,
    Selected,
    Hidden,
    Strikethrough,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub const fn ansi(&self) -> &'static str {
        match self {
            Color::Reset => "\x1B[0m",
            Color::Bold => "\x1B[1m",
            Color::Dim => "\x1B[2m",
            Color::Italic => "\x1B[3m",
            Color::URL => "\x1B[4m",
            Color::Blink => "\x1B[5m",
            Color::Blink2 => "\x1B[6m",
            Color::Selected => "\x1B[7m",
            Color::Hidden => "\x1B[8m",
            Color::Strikethrough => "\x1B[9m",
            Color::Black => "\x1B[30m",
            Color::Red => "\x1B[31m",
            Color::Green => "\x1B[32m",
            Color::Yellow => "\x1B[33m",
            Color::Blue => "\x1B[34m",
            Color::Magenta => "\x1B[35m",
            Color::Cyan => "\x1B[36m",
            Color::White => "\x1B[37m",
            Color::BrightBlack => "\x1B[30;1m",
            Color::BrightRed => "\x1B[31;1m",
            Color::BrightGreen => "\x1B[32;1m",
            Color::BrightYellow => "\x1B[33;1m",
            Color::BrightBlue => "\x1B[34;1m",
            Color::BrightMagenta => "\x1B[35;1m",
            Color::BrightCyan => "\x1B[36;1m",
            Color::BrightWhite => "\x1B[37;1m",
        }
    }

    pub fn shell_aware_ansi(&self, shell: &str) -> String {
        match shell {
            "zsh" => format!("%{{{}%}}", self.ansi()),
            "bash" => format!("\x01{}\x02", self.ansi()),
            _ => self.ansi().to_owned(),
        }
    }
}
