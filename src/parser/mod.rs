use regex::Regex;

pub fn parse(raw: String) {

}

mod patpat {
    struct Boolean {
        state: bool
    }

    struct Symbol {
        name: String
    }

    struct Define {}
    struct Space {}
    struct Let {}
}

pub const MATCHER: [(&str, &str); 5] = [
    ("Boolean", "/^(true|false)/"),
    ("Let", "/^let/"),
    ("Symbol", "/^[a-z_][a-z_\\d]*/"),
    ("Define", "/^:/"),
    ("Space", "/^\\s+/")
];

// This should be enough to be able to parse `let is_toast: true`
