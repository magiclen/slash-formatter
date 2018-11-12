#[cfg(windows)]
const FILE_SEPARATOR: char = '\\';

#[cfg(not(windows))]
const FILE_SEPARATOR: char = '/';

/// Delete an ending FILE_SEPARATOR in a string except for the FILE_SEPARATOR.
///
/// ```
/// extern crate slash_formatter;
///
/// if cfg!(windows) {
///     assert_eq!("path", slash_formatter::delete_end_file_separator(r"path\"));
/// } else {
///     assert_eq!("path", slash_formatter::delete_end_file_separator("path/"));
/// }
/// ```
pub fn delete_end_file_separator(s: &str) -> &str {
    let length = s.len();

    if length > 1 && s.ends_with(FILE_SEPARATOR) {
        return &s[..length - 1];
    } else {
        return s;
    }
}

/// Delete an ending FILE_SEPARATOR in a string except for the FILE_SEPARATOR.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = if cfg!(windows) {
///     String::from(r"path\")
/// } else {
///     String::from("path/")
/// };
///
/// let s = slash_formatter::delete_end_file_separator_owned(s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_end_file_separator_owned(mut s: String) -> String {
    let length = s.len();

    if length > 1 && s.ends_with(FILE_SEPARATOR) {
        s.remove(length - 1);
    }

    return s;
}

/// Delete an ending FILE_SEPARATOR in a string except for the FILE_SEPARATOR.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = if cfg!(windows) {
///     String::from(r"path\")
/// } else {
///     String::from("path/")
/// };
///
/// slash_formatter::delete_end_file_separator_mut(&mut s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_end_file_separator_mut(s: &mut String) {
    let length = s.len();

    if length > 1 && s.ends_with(FILE_SEPARATOR) {
        s.remove(length - 1);
    }
}

/// Delete an starting FILE_SEPARATOR in a string except for the FILE_SEPARATOR.
///
/// ```
/// extern crate slash_formatter;
///
/// if cfg!(windows) {
///     assert_eq!("path", slash_formatter::delete_start_file_separator(r"\path"));
/// } else {
///     assert_eq!("path", slash_formatter::delete_start_file_separator("/path"));
/// }
/// ```
pub fn delete_start_file_separator(s: &str) -> &str {
    let length = s.len();

    if length > 1 && s.starts_with(FILE_SEPARATOR) {
        return &s[1..];
    } else {
        return s;
    }
}

/// Delete an starting FILE_SEPARATOR in a string except for the FILE_SEPARATOR.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = if cfg!(windows) {
///     String::from(r"\path")
/// } else {
///     String::from("/path")
/// };
///
/// let s = slash_formatter::delete_start_file_separator_owned(s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_start_file_separator_owned(mut s: String) -> String {
    let length = s.len();

    if length > 1 && s.starts_with(FILE_SEPARATOR) {
        s.remove(0);
    }

    return s;
}

/// Delete an starting FILE_SEPARATOR in a string except for the FILE_SEPARATOR.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = if cfg!(windows) {
///     String::from(r"\path")
/// } else {
///     String::from("/path")
/// };
///
/// slash_formatter::delete_start_file_separator_mut(&mut s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_start_file_separator_mut(s: &mut String) {
    let length = s.len();

    if length > 1 && s.starts_with(FILE_SEPARATOR) {
        s.remove(0);
    }
}

/// Add an starting FILE_SEPARATOR into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// if cfg!(windows) {
///     assert_eq!(r"\path", slash_formatter::add_start_file_separator("path"));
/// } else {
///     assert_eq!("/path", slash_formatter::add_start_file_separator("path"));
/// }
/// ```
pub fn add_start_file_separator(s: &str) -> String {
    add_start_file_separator_owned(s.to_string())
}

