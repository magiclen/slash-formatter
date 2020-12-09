use alloc::borrow::Cow;
use alloc::string::String;

/// Delete an ending ``FILE_SEPARATOR_ON_WORKSTATION`` in a string except for just ``FILE_SEPARATOR_ON_WORKSTATION``.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(
///     "path",
///     slash_formatter::delete_end_file_separator_build(concat!(
///         "path",
///         slash_formatter::file_separator_build!()
///     ))
/// );
/// ```
#[inline]
pub fn delete_end_file_separator_build<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    #[cfg(from_unix)]
    {
        crate::delete_end_slash(s)
    }

    #[cfg(from_windows)]
    {
        crate::delete_end_backslash(s)
    }
}

/// Delete an ending ``FILE_SEPARATOR_ON_WORKSTATION`` in a string except for just ``FILE_SEPARATOR_ON_WORKSTATION``.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from(concat!("path", slash_formatter::file_separator_build!()));
///
/// slash_formatter::delete_end_file_separator_build_in_place(&mut s);
///
/// assert_eq!("path", s);
/// ```
#[inline]
pub fn delete_end_file_separator_build_in_place(s: &mut String) {
    #[cfg(from_unix)]
    {
        crate::delete_end_slash_in_place(s)
    }

    #[cfg(from_windows)]
    {
        crate::delete_end_backslash_in_place(s)
    }
}

/// Delete a starting ``FILE_SEPARATOR_ON_WORKSTATION`` in a string except for just ``FILE_SEPARATOR_ON_WORKSTATION``.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(
///     "path",
///     slash_formatter::delete_start_file_separator_build(concat!(
///         slash_formatter::file_separator_build!(),
///         "path"
///     ))
/// );
/// ```
#[inline]
pub fn delete_start_file_separator_build<S: ?Sized + AsRef<str>>(s: &S) -> &str {
    #[cfg(from_unix)]
    {
        crate::delete_start_slash(s)
    }

    #[cfg(from_windows)]
    {
        crate::delete_start_backslash(s)
    }
}

/// Delete a starting ``FILE_SEPARATOR_ON_WORKSTATION`` in a string except for just ``FILE_SEPARATOR_ON_WORKSTATION``.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from(concat!(slash_formatter::file_separator_build!(), "path"));
///
/// slash_formatter::delete_start_file_separator_build_in_place(&mut s);
///
/// assert_eq!("path", s);
/// ```
#[inline]
pub fn delete_start_file_separator_build_in_place(s: &mut String) {
    #[cfg(from_unix)]
    {
        crate::delete_start_slash_in_place(s)
    }

    #[cfg(from_windows)]
    {
        crate::delete_start_backslash_in_place(s)
    }
}

/// Add a starting ``FILE_SEPARATOR_ON_WORKSTATION`` into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(
///     concat!(slash_formatter::file_separator_build!(), "path"),
///     slash_formatter::add_start_file_separator_build("path")
/// );
/// ```
#[inline]
pub fn add_start_file_separator_build<S: ?Sized + AsRef<str>>(s: &S) -> Cow<str> {
    #[cfg(from_unix)]
    {
        crate::add_start_slash(s)
    }

    #[cfg(from_windows)]
    {
        crate::add_start_backslash(s)
    }
}

/// Add a starting ``FILE_SEPARATOR_ON_WORKSTATION`` into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::add_start_file_separator_build_in_place(&mut s);
///
/// assert_eq!(concat!(slash_formatter::file_separator_build!(), "path"), s);
/// ```
#[inline]
pub fn add_start_file_separator_build_in_place(s: &mut String) {
    #[cfg(from_unix)]
    {
        crate::add_start_slash_in_place(s)
    }

    #[cfg(from_windows)]
    {
        crate::add_start_backslash_in_place(s)
    }
}

/// Add an ending ``FILE_SEPARATOR_ON_WORKSTATION`` into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(
///     concat!("path", slash_formatter::file_separator_build!()),
///     slash_formatter::add_end_file_separator_build("path")
/// );
/// ```
#[inline]
pub fn add_end_file_separator_build<S: ?Sized + AsRef<str>>(s: &S) -> Cow<str> {
    #[cfg(from_unix)]
    {
        crate::add_end_slash(s)
    }

    #[cfg(from_windows)]
    {
        crate::add_end_backslash(s)
    }
}

