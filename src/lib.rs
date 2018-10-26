//! This crate provides functions to deal with slashes and backslashes in strings. That's right, I'm too lazy to always rewrite these same functions for other new projects.
//!
//! ## Examples
//!
//! To see examples, check out the documentation for each function.


/// Delete an ending slash in a string except for '/'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_end_slash("path/"));
/// ```
pub fn delete_end_slash(s: &str) -> &str {
    let length = s.len();

    if length > 1 && s.ends_with('/') {
        return &s[..length - 1];
    } else {
        return s;
    }
}

/// Delete an ending backslash in a string except for '\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_end_backslash(r"path\"));
/// ```
pub fn delete_end_backslash(s: &str) -> &str {
    let length = s.len();

    if length > 1 && s.ends_with('\\') {
        return &s[..length - 1];
    } else {
        return s;
    }
}

/// Delete an ending slash in a string except for '/'.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path/");
///
/// let s = slash_formatter::delete_end_slash_owned(s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_end_slash_owned(mut s: String) -> String {
    let length = s.len();

    if length > 1 && s.ends_with('/') {
        s.remove(length - 1);
    }

    return s;
}

/// Delete an ending backslash in a string except for '\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from(r"path\");
///
/// let s = slash_formatter::delete_end_backslash_owned(s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_end_backslash_owned(mut s: String) -> String {
    let length = s.len();

    if length > 1 && s.ends_with('\\') {
        s.remove(length - 1);
    }

    return s;
}

/// Delete an ending slash in a string except for '/'.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path/");
///
/// slash_formatter::delete_end_slash_mut(&mut s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_end_slash_mut(s: &mut String) {
    let length = s.len();

    if length > 1 && s.ends_with('/') {
        s.remove(length - 1);
    }
}

/// Delete an ending backslash in a string except for '\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from(r"path\");
///
/// slash_formatter::delete_end_backslash_mut(&mut s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_end_backslash_mut(s: &mut String) {
    let length = s.len();

    if length > 1 && s.ends_with('\\') {
        s.remove(length - 1);
    }
}

/// Delete an starting slash in a string except for '/'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_start_slash("/path"));
/// ```
pub fn delete_start_slash(s: &str) -> &str {
    let length = s.len();

    if length > 1 && s.starts_with('/') {
        return &s[1..];
    } else {
        return s;
    }
}

/// Delete an starting backslash in a string except for '\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_start_backslash(r"\path"));
/// ```
pub fn delete_start_backslash(s: &str) -> &str {
    let length = s.len();

    if length > 1 && s.starts_with('\\') {
        return &s[1..];
    } else {
        return s;
    }
}

/// Delete an starting slash in a string except for '/'.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("/path");
///
/// let s = slash_formatter::delete_start_slash_owned(s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_start_slash_owned(mut s: String) -> String {
    let length = s.len();

    if length > 1 && s.starts_with('/') {
        s.remove(0);
    }

    return s;
}

/// Delete an starting backslash in a string except for '\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from(r"\path");
///
/// let s = slash_formatter::delete_start_backslash_owned(s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_start_backslash_owned(mut s: String) -> String {
    let length = s.len();

    if length > 1 && s.starts_with('\\') {
        s.remove(0);
    }

    return s;
}

/// Delete an starting slash in a string except for '/'.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("/path");
///
/// slash_formatter::delete_start_slash_mut(&mut s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_start_slash_mut(s: &mut String) {
    let length = s.len();

    if length > 1 && s.starts_with('/') {
        s.remove(0);
    }
}

/// Delete an starting backslash in a string except for '\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from(r"\path");
///
/// slash_formatter::delete_start_backslash_mut(&mut s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_start_backslash_mut(s: &mut String) {
    let length = s.len();

    if length > 1 && s.starts_with('\\') {
        s.remove(0);
    }
}

/// Add an starting slash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("/path", slash_formatter::add_start_slash("path"));
/// ```
pub fn add_start_slash(s: &str) -> String {
    add_start_slash_owned(s.to_string())
}

/// Add an starting backslash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(r"\path", slash_formatter::add_start_backslash("path"));
/// ```
pub fn add_start_backslash(s: &str) -> String {
    add_start_backslash_owned(s.to_string())
}

/// Add an starting slash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path");
///
/// let s = slash_formatter::add_start_slash_owned(s);
///
/// assert_eq!("/path", s);
/// ```
pub fn add_start_slash_owned(mut s: String) -> String {
    if !s.starts_with('/') {
        s.insert(0, '/');
    }

    return s;
}

