// This module contains the definition of the `Alternative` type and its
// implementations.

// Since I'm not publishing an `alternative` crate, I'm not going to
// replicate all implementations for `std::option::Option` here. For
// now, I will only include the methods and traits that I need for this
// project, plus some additional ones that may be needed soon.

// This is all based on a reply to this StackOverflow question:
// https://stackoverflow.com/questions/44331037/how-can-i-distinguish-between-a-deserialized-field-that-is-missing-and-one-that.

// Note: before anyone complains about the comments in this file, saying
// that these aren't idiomatic Rust docs, I will make this very clear:
// `Alternative` is not a crate, and this isn't its documentation.

// "Alternative" is a non-exact synonym to "Option", as it is an
// alternative to it. Its applications are the ones where the ability to
// distinguish missing values from `null` values, particularly in JSON
// Documents, is necessary.

// A synonym to "Option"'s meaning in this context, such as the name of
// a similar enum on the StackOverflow question linked above, "Maybe",
// would be more appropriate, nut I wanted to make that annoying pun in
// the comment above.

// In order to use this enum in a struct that needs to be Serialized,
// each field of type `Alternative<T>` must be annotated using the
// following attribute macro:
// `#[serde(default, skip_serializing_if = "Alternative::is_none")]`.

// Example 1:

// #[derive(Serialize, Deserialize)]
// struct Test {
//     // A value that must be included.
//     foo: String,

//     // A value that could be included.
//     bar: Option<u32>,

//     // A value that could be included, and could have a `null` value.*
//     #[serde(default, skip_serializing_if = "Alternative::is_none")]
//     baz: Alternative<bool>,
// }

// * `bar` could also have a `null` value, however in this hypothetical
//   example, it would be considered the same as not including the
//   field.

// Example 2:

// use serde_json::from_str;

// let bar_null = r#"{
//     "foo": "foo",
//     "bar": null,
//     "baz": true
// }"#;

// let bar_none = r#"{
//     "foo": "foo",
//     "baz": true
// }"#;

// // Missing fields are null.
// assert_eq!(
//     from_str::<Test>(bar_null),
//     from_str::<Test>(bar_none),
// );

// let baz_null = r#"{
//     "foo": "foo",
//     "bar": 123,
//     "baz": null
// }"#;

// let baz_none = r#"{
//     "foo": "foo",
//     "bar": 123,
// }"#;

// // Missing fields are not null.
// assert_ne!(
//     from_str::<Test>(baz_null),
//     from_str::<Test>(baz_none),
// );

use serde::{ser::Error, Deserialize, Deserializer, Serialize, Serializer};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Alternative<T> {
    None,    // No value was specified
    Null,    // `null` was specified.
    Some(T), // A different value was specified.
}

impl<T> Default for Alternative<T> {
    fn default() -> Self {
        Alternative::None
    }
}

impl<T> From<Option<T>> for Alternative<T> {
    fn from(option: Option<T>) -> Alternative<T> {
        match option {
            Some(value) => Alternative::Some(value),
            None => Alternative::Null,
        }
    }
}

impl<T> From<Alternative<T>> for Option<T> {
    fn from(alternative: Alternative<T>) -> Option<T> {
        match alternative {
            Alternative::Some(value) => Some(value),
            _ => None,
        }
    }
}

impl<'de, T> Deserialize<'de> for Alternative<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

impl<T: Serialize> Serialize for Alternative<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Alternative::Null => serializer.serialize_none(),
            Alternative::Some(value) => value.serialize(serializer),
            Alternative::None => Err(Error::custom(
                r#"Alternative fields must be annotated with:
  #[serde(default, skip_serializing_if = "Alternative::is_none")]"#,
            )),
        }
    }
}

#[allow(dead_code)]
impl<T> Alternative<T>
where
    T: Clone,
{
    pub fn to_option(&self) -> Option<T> {
        match self {
            Alternative::Some(value) => Some(value.clone()),

            // I could use `Alternative::Null | Alternative::None`,
            // but I'd rather only have to write (or read) `_`.
            _ => None,
        }
    }
}

#[allow(dead_code)]
impl<T> Alternative<T> {
    pub fn is_some(&self) -> bool {
        // I could use `if let` instead, but `std::option::Option` uses
        // `match!()` so I might as well use it too.
        matches!(*self, Alternative::Some(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(*self, Alternative::Null)
    }

    pub fn is_none(&self) -> bool {
        matches!(*self, Alternative::None)
    }

    pub fn contains<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        match self {
            Alternative::Some(y) => x == y,
            _ => false,
        }
    }

    pub const fn as_ref(&self) -> Alternative<&T> {
        match *self {
            Alternative::Some(ref value) => Alternative::Some(value),
            Alternative::Null => Alternative::Null,
            Alternative::None => Alternative::None,
        }
    }

    pub fn as_mut(&mut self) -> Alternative<&mut T> {
        match *self {
            Alternative::Some(ref mut value) => Alternative::Some(value),
            Alternative::Null => Alternative::Null,
            Alternative::None => Alternative::None,
        }
    }

    pub fn expect(self, msg: &str) -> T {
        match self {
            Alternative::Some(value) => value,
            _ => panic!("{}", msg),
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Alternative::Some(value) => value,
            Alternative::Null => {
                panic!("called `Alternative::unwrap()` on a `Alternative::Null` value")
            }
            Alternative::None => {
                panic!("called `Alternative::unwrap()` on a `Alternative::None` value")
            }
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Alternative::Some(value) => value,
            _ => default,
        }
    }

    pub fn unwrap_or_else<F: FnOnce() -> T>(self, function: F) -> T {
        match self {
            Alternative::Some(value) => value,
            _ => function(),
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, function: F) -> Alternative<U> {
        match self {
            Alternative::Some(x) => Alternative::Some(function(x)),
            Alternative::Null => Alternative::Null,
            Alternative::None => Alternative::None,
        }
    }

    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Alternative::Some(t) => f(t),
            _ => default,
        }
    }

    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, function: F) -> U {
        match self {
            Alternative::Some(value) => function(value),
            _ => default(),
        }
    }
}