/// Add an ending ``FILE_SEPARATOR_ON_WORKSTATION`` into a string.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::add_end_file_separator_build_in_place(&mut s);
///
/// assert_eq!(concat!("path", slash_formatter::file_separator_build!()), s);
/// ```
#[inline]
pub fn add_end_file_separator_build_in_place(s: &mut String) {
    #[cfg(from_unix)]
    {
        crate::add_end_slash_in_place(s)
    }

    #[cfg(from_windows)]
    {
        crate::add_end_backslash_in_place(s)
    }
}

/// Concatenate two strings with ``FILE_SEPARATOR_ON_WORKSTATION``.
///
/// ```
/// extern crate slash_formatter;
///
/// assert_eq!(
///     concat!("path", slash_formatter::file_separator_build!(), "to"),
///     slash_formatter::concat_with_file_separator_build(
///         "path",
///         concat!("to", slash_formatter::file_separator_build!())
///     )
/// );
/// ```
#[inline]
pub fn concat_with_file_separator_build<S1: Into<String>, S2: AsRef<str>>(
    s1: S1,
    s2: S2,
) -> String {
    #[cfg(from_unix)]
    {
        crate::concat_with_slash(s1, s2)
    }

    #[cfg(from_windows)]
    {
        crate::concat_with_backslash(s1, s2)
    }
}

/// Concatenate two strings with ``FILE_SEPARATOR_ON_WORKSTATION``.
///
/// ```
/// extern crate slash_formatter;
///
/// let mut s = String::from("path");
///
/// slash_formatter::concat_with_file_separator_build_in_place(
///     &mut s,
///     concat!("to", slash_formatter::file_separator_build!()),
/// );
///
/// assert_eq!(concat!("path", slash_formatter::file_separator_build!(), "to"), s);
/// ```
#[inline]
pub fn concat_with_file_separator_build_in_place<S2: AsRef<str>>(s1: &mut String, s2: S2) {
    #[cfg(from_unix)]
    {
        crate::concat_with_slash_in_place(s1, s2)
    }

    #[cfg(from_windows)]
    {
        crate::concat_with_backslash_in_place(s1, s2)
    }
}

#[cfg(from_unix)]
/**
Concatenate multiple strings with ``FILE_SEPARATOR_ON_WORKSTATION``. It can also be used to get the literal ``FILE_SEPARATOR_ON_WORKSTATION``.

```
#[macro_use] extern crate slash_formatter;

assert_eq!(slash_formatter::concat_with_file_separator_build!("path", "to", "file"), file_separator_build!("path", concat!("to", slash_formatter::file_separator_build!()), concat!(slash_formatter::file_separator_build!(), "file", slash_formatter::file_separator_build!())));

let s = String::from("path");

let s = file_separator_build!(s, concat!("to", slash_formatter::file_separator_build!()), concat!(slash_formatter::file_separator_build!(), "file", slash_formatter::file_separator_build!()));

assert_eq!(slash_formatter::concat_with_file_separator_build!("path", "to", "file"), s);
```
*/
#[macro_export]
macro_rules! file_separator_build {
    ($($t:tt)*) => {
        $crate::slash!($($t)*)
    };
}

#[cfg(from_windows)]
/**
Concatenate multiple strings with ``FILE_SEPARATOR_ON_WORKSTATION``. It can also be used to get the literal ``FILE_SEPARATOR_ON_WORKSTATION``.

```
#[macro_use] extern crate slash_formatter;

assert_eq!(slash_formatter::concat_with_file_separator_build!("path", "to", "file"), file_separator_build!("path", concat!("to", slash_formatter::file_separator_build!()), concat!(slash_formatter::file_separator_build!(), "file", slash_formatter::file_separator_build!())));

let s = String::from("path");

let s = file_separator_build!(s, concat!("to", slash_formatter::file_separator_build!()), concat!(slash_formatter::file_separator_build!(), "file", slash_formatter::file_separator_build!()));

assert_eq!(slash_formatter::concat_with_file_separator_build!("path", "to", "file"), s);
```
*/
#[macro_export]
macro_rules! file_separator_build {
    ($($t:tt)*) => {
        $crate::backslash!($($t)*)
    };
}

#[cfg(from_unix)]
/**
Concatenate multiple strings with ``FILE_SEPARATOR_ON_WORKSTATION``. It can also be used to get the literal ``FILE_SEPARATOR_ON_WORKSTATION``.

```
#[macro_use] extern crate slash_formatter;

let mut s = String::from("path");

file_separator_build_in_place!(&mut s, concat!("to", slash_formatter::file_separator_build!()), concat!(slash_formatter::file_separator_build!(), "file", slash_formatter::file_separator_build!()));

assert_eq!(slash_formatter::concat_with_file_separator_build!("path", "to", "file"), s);
```
*/
#[macro_export]
macro_rules! file_separator_build_in_place {
    ($($t:tt)*) => {
        $crate::slash_in_place!($($t)*)
    };
}

