# weather
PCT CIT368 - get weather forecast from command line. Learning some Rust in the process.

This expects a Config.toml file in the project root containing the line:

```weather_api_key = "your api key"```

This will also create an activity log `log.txt` in the project root that resets at 20kb. An example follows:

```bash
2024-04-29 11:28:11: Startup.
2024-04-29 11:28:11: Missing or invalid zip, defaulted to 17701.
2024-04-29 11:28:12: Current weather data retrieved.
2024-04-29 11:28:12: Forecast data retrieved.
2024-04-29 11:28:12: Shutdown.
2024-04-29 11:28:23: Startup.
2024-04-29 11:28:23: Current weather data retrieved.
2024-04-29 11:28:23: Error fetching forecast data: error decoding response body: missing field `lat` at line 1 column 16190
2024-04-29 11:28:23: Shutdown.
```


Expects a 5 digit leading-zero zip code. Usage: 

```bash

$ weather 07001

Current weather conditions for Avenel:

Temperature:    84.47 °F
Feels like:     86.7 °F
Description:    scattered clouds

The 3 day forecast:

Today    18:00:00        84.47°F 🌥️      scattered clouds
Today    21:00:00        83.86°F 🌥️      scattered clouds
 
Tue      00:00:00        74.80°F 🌧️      light rain
Tue      03:00:00        59.63°F 🌥️      scattered clouds
Tue      06:00:00        55.81°F 🌥️      broken clouds
Tue      09:00:00        55.69°F 🌧️      light rain
Tue      12:00:00        55.60°F 🌧️      light rain
Tue      15:00:00        57.25°F 🌥️      overcast clouds
Tue      18:00:00        57.58°F 🌥️      overcast clouds
Tue      21:00:00        57.97°F 🌥️      overcast clouds
 
Wed      00:00:00        56.08°F 🌥️      overcast clouds
Wed      03:00:00        55.45°F 🌥️      overcast clouds
Wed      06:00:00        54.72°F 🌥️      overcast clouds
Wed      09:00:00        54.10°F 🌥️      overcast clouds
Wed      12:00:00        56.03°F 🌧️      light rain
Wed      15:00:00        57.00°F 🌥️      overcast clouds
Wed      18:00:00        65.50°F 🌥️      overcast clouds
Wed      21:00:00        64.20°F 🌥️      broken clouds
 
Thu      00:00:00        58.93°F 🌥️      broken clouds
Thu      03:00:00        54.43°F 🌞     clear sky
Thu      06:00:00        52.05°F 🌥️      few clouds
Thu      09:00:00        50.76°F 🌥️      scattered clouds
Thu      12:00:00        52.81°F 🌥️      few clouds
Thu      15:00:00        59.88°F 🌥️      scattered clouds
Thu      18:00:00        64.29°F 🌥️      scattered clouds
Thu      21:00:00        63.39°F 🌥️      overcast clouds

```

Dependencies:

```toml
chrono = "0.4.33"
jsonschema = "0.17.1"
regex = "1.10.4"
reqwest = {version = "0.11.23", features = ["blocking", "json"] }
serde = {version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
toml = "0.8.8"

```

There is currently test coverage for:
- valid zip
- invalid zip
- missing zip
- geocoding from api
- weather retrieval

```bash
(base) ➜  weather git:(secure) ✗ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/main.rs (target/debug/deps/weather..)

running 5 tests
test tests::test_missing_zip ... ok
test tests::test_valid_zip ... ok
test tests::test_invalid_zip ... ok
test tests::test_fetch_coords ... ok
test tests::test_fetch_weather ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.40s
```

[Cargo-audit](https://docs.rs/cargo-audit/latest/cargo_audit/) was used in a pre-commit git hook and at the time of this commit there are a couple dependency issues to note:

```bash
(base) ➜  weather git:(secure) ✗ git commit -m "test commit"
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 623 security advisories (from /Users/tommahoney/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (189 crate dependencies)
Crate:     h2
Version:   0.3.24
Title:     Degradation of service in h2 servers with CONTINUATION Flood
Date:      2024-04-03
ID:        RUSTSEC-2024-0332
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0332
Solution:  Upgrade to ^0.3.26 OR >=0.4.4
Dependency tree:
h2 0.3.24
├── reqwest 0.11.23
│   ├── weather 0.1.0
│   └── jsonschema 0.17.1
│       └── weather 0.1.0
└── hyper 0.14.28
    ├── reqwest 0.11.23
    └── hyper-tls 0.5.0
        └── reqwest 0.11.23

Crate:     mio
Version:   0.8.10
Title:     Tokens for named pipes may be delivered after deregistration
Date:      2024-03-04
ID:        RUSTSEC-2024-0019
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0019
Solution:  Upgrade to >=0.8.11
Dependency tree:
mio 0.8.10
└── tokio 1.35.1
    ├── tokio-util 0.7.10
    │   └── h2 0.3.24
    │       ├── reqwest 0.11.23
    │       │   ├── weather 0.1.0
    │       │   └── jsonschema 0.17.1
    │       │       └── weather 0.1.0
    │       └── hyper 0.14.28
    │           ├── reqwest 0.11.23
    │           └── hyper-tls 0.5.0
    │               └── reqwest 0.11.23
    ├── tokio-native-tls 0.3.1
    │   ├── reqwest 0.11.23
    │   └── hyper-tls 0.5.0
    ├── reqwest 0.11.23
    ├── hyper-tls 0.5.0
    ├── hyper 0.14.28
    └── h2 0.3.24

Crate:     iana-time-zone
Version:   0.1.59
Warning:   yanked
Dependency tree:
iana-time-zone 0.1.59
└── chrono 0.4.33
    └── weather 0.1.0

error: 2 vulnerabilities found!
warning: 1 allowed warning found
```