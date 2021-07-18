use biodome::biodome;
use std::env;
use uuid::Uuid;

fn new_varname() -> String {
    Uuid::new_v4().to_string()[..8].to_string()
}

#[test]
fn test_basic() {
    let varname = new_varname();
    let x = biodome(&varname, 123);
    assert_eq!(x, 123);
    env::set_var(&varname, "456");
    let x = biodome(&varname, 123);
    assert_eq!(x, 456);
}

#[test]
fn test_vec() {
    let varname = new_varname();
    let x = biodome(&varname, vec![1, 2, 3]);
    assert_eq!(x, vec![1, 2, 3]);
    env::set_var(&varname, "[4, 5, 6]");
    let x = biodome(&varname, vec![1, 2, 3]);
    assert_eq!(x, vec![4, 5, 6]);
}

#[test]
fn test_bool() {
    let varname = new_varname();
    let x = biodome(&varname, true);
    assert_eq!(x, true);
    env::set_var(&varname, "0");
    let x = biodome(&varname, true);
    assert_eq!(x, false);
    env::set_var(&varname, "enabled");
    let x = biodome(&varname, true);
    assert_eq!(x, true);
}
