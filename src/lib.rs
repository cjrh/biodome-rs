#![allow(non_snake_case)]

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
