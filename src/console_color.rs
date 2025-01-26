pub enum ConsoleColor {
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

impl ConsoleColor {
    pub fn to_fg_ansi_code(&self) -> &str {
        match self {
            ConsoleColor::Black => "\x1b[30m",
            ConsoleColor::Red => "\x1b[31m",
            ConsoleColor::Green => "\x1b[32m",
            ConsoleColor::Yellow => "\x1b[33m",
            ConsoleColor::Blue => "\x1b[34m",
            ConsoleColor::Magenta => "\x1b[35m",
            ConsoleColor::Cyan => "\x1b[36m",
            ConsoleColor::White => "\x1b[37m",
            ConsoleColor::BrightBlack => "\x1b[90m",
            ConsoleColor::BrightRed => "\x1b[91m",
            ConsoleColor::BrightGreen => "\x1b[92m",
            ConsoleColor::BrightYellow => "\x1b[93m",
            ConsoleColor::BrightBlue => "\x1b[94m",
            ConsoleColor::BrightMagenta => "\x1b[95m",
            ConsoleColor::BrightCyan => "\x1b[96m",
            ConsoleColor::BrightWhite => "\x1b[97m",
        }
    }

    pub fn to_bg_ansi_code(&self) -> &str {
        match self {
            ConsoleColor::Black => "\x1b[40m",
            ConsoleColor::Red => "\x1b[41m",
            ConsoleColor::Green => "\x1b[42m",
            ConsoleColor::Yellow => "\x1b[43m",
            ConsoleColor::Blue => "\x1b[44m",
            ConsoleColor::Magenta => "\x1b[45m",
            ConsoleColor::Cyan => "\x1b[46m",
            ConsoleColor::White => "\x1b[47m",
            ConsoleColor::BrightBlack => "\x1b[100m",
            ConsoleColor::BrightRed => "\x1b[101m",
            ConsoleColor::BrightGreen => "\x1b[102m",
            ConsoleColor::BrightYellow => "\x1b[103m",
            ConsoleColor::BrightBlue => "\x1b[104m",
            ConsoleColor::BrightMagenta => "\x1b[105m",
            ConsoleColor::BrightCyan => "\x1b[106m",
            ConsoleColor::BrightWhite => "\x1b[107m",
        }
    }
}