/// Add an starting backslash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path");
///
/// let s = slash_formatter::add_start_backslash_owned(s);
///
/// assert_eq!(r"\path", s);
/// ```
pub fn add_start_backslash_owned(mut s: String) -> String {
    if !s.starts_with('\\') {
        s.insert(0, '\\');
    }

    return s;
}

/// Add an starting slash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::add_start_slash_mut(&mut s);
///
/// assert_eq!("/path", s);
/// ```
pub fn add_start_slash_mut(s: &mut String) {
    if !s.starts_with('/') {
        s.insert(0, '/');
    }
}

/// Add an starting backslash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::add_start_backslash_mut(&mut s);
///
/// assert_eq!(r"\path", s);
/// ```
pub fn add_start_backslash_mut(s: &mut String) {
    if !s.starts_with('\\') {
        s.insert(0, '\\');
    }
}

/// Add an ending slash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path/", slash_formatter::add_end_slash("path"));
/// ```
pub fn add_end_slash(s: &str) -> String {
    add_end_slash_owned(s.to_string())
}

/// Add an ending backslash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(r"path\", slash_formatter::add_end_backslash("path"));
/// ```
pub fn add_end_backslash(s: &str) -> String {
    add_end_backslash_owned(s.to_string())
}

/// Add an ending slash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path");
///
/// let s = slash_formatter::add_end_slash_owned(s);
///
/// assert_eq!("path/", s);
/// ```
pub fn add_end_slash_owned(mut s: String) -> String {
    if !s.ends_with('/') {
        s.push('/');
    }

    return s;
}

/// Add an ending backslash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path");
///
/// let s = slash_formatter::add_end_backslash_owned(s);
///
/// assert_eq!(r"path\", s);
/// ```
pub fn add_end_backslash_owned(mut s: String) -> String {
    if !s.ends_with('\\') {
        s.push('\\');
    }

    return s;
}

/// Add an ending slash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::add_end_slash_mut(&mut s);
///
/// assert_eq!("path/", s);
/// ```
pub fn add_end_slash_mut(s: &mut String) {
    if !s.ends_with('/') {
        s.push('/');
    }
}

/// Add an ending backslash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::add_end_backslash_mut(&mut s);
///
/// assert_eq!(r"path\", s);
/// ```
pub fn add_end_backslash_mut(s: &mut String) {
    if !s.ends_with('\\') {
        s.push('\\');
    }
}

/// Concatenate two strings with a slash.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path/to", slash_formatter::concat_with_slash("path", "to/"));
/// ```
pub fn concat_with_slash(s1: &str, s2: &str) -> String {
    concat_with_slash_owned(s1.to_string(), s2)
}

/// Concatenate two strings with a backslash.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(r"path\to", slash_formatter::concat_with_backslash("path", r"to\"));
/// ```
pub fn concat_with_backslash(s1: &str, s2: &str) -> String {
    concat_with_backslash_owned(s1.to_string(), s2)
}

/// Concatenate two strings with a slash.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path");
///
/// let s = slash_formatter::concat_with_slash_owned(s, "to/");
///
/// assert_eq!("path/to", s);
/// ```
pub fn concat_with_slash_owned(s1: String, s2: &str) -> String {
    return delete_end_slash_owned(add_end_slash_owned(s1) + delete_start_slash(s2));
}

/// Concatenate two strings with a backslash.
///
/// ```
/// extern crate slash_formatter;
///
/// let s = String::from("path");
///
/// let s = slash_formatter::concat_with_backslash_owned(s, r"to\");
///
/// assert_eq!(r"path\to", s);
/// ```
pub fn concat_with_backslash_owned(s1: String, s2: &str) -> String {
    return delete_end_backslash_owned(add_end_backslash_owned(s1) + delete_start_backslash(s2));
}

/// Concatenate two strings with a slash.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::concat_with_slash_mut(&mut s, "to/");
///
/// assert_eq!("path/to", s);
/// ```
pub fn concat_with_slash_mut(s1: &mut String, s2: &str) {
    add_end_slash_mut(s1);
    s1.push_str(delete_start_slash(s2));
    delete_end_slash_mut(s1);
}

/// Concatenate two strings with a backslash.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::concat_with_backslash_mut(&mut s, r"to\");
///
/// assert_eq!(r"path\to", s);
/// ```
pub fn concat_with_backslash_mut(s1: &mut String, s2: &str) {
    add_end_backslash_mut(s1);
    s1.push_str(delete_start_backslash(s2));
    delete_end_backslash_mut(s1);
}