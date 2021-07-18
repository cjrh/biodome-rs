#![allow(non_snake_case)]

//! *Easier access to environment variables*
//!
//! *biodome* does two things:
//! 1. Automatically cast env vars to the "right" types.
//! 2. Automatically parse structured types from env vars.
//!
//! This crate is a rust implementation of a
//! [similar](https://github.com/cjrh/biodome) library I
//! made for Python several years ago. I used it mainly
//! for microservices, which I am now starting to build
//! in Rust also.
//!
//! # Demo
//!
//! This reads an environment variable called `TIMEOUT`:
//!
//! ```rust
//! use biodome::biodome;
//!
//! let TIMEOUT = biodome("TIMEOUT", 10);
//! let PORTS = biodome("PORTS", vec![8081, 8082, 8083]);
//! ```
//!
//! Consider the `TIMEOUT` identifier:
//!
//! - If the env var has not been set, the default value of `10`
//!   will be used, i.e., assigned to `TIMEOUT`.
//! - If the env var has been set, it will automatically be
//!   converted to the correct type, and then assigned.
//! - The type of the identifier `TIMEOUT` will always be the
//!   same as the type of the default value.
//!
//! Consider the `PORTS` identifier:
//!
//! - The default value is a `Vec<i32>`: this means that the
//!   type`PORTS` will always be a `Vec<i32>`.
//! - *biodome* will parse the env var, if set, to make that
//!   happen. An env var like this (bash) would
//!   work: `export PORTS=[81, 82]`
//!
//! # Simple Types
//!
//! In the above example, the literal integer `10` is of type
//! `i32` by default. For most primitive types this can be
//! controlled in an obvious way:
//!
//! ```rust
//! use biodome::biodome;
//!
//! let TIMEOUT = biodome("TIMEOUT", 10u8);
//! let TIMEOUT = biodome("TIMEOUT", 10f64);
//! ```
//!
//! In the first line above example, `TIMEOUT` will be a `u8`, and any
//! env var value will have to set appropriately or a runtime
//! error will occur. Likewise, if the default is an `f64`, then `TIMEOUT`
//! will be an `f64`.
//!
//! Boolean values are handled a little differently than for parsing:
//!
//! ```rust
//! use biodome::biodome;
//!
//! /// This line sets an environment variable, same as
//! /// if you have done `export DEBUG=yes` in Bash.
//! std::env::set_var("DEBUG", "yes");
//!
//! /// This line reads the value of the env var in Rust.
//! /// Because the default value is a bool, it means that
//! /// biodome will attempt to convert the value of the
//! /// env var into a bool.
//! let DEBUG = biodome("DEBUG", false);
//!
//! assert_eq!(DEBUG, true);
//! ```
//!
//! If the env var has been set to a wide range of "probably truthy"
//! patterns, the result will be `true`; otherwise, `false`. Some
//! of these values are (case-insensitively) `true`, `t`, `yes`, `y`,
//! `on`, `active`, `enabled`, `1`, `ok` and so on.
//!
//! # Structured Types
//!
//! If all *biodome* did was cast primitive types, it would be
//! mildly interesting. We also have support for more structured
//! types. To support this, we're parsing all structured types
//! using a limited subset of the [TOML](https://toml.io/en/v1.0.0)
//! markup format.
//!
//! Imagine that the following 3 env vars are set:
//!
//! ```bash
//! export LOGLEVELS='{ root = "warn", http = "info" }'
//! export TIMEOUTS='{ connect = 5.0, request = 10.0 }'
//! export PROXIES='["a.proxy.com:8000", "b.proxy.com:8001"]'
//! ```
//!
//! In the above, `LOGLEVELS` and `TIMEOUTS` are formatted
//! as [TOML inline tables](https://toml.io/en/v1.0.0#inline-table)
//! while `PROXIES` is formatted as
//! a [TOML array](https://toml.io/en/v1.0.0#array).
//!
//! These can be accessed with *biodome* like this:
//!
//! ```rust
//! use biodome::biodome;
//! use std::collections::HashMap;
//! use std::iter::FromIterator;
//!
//! /// Create the default values for the structured types
//! let default_loglevels = HashMap::from_iter(
//!     vec![(String::from("root"), String::from("info"))]
//! );
//!
//! let default_timeouts = HashMap::from_iter(
//!     vec![
//!         (String::from("resolve"), 1.0),
//!         (String::from("connect"), 1.0),
//!         (String::from("request"), 1.0),
//!     ]
//! );
//!
//! let default_proxies = vec![
//!     "dev.proxy.com:9009".to_string(),
//! ];
//!
//! /// Read the env vars
//! let LOGLEVELS = biodome("LOGLEVELS", default_loglevels);
//! let TIMEOUTS = biodome("TIMEOUTS", default_timeouts);
//! let PROXIES = biodome("TIMEOUTS", default_proxies);
//!
//! ```
//! In the above example, `LOGLEVELS` will be a `HashMap<String, String>`,
//! `TIMEOUTS` will be a `HashMap<String, f64>`, and `PROXIES` will be
//! a `Vec<String>`.
//!
//! # Alternative Projects
//!
//! [envy](https://github.com/softprops/envy) uses the power of
//! [Serde derive](https://serde.rs/derive.html) to work magic in populating
//! a "settings" struct.
//!
//! # Developer Info
//!
//! This README is generated with
//! [cargo-readme](https://github.com/livioribeiro/cargo-readme).
//! Please follow its instructions on how to set it up. The README
//! file can be regenerated with `cargo readme > README.md`.

mod rawconv;

use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::str::FromStr;

pub trait TryFromEnv: Sized {
    type Error;
    fn try_from_env(value: &str) -> Result<Self, Self::Error>;
}

