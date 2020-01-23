use std::fmt;

pub enum Location<'a> {
    Char(&'a str, usize, usize),
    Line(&'a str, usize),
    Multiple(Vec<&'a str>, usize, usize), // (contents, fromLine, length)
}

pub struct CompInfo<'a> {
    msg: &'a str,
    location: Location<'a>,
}

pub struct CompError<'a> {
    exit_code: i32,
    infos: Vec<CompInfo<'a>>,
}

impl<'a> CompInfo<'a> {
    pub fn new(msg: &'a str, location: Location<'a>) -> Self {
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
        println!("{}", &self);
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
                    Location::Char(raw, line, ch) => {
                        writeln!(f, "|")?;
                        writeln!(f, "| {}", raw.lines().collect::<Vec<_>>()[line])?;
                        writeln!(f, "| {}^", " ".repeat(ch - 1))?;
                    },
                    _ => {/* not implemented yet */}
                }

                Ok(())
            },
            None => writeln!(f, "Unknown compile error!"),
        }
    }
}
