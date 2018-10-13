//! This crate provides functions to deal with slashes in strings.
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