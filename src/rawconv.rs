use std::fmt::Debug;
use std::str::FromStr;
use std::vec::Vec;
use toml::value::Datetime;

pub fn to_prim<T: FromStr>(s: &str) -> Result<T, &'static str> {
    s.parse().map_err(|err| "parse error")
}

pub fn to_bool(s: &str) -> bool {
    const TRUTHY_VALUES: [&str; 10] = [
        "true", "t", "1", "yes", "y", "ok", "enable", "enabled", "active", "on",
    ];
    TRUTHY_VALUES.iter().any(|&v| v == &s.trim().to_lowercase())
}

pub fn to_vec<T: FromStr>(s: &str) -> Result<Vec<T>, &'static str>
where
    <T as FromStr>::Err: Debug,
{
    let s = format!("x = {}", s);
    let out = s.parse::<toml::Value>().unwrap();
    let out = out["x"].as_array().unwrap();
    let out = out
        .iter()
        // .map(|v| v.as_integer().unwrap())
        .map(|v| v.to_string().parse().unwrap())
        .collect();

    Ok(out)
}

pub fn to_datetime(s: &str) -> Result<Datetime, &'static str> {
    s.parse().map_err(|err| "parse error")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prim() -> Result<(), &'static str> {
        let x: i32 = to_prim("1")?;
        assert_eq!(x, 1);
        let x: i64 = to_prim("1")?;
        assert_eq!(x, 1);
        let x: f32 = to_prim("1")?;
        assert_eq!(x, 1_f32);
        let x: f32 = to_prim("1.0")?;
        assert_eq!(x, 1_f32);
        let x: f64 = to_prim("1")?;
        assert_eq!(x, 1_f64);
        let x: f64 = to_prim("1.0")?;
        assert_eq!(x, 1_f64);
        Ok(())
    }

    #[test]
    fn boo() -> Result<(), &'static str> {
        let x = to_bool("1");
        assert_eq!(x, true);
        let x = to_bool("0");
        assert_eq!(x, false);
        let x = to_bool("true");
        assert_eq!(x, true);
        let x = to_bool("TRUE");
        assert_eq!(x, true);
        Ok(())
    }

    #[test]
    fn vecc() -> Result<(), &'static str> {
        let x: Vec<i32> = to_vec("[1, 2, 3]")?;
        assert_eq!(x, vec![1, 2, 3]);
        let x: Vec<i64> = to_vec("[1, 2, 3]")?;
        assert_eq!(x, vec![1, 2, 3]);
        let x: Vec<f32> = to_vec("[1, 2, 3]")?;
        assert_eq!(x, vec![1_f32, 2_f32, 3_f32]);
        let x: Vec<f64> = to_vec("[1, 2, 3]")?;
        assert_eq!(x, vec![1_f64, 2_f64, 3_f64]);
        Ok(())
    }

    #[test]
    fn dt() -> Result<(), &'static str> {
        let x = to_datetime("1979-05-27T07:32:00-08:00")?;
        let dt = Datetime::from_str("1979-05-27T07:32:00-08:00").unwrap();
        assert_eq!(x, dt);
        Ok(())
    }
}
