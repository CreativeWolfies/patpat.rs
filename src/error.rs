use std::fmt;
use crate::Location;

pub enum CompLocation<'a> {
    Char(&'a str, usize, usize), // (contents, line, char)
    Line(&'a str, usize),
    LineSpan(&'a str, usize, usize), // (contents, fromLine, length)
}

pub struct CompInfo<'a> {
    msg: &'a str,
    location: CompLocation<'a>,
}

pub struct CompError<'a> {
    exit_code: i32,
    infos: Vec<CompInfo<'a>>,
}

impl<'a> CompInfo<'a> {
    pub fn new(msg: &'a str, location: CompLocation<'a>) -> Self {
        CompInfo {
            msg,
            location,
        }
    }
}

impl<'a> CompError<'a> {
    pub fn new(exit_code: i32, infos: Vec<CompInfo<'a>>) -> Self {
        CompError {
            exit_code,
            infos,
        }
    }

    pub fn from(exit_code: i32, info: CompInfo<'a>) -> Self {
        Self::new(exit_code, vec![info])
    }

    pub fn add_info(&mut self, info: CompInfo<'a>) {
        self.infos.push(info);
    }

    pub fn print_and_exit(self) {
        eprintln!("{}", &self);
        std::process::exit(self.exit_code);
    }
}

impl<'a> fmt::Display for CompError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.infos.iter();
        match iter.next() {
            Some(info) => {
                writeln!(f, "Compile error: {}", info.msg)?;
                match info.location {
                    CompLocation::Char(raw, line, ch) => {
                        writeln!(f, "┌── at line {}, char {}", line, ch)?;
                        writeln!(f, "│ {}", raw.lines().collect::<Vec<_>>()[line])?;
                        writeln!(f, "│ {}^", " ".repeat(ch))?;
                    },
                    CompLocation::Line(raw, line) => {
                        writeln!(f, "┌── at line {}", line)?;
                        writeln!(f, "│ {}", raw.lines().collect::<Vec<_>>()[line])?;
                        writeln!(f, "│")?;
                    },
                    CompLocation::LineSpan(raw, line, length) => {
                        writeln!(f, "┌── from line {} to line {}", line, line + length)?;
                        let lines = raw.lines().skip(line).take(length);
                        for current_line in lines {
                            writeln!(f, "│ {}", current_line)?;
                        }
                        writeln!(f, "│")?;
                    }
                }

                Ok(())
            },
            None => writeln!(f, "Unknown compile error!"),
        }
    }
}

impl<'a> From<Location<'a>> for CompLocation<'a> {
    fn from(loc: Location<'a>) -> CompLocation<'a> {
        CompLocation::Char(
            loc.src,
            loc.line,
            loc.ch
        )
    }
}

impl<'a> From<&'a Location<'a>> for CompLocation<'a> {
    fn from(loc: &'a Location<'a>) -> CompLocation<'a> {
        CompLocation::Char(
            loc.src.clone(),
            loc.line,
            loc.ch
        )
    }
}
