/// Delete an ending backslash in a string except for '\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_end_backslash(r"path\"));
/// ```
pub fn delete_end_backslash<'a, S: ?Sized + AsRef<str> + 'a>(s: &'a S) -> &'a str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.ends_with('\\') {
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
/// let s = String::from(r"path\");
///
/// let s = slash_formatter::delete_end_backslash_owned(s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_end_backslash_owned<S: Into<String>>(s: S) -> String {
    let mut s = s.into();

    let length = s.len();

    if length > 1 && s.ends_with('\\') {
        s.remove(length - 1);
    }

    return s;
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

/// Delete an starting backslash in a string except for '\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_start_backslash(r"\path"));
/// ```
pub fn delete_start_backslash<'a, S: ?Sized + AsRef<str> + 'a>(s: &'a S) -> &'a str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.starts_with('\\') {
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
/// let s = String::from(r"\path");
///
/// let s = slash_formatter::delete_start_backslash_owned(s);
///
/// assert_eq!("path", s);
/// ```
pub fn delete_start_backslash_owned<S: Into<String>>(s: S) -> String {
    let mut s = s.into();

    let length = s.len();

    if length > 1 && s.starts_with('\\') {
        s.remove(0);
    }

    return s;
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

/// Add an starting backslash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(r"\path", slash_formatter::add_start_backslash("path"));
/// ```
pub fn add_start_backslash<S: AsRef<str>>(s: S) -> String {
    add_start_backslash_owned(s.as_ref())
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
pub fn add_start_backslash_owned<S: Into<String>>(s: S) -> String {
    let mut s = s.into();

    if !s.starts_with('\\') {
        s.insert(0, '\\');
    }

    return s;
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

/// Add an ending backslash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(r"path\", slash_formatter::add_end_backslash("path"));
/// ```
pub fn add_end_backslash<S: AsRef<str>>(s: S) -> String {
    add_end_backslash_owned(s.as_ref())
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
pub fn add_end_backslash_owned<S: Into<String>>(s: S) -> String {
    let mut s = s.into();

    if !s.ends_with('\\') {
        s.push('\\');
    }

    return s;
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

/// Concatenate two strings with a backslash.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(r"path\to", slash_formatter::concat_with_backslash("path", r"to\"));
/// ```
pub fn concat_with_backslash<S1: AsRef<str>, S2: AsRef<str>>(s1: S1, s2: S2) -> String {
    concat_with_backslash_owned(s1.as_ref(), s2)
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
pub fn concat_with_backslash_owned<S1: Into<String>, S2: AsRef<str>>(s1: S1, s2: S2) -> String {
    return delete_end_backslash_owned(add_end_backslash_owned(s1) + delete_start_backslash(s2.as_ref()));
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
pub fn concat_with_backslash_mut<S2: AsRef<str>>(s1: &mut String, s2: S2) {
    add_end_backslash_mut(s1);
    s1.push_str(delete_start_backslash(s2.as_ref()));
    delete_end_backslash_mut(s1);
}