pub trait TryIntoEnv<T>: Sized {
    type Error;
    fn try_into_env(&self) -> Result<T, Self::Error>;
}

// How the heck to make this work??
// impl<T: FromStr> TryFromEnv<String> for T {
//     type Error = &'static str;
//
//     fn try_from_env(value: String) -> Result<Self, Self::Error> {
//         value.v.parse().map_err(|err| "parse error")
//     }
// }

// How the heck to make this work??
// impl TryFromEnv for &str {
//     type Error = &'static str;
//
//     fn try_from_env(value: &str) -> Result<Self, Self::Error> {
//         Ok(value)
//     }
// }

impl TryFromEnv for String {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        Ok(value.to_string())
    }
}

impl TryFromEnv for bool {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        // value.parse().map_err(|err| "parse error")
        Ok(rawconv::to_bool(value))
    }
}

impl TryFromEnv for usize {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        // value.parse().map_err(|err| "parse error")
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for i8 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for u8 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for i16 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for u16 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for i32 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for i64 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for u32 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for u64 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for f32 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl TryFromEnv for f64 {
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_prim(value)
    }
}

impl<T: FromStr + Debug> TryFromEnv for Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_vec(&value)
    }
}

impl<T: FromStr + Debug> TryFromEnv for HashMap<String, T>
where
    <T as FromStr>::Err: Debug,
{
    type Error = &'static str;

    fn try_from_env(value: &str) -> Result<Self, Self::Error> {
        rawconv::to_hashmap(&value)
    }
}

/// Read the env var "key", and convert to type T. If the env
/// var has not been set, "default" will be used. If the env
/// var (or the default value) fail to parse correctly to
/// type T, panic.
pub fn biodome<T: TryFromEnv>(key: &str, default: T) -> T
where
    <T as TryFromEnv>::Error: std::fmt::Debug,
{
    let opt = env::var(key).ok();
    if let Some(v) = opt {
        T::try_from_env(&v).expect("Failed to parse")
    } else {
        default
    }
}

pub fn biodome_callable<T: TryFromEnv + Copy>(key: &str, default: T) -> impl Fn() -> T
where
    <T as TryFromEnv>::Error: std::fmt::Debug,
{
    let key = key.to_string();
    move || {
        let opt = env::var(key.clone()).ok();
        if let Some(v) = opt {
            T::try_from_env(&v).expect("Failed to parse")
        } else {
            default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    #[test]
    fn value_as_types_hashmap() {
        // Test data
        let tuples = vec![("XX".to_string(), 1), ("YY".to_string(), 2)];
        let original = HashMap::from_iter(tuples.clone());
        let default = original.clone();
        let expected = default.clone();
        // Check for the variable, fall back to the default.
        let got = biodome("TAHM", default);
        assert_eq!(got, expected);

        // Now populated a real variable
        env::set_var("TAHM", "{XX=3,YY=4}");
        let tuples = vec![("XX".to_string(), 3), ("YY".to_string(), 4)];
        let expected = HashMap::from_iter(tuples.clone());
        let default = original.clone();
        let got = biodome("TAHM", default);
        assert_eq!(got, expected);
    }

    #[test]
    fn values_as_types() {
        assert_eq!(biodome("ABC", "123".to_string()), "123");

        assert_eq!(biodome("ABC", 123_i8), 123_i8);
        assert_eq!(biodome("ABC", 123_i16), 123_i16);
        assert_eq!(biodome("ABC", 123_u8), 123_u8);
        assert_eq!(biodome("ABC", 123_u16), 123_u16);

        assert_eq!(biodome("ABC", 123_i32), 123_i32);
        assert_eq!(biodome("ABC", 123_i64), 123_i64);
        assert_eq!(biodome("ABC", 123_u32), 123_u32);
        assert_eq!(biodome("ABC", 123_u64), 123_u64);

        assert_eq!(biodome("ABC", 123_f32), 123_f32);
        assert_eq!(biodome("ABC", 123_f64), 123_f64);

        let v = biodome("XYZ", vec![1, 2, 3]);
        assert_eq!(v, vec![1, 2, 3]);
        env::set_var("XYZ", "[4, 5, 6]");
        let v = biodome("XYZ", vec![1, 2, 3]);
        assert_eq!(v, vec![4, 5, 6]);
    }

    #[test]
    fn basic() {
        struct Settings {
            NUM_THREADS: usize,
            DELAY: f32,
            PRECISON: f64,
            LOG_LEVEL: String,
        }

        impl Default for Settings {
            fn default() -> Settings {
                Settings {
                    NUM_THREADS: biodome("NUM_THREADS", 8),
                    DELAY: biodome("DELAY", 1.23),
                    PRECISON: biodome("PRECISON", 1.23_f64),
                    LOG_LEVEL: biodome("LOG_LEVEL", "info".to_string()),
                }
            }
        }

        let settings = Settings::default();
        assert_eq!(settings.NUM_THREADS, 8);
        assert!((settings.DELAY - 1.23).abs() < 1e-8);
        assert!((settings.PRECISON - 1.23).abs() < 1e-8);
        assert_eq!(settings.LOG_LEVEL, "info");
    }

    #[test]
    fn stat() {
        let mut NT = biodome("NT", 8);
        assert_eq!(NT, 8);
        env::set_var("NT", "16");
        NT = biodome("NT", 8);
        assert_eq!(NT, 16);
        env::remove_var("NT");
    }

    #[test]
    fn callables() {
        let NUM_THREADS = biodome_callable("NUM_THREADS", 8);
        assert_eq!(NUM_THREADS(), 8);
        env::set_var("NUM_THREADS", "16");
        assert_eq!(NUM_THREADS(), 16);
        env::remove_var("NUM_THREADS");
    }
}
