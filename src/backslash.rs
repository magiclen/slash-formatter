use alloc::borrow::Cow;
use alloc::string::String;

/// Delete an ending backslash in a string except for '\\\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_end_backslash("path\\"));
/// ```
#[inline]
pub fn delete_end_backslash<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.ends_with('\\') {
        &s[..length - 1]
    } else {
        s
    }
}

/// Delete an ending backslash in a string except for '\\\\'.
///
/// ```
/// extern crate slash_formatter;
///
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
        s.remove(length - 1);
    }
}

/// Delete a starting backslash in a string except for '\\\\'.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!("path", slash_formatter::delete_start_backslash("\\path"));
/// ```
#[inline]
pub fn delete_start_backslash<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    let s = s.as_ref();

    let length = s.len();

    if length > 1 && s.starts_with('\\') {
        &s[1..]
    } else {
        s
    }
}

/// Delete a starting backslash in a string except for '\\\\'.
///
/// ```
/// extern crate slash_formatter;
///
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
/// extern crate slash_formatter;
///
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
/// extern crate slash_formatter;
///
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
/// extern crate slash_formatter;
///
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
/// extern crate slash_formatter;
///
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
/// extern crate slash_formatter;
///
/// assert_eq!("path\\to", slash_formatter::concat_with_backslash("path", "to\\"));
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
/// extern crate slash_formatter;
///
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
#[macro_use] extern crate slash_formatter;

assert_eq!("path\\to\\file", backslash!("path", "to\\", "\\file\\"));

let s = String::from("path");

let s = backslash!(s, "to\\", "\\file\\");

assert_eq!("path\\to\\file", s);
```
*/
#[macro_export]
macro_rules! backslash {
    ($s:expr, $($sc:expr), *) => {
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
#[macro_use] extern crate slash_formatter;

let mut s = String::from("path");

backslash_in_place!(&mut s, "to\\", "\\file\\");

assert_eq!("path\\to\\file", s);
```
*/
#[macro_export]
macro_rules! backslash_in_place {
    ($s:expr, $($sc:expr), *) => {
        {
            $(
                $crate::concat_with_backslash_in_place($s, $sc);
            )*
        }
    };
}

/**
Concatenates literals into a static string slice separated by a backslash. Prefixes and suffixes can also be added.

```rust
#[macro_use] extern crate slash_formatter;

assert_eq!("test\\10\\b\\true", concat_with_backslash!("test", 10, 'b', true));
```
*/
#[macro_export]
macro_rules! concat_with_backslash {
    ($($e:expr),* $(,)*) => {
        $crate::concat_with::concat!(with "\\" $(, $e)*)
    };
    (prefix $p:expr $(, $e:expr)* $(,)*) => {
        $crate::concat_with::concat!(with "\\", prefix $p $(, $e)*)
    };
    (suffix $s:expr $(, $e:expr)* $(,)*) => {
        $crate::concat_with::concat!(with "\\", suffix $s $(, $e)*)
    };
    (prefix $p:expr, suffix $s:expr $(, $e:expr)* $(,)*) => {
        $crate::concat_with::concat!(with "\\", prefix $p, suffix $s $(, $e)*)
    };
    (suffix $s:expr, prefix $p:expr $(, $e:expr)* $(,)*) => {
        $crate::concat_with::concat!(with "\\", prefix $p, suffix $s $(, $e)*)
    };
}