/// Add an starting FILE_SEPARATOR into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path");
///
/// let s = slash_formatter::add_start_file_separator_owned(s);
///
/// if cfg!(windows) {
///     assert_eq!(r"\path", s);
/// } else {
///     assert_eq!("/path", s);
/// }
/// ```
pub fn add_start_file_separator_owned(mut s: String) -> String {
    if !s.starts_with(FILE_SEPARATOR) {
        s.insert(0, FILE_SEPARATOR);
    }

    return s;
}

/// Add an starting FILE_SEPARATOR into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::add_start_file_separator_mut(&mut s);
///
/// if cfg!(windows) {
///     assert_eq!(r"\path", s);
/// } else {
///     assert_eq!("/path", s);
/// }
/// ```
pub fn add_start_file_separator_mut(s: &mut String) {
    if !s.starts_with(FILE_SEPARATOR) {
        s.insert(0, FILE_SEPARATOR);
    }
}

/// Add an ending FILE_SEPARATOR into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// if cfg!(windows) {
///     assert_eq!(r"path\", slash_formatter::add_end_file_separator("path"));
/// } else {
///     assert_eq!("path/", slash_formatter::add_end_file_separator("path"));
/// }
/// ```
pub fn add_end_file_separator(s: &str) -> String {
    add_end_file_separator_owned(s.to_string())
}

/// Add an ending FILE_SEPARATOR into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path");
///
/// let s = slash_formatter::add_end_file_separator_owned(s);
///
/// if cfg!(windows) {
///     assert_eq!(r"path\", s);
/// } else {
///     assert_eq!("path/", s);
/// }
/// ```
pub fn add_end_file_separator_owned(mut s: String) -> String {
    if !s.ends_with(FILE_SEPARATOR) {
        s.push(FILE_SEPARATOR);
    }

    return s;
}

/// Add an ending FILE_SEPARATOR into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::add_end_file_separator_mut(&mut s);
///
/// if cfg!(windows) {
///     assert_eq!(r"path\", s);
/// } else {
///     assert_eq!("path/", s);
/// }
/// ```
pub fn add_end_file_separator_mut(s: &mut String) {
    if !s.ends_with(FILE_SEPARATOR) {
        s.push(FILE_SEPARATOR);
    }
}

/// Concatenate two strings with a FILE_SEPARATOR.
///
/// ```
/// extern crate slash_formatter;
///
/// if cfg!(windows) {
///     assert_eq!(r"path\to", slash_formatter::concat_with_file_separator("path", r"to\"));
/// } else {
///     assert_eq!("path/to", slash_formatter::concat_with_file_separator("path", "to/"));
/// }
/// ```
pub fn concat_with_file_separator(s1: &str, s2: &str) -> String {
    concat_with_file_separator_owned(s1.to_string(), s2)
}

/// Concatenate two strings with a FILE_SEPARATOR.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path");
///
/// if cfg!(windows) {
///     let s = slash_formatter::concat_with_file_separator_owned(s, r"to\");
///
///     assert_eq!(r"path\to", s);
/// } else {
///     let s = slash_formatter::concat_with_file_separator_owned(s, "to/");
///
///     assert_eq!("path/to", s);
/// }
/// ```
pub fn concat_with_file_separator_owned(s1: String, s2: &str) -> String {
    return delete_end_file_separator_owned(add_end_file_separator_owned(s1) + delete_start_file_separator(s2));
}

/// Concatenate two strings with a FILE_SEPARATOR.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// if cfg!(windows) {
///     slash_formatter::concat_with_file_separator_mut(&mut s, r"to\");
///
///     assert_eq!(r"path\to", s);
/// } else {
///     slash_formatter::concat_with_file_separator_mut(&mut s, "to/");
///
///     assert_eq!("path/to", s);
/// }
/// ```
pub fn concat_with_file_separator_mut(s1: &mut String, s2: &str) {
    add_end_file_separator_mut(s1);
    s1.push_str(delete_start_file_separator(s2));
    delete_end_file_separator_mut(s1);
}