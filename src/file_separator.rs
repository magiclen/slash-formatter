use alloc::{borrow::Cow, string::String};

/// Delete an ending `FILE_SEPARATOR` in a string except for just `FILE_SEPARATOR`.
///
/// ```
/// assert_eq!(
///     "path",
///     slash_formatter::delete_end_file_separator(concat!(
///         "path",
///         slash_formatter::file_separator!()
///     ))
/// );
/// ```
#[inline]
pub fn delete_end_file_separator<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    #[cfg(unix)]
    {
        crate::delete_end_slash(s)
    }

    #[cfg(windows)]
    {
        crate::delete_end_backslash(s)
    }
}

/// Delete an ending `FILE_SEPARATOR` in a string except for just `FILE_SEPARATOR`.
///
/// ```
/// let mut s =
///     String::from(concat!("path", slash_formatter::file_separator!()));
///
/// slash_formatter::delete_end_file_separator_in_place(&mut s);
///
/// assert_eq!("path", s);
/// ```
#[inline]
pub fn delete_end_file_separator_in_place(s: &mut String) {
    #[cfg(unix)]
    {
        crate::delete_end_slash_in_place(s)
    }

    #[cfg(windows)]
    {
        crate::delete_end_backslash_in_place(s)
    }
}

/// Delete a starting `FILE_SEPARATOR` in a string except for just `FILE_SEPARATOR`.
///
/// ```
/// assert_eq!(
///     "path",
///     slash_formatter::delete_start_file_separator(concat!(
///         slash_formatter::file_separator!(),
///         "path"
///     ))
/// );
/// ```
#[inline]
pub fn delete_start_file_separator<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    #[cfg(unix)]
    {
        crate::delete_start_slash(s)
    }

    #[cfg(windows)]
    {
        crate::delete_start_backslash(s)
    }
}

/// Delete a starting `FILE_SEPARATOR` in a string except for just `FILE_SEPARATOR`.
///
/// ```
/// let mut s =
///     String::from(concat!(slash_formatter::file_separator!(), "path"));
///
/// slash_formatter::delete_start_file_separator_in_place(&mut s);
///
/// assert_eq!("path", s);
/// ```
#[inline]
pub fn delete_start_file_separator_in_place(s: &mut String) {
    #[cfg(unix)]
    {
        crate::delete_start_slash_in_place(s)
    }

    #[cfg(windows)]
    {
        crate::delete_start_backslash_in_place(s)
    }
}

/// Add a starting `FILE_SEPARATOR` into a string.
///
/// ```
/// assert_eq!(
///     concat!(slash_formatter::file_separator!(), "path"),
///     slash_formatter::add_start_file_separator("path")
/// );
/// ```
#[inline]
pub fn add_start_file_separator<S: ?Sized + AsRef<str>>(s: &S) -> Cow<str> {
    #[cfg(unix)]
    {
        crate::add_start_slash(s)
    }

    #[cfg(windows)]
    {
        crate::add_start_backslash(s)
    }
}

/// Add a starting `FILE_SEPARATOR` into a string.
///
/// ```
/// let mut s = String::from("path");
///
/// slash_formatter::add_start_file_separator_in_place(&mut s);
///
/// assert_eq!(concat!(slash_formatter::file_separator!(), "path"), s);
/// ```
#[inline]
pub fn add_start_file_separator_in_place(s: &mut String) {
    #[cfg(unix)]
    {
        crate::add_start_slash_in_place(s)
    }

    #[cfg(windows)]
    {
        crate::add_start_backslash_in_place(s)
    }
}

/// Add an ending `FILE_SEPARATOR` into a string.
///
/// ```
/// assert_eq!(
///     concat!("path", slash_formatter::file_separator!()),
///     slash_formatter::add_end_file_separator("path")
/// );
/// ```
#[inline]
pub fn add_end_file_separator<S: ?Sized + AsRef<str>>(s: &S) -> Cow<str> {
    #[cfg(unix)]
    {
        crate::add_end_slash(s)
    }

    #[cfg(windows)]
    {
        crate::add_end_backslash(s)
    }
}

/// Add an ending `FILE_SEPARATOR` into a string.
///
/// ```
/// let mut s = String::from("path");
///
/// slash_formatter::add_end_file_separator_in_place(&mut s);
///
/// assert_eq!(concat!("path", slash_formatter::file_separator!()), s);
/// ```
#[inline]
pub fn add_end_file_separator_in_place(s: &mut String) {
    #[cfg(unix)]
    {
        crate::add_end_slash_in_place(s)
    }

    #[cfg(windows)]
    {
        crate::add_end_backslash_in_place(s)
    }
}

/// Concatenate two strings with `FILE_SEPARATOR`.
///
/// ```
/// assert_eq!(
///     concat!("path", slash_formatter::file_separator!(), "to"),
///     slash_formatter::concat_with_file_separator(
///         "path",
///         concat!("to", slash_formatter::file_separator!())
///     )
/// );
/// ```
#[inline]
pub fn concat_with_file_separator<S1: Into<String>, S2: AsRef<str>>(s1: S1, s2: S2) -> String {
    #[cfg(unix)]
    {
        crate::concat_with_slash(s1, s2)
    }

    #[cfg(windows)]
    {
        crate::concat_with_backslash(s1, s2)
    }
}

