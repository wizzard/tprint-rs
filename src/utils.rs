#[cfg(feature = "unicode")]
use unicode_width::UnicodeWidthStr;

pub fn get_string_width(s: &str) -> usize {
    #[cfg(feature = "unicode")]
    return UnicodeWidthStr::width(s);
    #[cfg(not(feature = "unicode"))]
    s.len()
}