#[cfg(from_windows)]
/**
Concatenate multiple strings with ``FILE_SEPARATOR_ON_WORKSTATION``. It can also be used to get the literal ``FILE_SEPARATOR_ON_WORKSTATION``.

```
#[macro_use] extern crate slash_formatter;

let mut s = String::from("path");

file_separator_build_in_place!(&mut s, concat!("to", slash_formatter::file_separator_build!()), concat!(slash_formatter::file_separator_build!(), "file", slash_formatter::file_separator_build!()));

assert_eq!(slash_formatter::concat_with_file_separator_build!("path", "to", "file"), s);
```
*/
#[macro_export]
macro_rules! file_separator_build_in_place {
    ($($t:tt)*) => {
        $crate::backslash_in_place!($($t)*)
    };
}

#[cfg(from_unix)]
/**
Concatenates literals into a static string slice separated by ``FILE_SEPARATOR_ON_WORKSTATION``. Prefixes and suffixes can also be added.

```
#[macro_use] extern crate slash_formatter;

assert_eq!(concat!("test", slash_formatter::file_separator_build!(), 10, slash_formatter::file_separator_build!(), 'b', slash_formatter::file_separator_build!(), true), concat_with_file_separator_build!("test", 10, 'b', true));
```
*/
#[macro_export]
macro_rules! concat_with_file_separator_build {
    ($($t:tt)*) => {
        $crate::concat_with_slash!($($t)*)
    };
}

#[cfg(from_windows)]
/**
Concatenates literals into a static string slice separated by ``FILE_SEPARATOR_ON_WORKSTATION``. Prefixes and suffixes can also be added.

```
#[macro_use] extern crate slash_formatter;

assert_eq!(concat!("test", slash_formatter::file_separator_build!(), 10, slash_formatter::file_separator_build!(), 'b', slash_formatter::file_separator_build!(), true), concat_with_file_separator_build!("test", 10, 'b', true));
```
*/
#[macro_export]
macro_rules! concat_with_file_separator_build {
    ($($t:tt)*) => {
        $crate::concat_with_backslash!($($t)*)
    };
}

#[cfg(debug_assertions)]
/**
Concatenates literals into a static string slice separated by file separators which depends on the target OS if in debug mode, or depends on the workstation if in release mode. Prefixes and suffixes can also be added.

```
#[macro_use] extern crate slash_formatter;

if cfg!(debug_assertions) {
    assert_eq!(concat!("test", slash_formatter::file_separator!(), 10, slash_formatter::file_separator!(), 'b', slash_formatter::file_separator!(), true), concat_with_file_separator_debug_release!("test", 10, 'b', true));
}

if !cfg!(debug_assertions) {
    assert_eq!(concat!("test", slash_formatter::file_separator_build!(), 10, slash_formatter::file_separator_build!(), 'b', slash_formatter::file_separator_build!(), true), concat_with_file_separator_debug_release!("test", 10, 'b', true));
}
```
*/
#[macro_export]
macro_rules! concat_with_file_separator_debug_release {
    ($($t:tt)*) => {
        $crate::concat_with_file_separator!($($t)*)
    };
}

#[cfg(not(debug_assertions))]
/**
Concatenates literals into a static string slice separated by file separators which dependents on the target OS if in debug mode, or dependents on the workstation if in release mode. Prefixes and suffixes can also be added.

```
#[macro_use] extern crate slash_formatter;

if cfg!(debug_assertions) {
    assert_eq!(concat!("test", slash_formatter::file_separator!(), 10, slash_formatter::file_separator!(), 'b', slash_formatter::file_separator!(), true), concat_with_file_separator_debug_release!("test", 10, 'b', true));
}

if !cfg!(debug_assertions) {
    assert_eq!(concat!("test", slash_formatter::file_separator_build!(), 10, slash_formatter::file_separator_build!(), 'b', slash_formatter::file_separator_build!(), true), concat_with_file_separator_debug_release!("test", 10, 'b', true));
}
```
*/
#[macro_export]
macro_rules! concat_with_file_separator_debug_release {
    ($($t:tt)*) => {
        $crate::concat_with_file_separator_build!($($t)*)
    };
}