/// Concatenate two strings with `FILE_SEPARATOR`.
///
/// ```
/// let mut s = String::from("path");
///
/// slash_formatter::concat_with_file_separator_in_place(
///     &mut s,
///     concat!("to", slash_formatter::file_separator!()),
/// );
///
/// assert_eq!(concat!("path", slash_formatter::file_separator!(), "to"), s);
/// ```
#[inline]
pub fn concat_with_file_separator_in_place<S2: AsRef<str>>(s1: &mut String, s2: S2) {
    #[cfg(unix)]
    {
        crate::concat_with_slash_in_place(s1, s2)
    }

    #[cfg(windows)]
    {
        crate::concat_with_backslash_in_place(s1, s2)
    }
}

#[cfg(unix)]
/**
Concatenate multiple strings with `FILE_SEPARATOR`. It can also be used to get the literal `FILE_SEPARATOR`.

```
#[macro_use] extern crate slash_formatter;

assert_eq!(slash_formatter::concat_with_file_separator!("path", "to", "file"), file_separator!("path", concat!("to", slash_formatter::file_separator!()), concat!(slash_formatter::file_separator!(), "file", slash_formatter::file_separator!())));

let s = String::from("path");

let s = file_separator!(s, concat!("to", slash_formatter::file_separator!()), concat!(slash_formatter::file_separator!(), "file", slash_formatter::file_separator!()));

assert_eq!(slash_formatter::concat_with_file_separator!("path", "to", "file"), s);
```
*/
#[macro_export]
macro_rules! file_separator {
    ($($t:tt)*) => {
        $crate::slash!($($t)*)
    };
}

#[cfg(windows)]
/**
Concatenate multiple strings with `FILE_SEPARATOR`. It can also be used to get the literal `FILE_SEPARATOR`.

```
#[macro_use] extern crate slash_formatter;

assert_eq!(slash_formatter::concat_with_file_separator!("path", "to", "file"), file_separator!("path", concat!("to", slash_formatter::file_separator!()), concat!(slash_formatter::file_separator!(), "file", slash_formatter::file_separator!())));

let s = String::from("path");

let s = file_separator!(s, concat!("to", slash_formatter::file_separator!()), concat!(slash_formatter::file_separator!(), "file", slash_formatter::file_separator!()));

assert_eq!(slash_formatter::concat_with_file_separator!("path", "to", "file"), s);
```
*/
#[macro_export]
macro_rules! file_separator {
    ($($t:tt)*) => {
        $crate::backslash!($($t)*)
    };
}

#[cfg(unix)]
/**
Concatenate multiple strings with `FILE_SEPARATOR`. It can also be used to get the literal `FILE_SEPARATOR`.

```
#[macro_use] extern crate slash_formatter;

let mut s = String::from("path");

file_separator_in_place!(&mut s, concat!("to", slash_formatter::file_separator!()), concat!(slash_formatter::file_separator!(), "file", slash_formatter::file_separator!()));

assert_eq!(slash_formatter::concat_with_file_separator!("path", "to", "file"), s);
```
*/
#[macro_export]
macro_rules! file_separator_in_place {
    ($($t:tt)*) => {
        $crate::slash_in_place!($($t)*)
    };
}

#[cfg(windows)]
/**
Concatenate multiple strings with `FILE_SEPARATOR`. It can also be used to get the literal `FILE_SEPARATOR`.

```
#[macro_use] extern crate slash_formatter;

let mut s = String::from("path");

file_separator_in_place!(&mut s, concat!("to", slash_formatter::file_separator!()), concat!(slash_formatter::file_separator!(), "file", slash_formatter::file_separator!()));

assert_eq!(slash_formatter::concat_with_file_separator!("path", "to", "file"), s);
```
*/
#[macro_export]
macro_rules! file_separator_in_place {
    ($($t:tt)*) => {
        $crate::backslash_in_place!($($t)*)
    };
}

#[cfg(unix)]
/**
Concatenates literals into a static string slice separated by `FILE_SEPARATOR`. Prefixes and suffixes can also be added.

```rust
#[macro_use] extern crate slash_formatter;

assert_eq!(concat!("test", slash_formatter::file_separator!(), 10, slash_formatter::file_separator!(), 'b', slash_formatter::file_separator!(), true), concat_with_file_separator!("test", 10, 'b', true));
```
*/
#[macro_export]
macro_rules! concat_with_file_separator {
    ($($t:tt)*) => {
        $crate::concat_with_slash!($($t)*)
    };
}

#[cfg(windows)]
/**
Concatenates literals into a static string slice separated by `FILE_SEPARATOR`. Prefixes and suffixes can also be added.

```rust
#[macro_use] extern crate slash_formatter;

assert_eq!(concat!("test", slash_formatter::file_separator!(), 10, slash_formatter::file_separator!(), 'b', slash_formatter::file_separator!(), true), concat_with_file_separator!("test", 10, 'b', true));
```
*/
#[macro_export]
macro_rules! concat_with_file_separator {
    ($($t:tt)*) => {
        $crate::concat_with_backslash!($($t)*)
    };
}
