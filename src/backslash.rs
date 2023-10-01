use alloc::{borrow::Cow, string::String};

/// Delete an ending backslash in a string except for '\\\\'.
///
/// ```
/// assert_eq!("path", slash_formatter::delete_end_backslash("path\\"));
/// ```
#[inline]
pub fn delete_end_backslash<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.ends_with('\\') {
        unsafe { s.get_unchecked(..length - 1) }
    } else {
        s
    }
}

/// Delete an ending backslash in a string except for '\\\\'.
///
/// ```
/// let mut s = String::from("path\\");
///
/// slash_formatter::delete_end_backslash_in_place(&mut s);
///
/// assert_eq!("path", s);
/// ```
#[inline]
pub fn delete_end_backslash_in_place(s: &mut String) {
    let length = s.len();

    if length > 1 && s.ends_with('\\') {
        unsafe {
            s.as_mut_vec().set_len(length - 1);
        }
    }
}

/// Delete a starting backslash in a string except for '\\\\'.
///
/// ```
/// assert_eq!("path", slash_formatter::delete_start_backslash("\\path"));
/// ```
#[inline]
pub fn delete_start_backslash<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.starts_with('\\') {
        unsafe { s.get_unchecked(1..) }
    } else {
        s
    }
}

/// Delete a starting backslash in a string except for '\\\\'.
///
/// ```
/// let mut s = String::from("\\path");
///
/// slash_formatter::delete_start_backslash_in_place(&mut s);
///
/// assert_eq!("path", s);
/// ```
#[inline]
pub fn delete_start_backslash_in_place(s: &mut String) {
    let length = s.len();

    if length > 1 && s.starts_with('\\') {
        s.remove(0);
    }
}

/// Add a starting backslash into a string.
///
/// ```
/// assert_eq!("\\path", slash_formatter::add_start_backslash("path"));
/// ```
#[inline]
pub fn add_start_backslash<S: ?Sized + AsRef<str>>(s: &S) -> Cow<str> {
    let s = s.as_ref();

    if s.starts_with('\\') {
        Cow::from(s)
    } else {
        Cow::from(format!("\\{}", s))
    }
}

/// Add a starting backslash into a string.
///
/// ```
/// let mut s = String::from("path");
///
/// slash_formatter::add_start_backslash_in_place(&mut s);
///
/// assert_eq!("\\path", s);
/// ```
#[inline]
pub fn add_start_backslash_in_place(s: &mut String) {
    if !s.starts_with('\\') {
        s.insert(0, '\\');
    }
}

/// Add an ending backslash into a string.
///
/// ```
/// assert_eq!("path\\", slash_formatter::add_end_backslash("path"));
/// ```
#[inline]
pub fn add_end_backslash<S: ?Sized + AsRef<str>>(s: &S) -> Cow<str> {
    let s = s.as_ref();

    if s.ends_with('\\') {
        Cow::from(s)
    } else {
        Cow::from(format!("{}\\", s))
    }
}

/// Add an ending backslash into a string.
///
/// ```
/// let mut s = String::from("path");
///
/// slash_formatter::add_end_backslash_in_place(&mut s);
///
/// assert_eq!("path\\", s);
/// ```
#[inline]
pub fn add_end_backslash_in_place(s: &mut String) {
    if !s.ends_with('\\') {
        s.push('\\');
    }
}

/// Concatenate two strings with a backslash.
///
/// ```
/// assert_eq!(
///     "path\\to",
///     slash_formatter::concat_with_backslash("path", "to\\")
/// );
/// ```
#[inline]
pub fn concat_with_backslash<S1: Into<String>, S2: AsRef<str>>(s1: S1, s2: S2) -> String {
    let mut s1 = s1.into();

    concat_with_backslash_in_place(&mut s1, s2);

    s1
}

/// Concatenate two strings with a backslash.
///
/// ```
/// let mut s = String::from("path");
///
/// slash_formatter::concat_with_backslash_in_place(&mut s, "to\\");
///
/// assert_eq!("path\\to", s);
/// ```
#[inline]
pub fn concat_with_backslash_in_place<S2: AsRef<str>>(s1: &mut String, s2: S2) {
    add_end_backslash_in_place(s1);
    s1.push_str(delete_start_backslash(s2.as_ref()));
    delete_end_backslash_in_place(s1);
}

/**
Concatenate multiple strings with backslashes.

```
assert_eq!("path\\to\\file", slash_formatter::backslash!("path", "to\\", "\\file\\"));

let s = String::from("path");

let s = slash_formatter::backslash!(s, "to\\", "\\file\\");

assert_eq!("path\\to\\file", s);
```
*/
#[macro_export]
macro_rules! backslash {
    () => {
        '\\'
    };
    ($s:expr $(, $sc:expr)* $(,)*) => {
        {
            let mut s = $s.to_owned();

            $(
                $crate::concat_with_backslash_in_place(&mut s, $sc);
            )*

            s
        }
    };
}

/**
Concatenate multiple strings with backslashes.

```
let mut s = String::from("path");

slash_formatter::backslash_in_place!(&mut s, "to\\", "\\file\\");

assert_eq!("path\\to\\file", s);
```
*/
#[macro_export]
macro_rules! backslash_in_place {
    () => {
        '\\'
    };
    ($s:expr $(, $sc:expr)* $(,)*) => {
        $(
            $crate::concat_with_backslash_in_place($s, $sc);
        )*
    };
}

concat_with::concat_impl! {
    #[macro_export]
    /// Concatenates literals into a static string slice separated by a backslash. Prefixes and suffixes can also be added.
    ///
    /// ```rust
    /// assert_eq!("test\\10\\b\\true", slash_formatter::concat_with_backslash!("test", 10, 'b', true));
    /// ```
    concat_with_backslash => "\\"
}
