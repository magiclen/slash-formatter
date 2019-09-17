use alloc::string::String;

/// Delete an ending slash in a string except for '/'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_end_slash("path/"));
/// ```
pub fn delete_end_slash<'a, S: ?Sized + AsRef<str> + 'a>(s: &'a S) -> &'a str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.ends_with('/') {
        &s[..length - 1]
    } else {
        s
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
pub fn delete_end_slash_owned<S: Into<String>>(s: S) -> String {
    let mut s = s.into();

    let length = s.len();

    if length > 1 && s.ends_with('/') {
        s.remove(length - 1);
    }

    s
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

/// Delete a starting slash in a string except for '/'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_start_slash("/path"));
/// ```
pub fn delete_start_slash<'a, S: ?Sized + AsRef<str> + 'a>(s: &'a S) -> &'a str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.starts_with('/') {
        &s[1..]
    } else {
        s
    }
}

/// Delete a starting slash in a string except for '/'.
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
pub fn delete_start_slash_owned<S: Into<String>>(s: S) -> String {
    let mut s = s.into();

    let length = s.len();

    if length > 1 && s.starts_with('/') {
        s.remove(0);
    }

    s
}

/// Delete a starting slash in a string except for '/'.
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

/// Add a starting slash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("/path", slash_formatter::add_start_slash("path"));
/// ```
pub fn add_start_slash<S: AsRef<str>>(s: S) -> String {
    add_start_slash_owned(s.as_ref())
}

/// Add a starting slash into a string.
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
pub fn add_start_slash_owned<S: Into<String>>(s: S) -> String {
    let mut s = s.into();

    if !s.starts_with('/') {
        s.insert(0, '/');
    }

    s
}

/// Add a starting slash into a string.
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

/// Add an ending slash into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path/", slash_formatter::add_end_slash("path"));
/// ```
pub fn add_end_slash<S: AsRef<str>>(s: S) -> String {
    add_end_slash_owned(s.as_ref())
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
pub fn add_end_slash_owned<S: Into<String>>(s: S) -> String {
    let mut s = s.into();

    if !s.ends_with('/') {
        s.push('/');
    }

    s
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

/// Concatenate two strings with a slash.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path/to", slash_formatter::concat_with_slash("path", "to/"));
/// ```
pub fn concat_with_slash<S1: AsRef<str>, S2: AsRef<str>>(s1: S1, s2: S2) -> String {
    concat_with_slash_owned(s1.as_ref(), s2)
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
pub fn concat_with_slash_owned<S1: Into<String>, S2: AsRef<str>>(s1: S1, s2: S2) -> String {
    delete_end_slash_owned(add_end_slash_owned(s1) + delete_start_slash(s2.as_ref()))
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
pub fn concat_with_slash_mut<S2: AsRef<str>>(s1: &mut String, s2: S2) {
    add_end_slash_mut(s1);
    s1.push_str(delete_start_slash(s2.as_ref()));
    delete_end_slash_mut(s1);
}

/// Concatenate multiple strings with slashes.
///
/// ```
/// #[macro_use]
/// extern crate slash_formatter;
///
/// assert_eq!("path/to/file", concat_with_slash!("path", "to/", "/file/"));
///
/// let s = String::from("path");
///
/// let s = concat_with_slash!(s, "to/", "/file/");
///
/// assert_eq!("path/to/file", s);
/// ```
#[macro_export]
macro_rules! concat_with_slash {
    ($s:expr, $($sc:expr), *) => {
        {
            let mut s = $s.to_owned();

            $(
                ::slash_formatter::concat_with_slash_mut(&mut s, $sc);
            )*

            s
        }
    };
}

/// Concatenate multiple strings with slashes.
///
/// ```
/// #[macro_use]
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// concat_with_slash_mut!(&mut s, "to/", "/file/");
///
/// assert_eq!("path/to/file", s);
/// ```
#[macro_export]
macro_rules! concat_with_slash_mut {
    ($s:expr, $($sc:expr), *) => {
        {
            $(
                ::slash_formatter::concat_with_slash_mut($s, $sc);
            )*
        }
    };
}
