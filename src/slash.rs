use alloc::{borrow::Cow, string::String};

/// Delete an ending slash in a string except for '/'.
///
/// ```
/// assert_eq!("path", slash_formatter::delete_end_slash("path/"));
/// ```
#[inline]
pub fn delete_end_slash<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.ends_with('/') {
        unsafe { s.get_unchecked(..length - 1) }
    } else {
        s
    }
}

/// Delete an ending slash in a string except for '/'.
///
/// ```
/// let mut s = String::from("path/");
///
/// slash_formatter::delete_end_slash_in_place(&mut s);
///
/// assert_eq!("path", s);
/// ```
#[inline]
pub fn delete_end_slash_in_place(s: &mut String) {
    let length = s.len();

    if length > 1 && s.ends_with('/') {
        s.truncate(length - 1);
    }
}

/// Delete a starting slash in a string except for '/'.
///
/// ```
/// assert_eq!("path", slash_formatter::delete_start_slash("/path"));
/// ```
#[inline]
pub fn delete_start_slash<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.starts_with('/') {
        unsafe { s.get_unchecked(1..) }
    } else {
        s
    }
}

/// Delete a starting slash in a string except for '/'.
///
/// ```
/// let mut s = String::from("/path");
///
/// slash_formatter::delete_start_slash_in_place(&mut s);
///
/// assert_eq!("path", s);
/// ```
#[inline]
pub fn delete_start_slash_in_place(s: &mut String) {
    let length = s.len();

    if length > 1 && s.starts_with('/') {
        s.remove(0);
    }
}

/// Add a starting slash into a string.
///
/// ```
/// assert_eq!("/path", slash_formatter::add_start_slash("path"));
/// ```
#[inline]
pub fn add_start_slash<S: ?Sized + AsRef<str>>(s: &S) -> Cow<str> {
    let s = s.as_ref();

    if s.starts_with('/') {
        Cow::from(s)
    } else {
        Cow::from(format!("/{}", s))
    }
}

/// Add a starting slash into a string.
///
/// ```
/// let mut s = String::from("path");
///
/// slash_formatter::add_start_slash_in_place(&mut s);
///
/// assert_eq!("/path", s);
/// ```
#[inline]
pub fn add_start_slash_in_place(s: &mut String) {
    if !s.starts_with('/') {
        s.insert(0, '/');
    }
}

/// Add an ending slash into a string.
///
/// ```
/// assert_eq!("path/", slash_formatter::add_end_slash("path"));
/// ```
#[inline]
pub fn add_end_slash<S: ?Sized + AsRef<str>>(s: &S) -> Cow<str> {
    let s = s.as_ref();

    if s.ends_with('/') {
        Cow::from(s)
    } else {
        Cow::from(format!("{}/", s))
    }
}

/// Add an ending slash into a string.
///
/// ```
/// let mut s = String::from("path");
///
/// slash_formatter::add_end_slash_in_place(&mut s);
///
/// assert_eq!("path/", s);
/// ```
#[inline]
pub fn add_end_slash_in_place(s: &mut String) {
    if !s.ends_with('/') {
        s.push('/');
    }
}

/// Concatenate two strings with a slash.
///
/// ```
/// assert_eq!("path/to", slash_formatter::concat_with_slash("path", "to/"));
/// ```
#[inline]
pub fn concat_with_slash<S1: Into<String>, S2: AsRef<str>>(s1: S1, s2: S2) -> String {
    let mut s1 = s1.into();

    concat_with_slash_in_place(&mut s1, s2);

    s1
}

/// Concatenate two strings with a slash.
///
/// ```
/// let mut s = String::from("path");
///
/// slash_formatter::concat_with_slash_in_place(&mut s, "to/");
///
/// assert_eq!("path/to", s);
/// ```
#[inline]
pub fn concat_with_slash_in_place<S2: AsRef<str>>(s1: &mut String, s2: S2) {
    add_end_slash_in_place(s1);
    s1.push_str(delete_start_slash(s2.as_ref()));
    delete_end_slash_in_place(s1);
}

/**
Concatenate multiple strings with slashes. It can also be used to get the literal `'/'`.

```
#[macro_use] extern crate slash_formatter;

assert_eq!("path/to/file", slash!("path", "to/", "/file/"));

let s = String::from("path");

let s = slash!(s, "to/", "/file/");

assert_eq!("path/to/file", s);
```
*/
#[macro_export]
macro_rules! slash {
    () => {
        '/'
    };
    ($s:expr $(, $sc:expr)* $(,)*) => {
        {
            let mut s = $s.to_owned();

            $(
                $crate::concat_with_slash_in_place(&mut s, $sc);
            )*

            s
        }
    };
}

/**
Concatenate multiple strings with slashes. It can also be used to get the literal `'/'`.

```
#[macro_use] extern crate slash_formatter;

let mut s = String::from("path");

slash_in_place!(&mut s, "to/", "/file/");

assert_eq!("path/to/file", s);
```
*/
#[macro_export]
macro_rules! slash_in_place {
    () => {
        '/'
    };
    ($s:expr $(, $sc:expr)* $(,)*) => {
        $(
            $crate::concat_with_slash_in_place($s, $sc);
        )*
    };
}

concat_impl! {
    #[macro_export]
    /// Concatenates literals into a static string slice separated by a slash. Prefixes and suffixes can also be added.
    ///
    /// ```rust
    /// #[macro_use] extern crate slash_formatter;
    ///
    /// assert_eq!("test/10/b/true", concat_with_slash!("test", 10, 'b', true));
    /// ```
    concat_with_slash => "/"
}
