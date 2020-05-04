//! utils for test

/// remove whitespace, \n, \r, \t
pub fn unindent(s: &str) -> String {
    s.replace(" ", "")
        .replace('\n', "")
        .replace('\r', "")
        .replace('\t', "")
}
