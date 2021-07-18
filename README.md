# biodome

*Easier access to environment variables*

*biodome* does two things:
1. Automatically cast env vars to the "right" types.
2. Automatically parse structured types from env vars.

This crate is a rust implementation of a
[similar](https://github.com/cjrh/biodome) library I
made for Python several years ago. I used it mainly
for microservices, which I am now starting to build
in Rust also.

## Demo

This reads an environment variable called `TIMEOUT`:

```rust
use biodome::biodome;

let TIMEOUT = biodome("TIMEOUT", 10);
let PORTS = biodome("PORTS", vec![8081, 8082, 8083]);
```

Consider the `TIMEOUT` identifier:

- If the env var has not been set, the default value of `10`
  will be used, i.e., assigned to `TIMEOUT`.
- If the env var has been set, it will automatically be
  converted to the correct type, and then assigned.
- The type of the identifier `TIMEOUT` will always be the
  same as the type of the default value.

Consider the `PORTS` identifier:

- The default value is a `Vec<i32>`: this means that the
  type`PORTS` will always be a `Vec<i32>`.
- *biodome* will parse the env var, if set, to make that
  happen. An env var like this (bash) would
  work: `export PORTS=[81, 82]`

## Simple Types

In the above example, the literal integer `10` is of type
`i32` by default. For most primitive types this can be
controlled in an obvious way:

```rust
use biodome::biodome;

let TIMEOUT = biodome("TIMEOUT", 10u8);
let TIMEOUT = biodome("TIMEOUT", 10f64);
```

In the first line above example, `TIMEOUT` will be a `u8`, and any
env var value will have to set appropriately or a runtime
error will occur. Likewise, if the default is an `f64`, then `TIMEOUT`
will be an `f64`.

Boolean values are handled a little differently than for parsing:

```rust
use biodome::biodome;

/// This line sets an environment variable, same as
/// if you have done `export DEBUG=yes` in Bash.
std::env::set_var("DEBUG", "yes");

/// This line reads the value of the env var in Rust.
/// Because the default value is a bool, it means that
/// biodome will attempt to convert the value of the
/// env var into a bool.
let DEBUG = biodome("DEBUG", false);

assert_eq!(DEBUG, true);
```

If the env var has been set to a wide range of "probably truthy"
patterns, the result will be `true`; otherwise, `false`. Some
of these values are (case-insensitively) `true`, `t`, `yes`, `y`,
`on`, `active`, `enabled`, `1`, `ok` and so on.

## Structured Types

If all *biodome* did was cast primitive types, it would be
mildly interesting. We also have support for more structured
types. To support this, we're parsing all structured types
using a limited subset of the [TOML](https://toml.io/en/v1.0.0)
markup format.

Imagine that the following 3 env vars are set:

```bash
export LOGLEVELS='{ root = "warn", http = "info" }'
export TIMEOUTS='{ connect = 5.0, request = 10.0 }'
export PROXIES='["a.proxy.com:8000", "b.proxy.com:8001"]'
```

In the above, `LOGLEVELS` and `TIMEOUTS` are formatted
as [TOML inline tables](https://toml.io/en/v1.0.0#inline-table)
while `PROXIES` is formatted as
a [TOML array](https://toml.io/en/v1.0.0#array).

These can be accessed with *biodome* like this:

```rust
use biodome::biodome;
use std::collections::HashMap;
use std::iter::FromIterator;

/// Create the default values for the structured types
let default_loglevels = HashMap::from_iter(
    vec![(String::from("root"), String::from("info"))]
);

let default_timeouts = HashMap::from_iter(
    vec![
        (String::from("resolve"), 1.0),
        (String::from("connect"), 1.0),
        (String::from("request"), 1.0),
    ]
);

let default_proxies = vec![
    "dev.proxy.com:9009".to_string(),
];

/// Read the env vars
let LOGLEVELS = biodome("LOGLEVELS", default_loglevels);
let TIMEOUTS = biodome("TIMEOUTS", default_timeouts);
let PROXIES = biodome("TIMEOUTS", default_proxies);

```
In the above example, `LOGLEVELS` will be a `HashMap<String, String>`,
`TIMEOUTS` will be a `HashMap<String, f64>`, and `PROXIES` will be
a `Vec<String>`.

## Alternative Projects

[envy](https://github.com/softprops/envy) uses the power of
[Serde derive](https://serde.rs/derive.html) to work magic in populating
a "settings" struct.

## Developer Info

This README is generated with
[cargo-readme](https://github.com/livioribeiro/cargo-readme).
Please follow its instructions on how to set it up. The README
file can be regenerated with `cargo readme > README.md`.
