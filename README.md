
# Weather
Weather CLI for Windows/Linux/macOS, which is responsible for showing weather to a user. The target consumer of the CLI is a human.

Currently next providers are supported:

- https://openweathermap.org/
- https://www.weatherapi.com/

To use weather cli application you need an API key, which you could receive after registering on provider website.

## CI/CD

Latest release CI/CD status

Platform | CI Status
---------|:---------
OSX      | [![Release](https://github.com/andrewDubyk/weather/actions/workflows/release.yaml/badge.svg)](https://github.com/andrewDubyk/weather/actions/workflows/release.yaml)
Linux    | [![Release](https://github.com/andrewDubyk/weather/actions/workflows/release.yaml/badge.svg)](https://github.com/andrewDubyk/weather/actions/workflows/release.yaml)
Windows  | [![Release](https://github.com/andrewDubyk/weather/actions/workflows/release.yaml/badge.svg)](https://github.com/andrewDubyk/weather/actions/workflows/release.yaml)

For more details check https://github.com/andrewDubyk/weather/actions

## Technical details overview

The whole project consts of next modules:

- `configuration` - handle user defined configurations (configure provider, change provider, set API_KEY)
- `api` - responsible for access to weather API providers.
    - Thers is a common `Trait` which should be implemented for all privders;
    - Currently all providers use HTTP REST API for requests;

Basic provider functionality is covered withunit tests. [codecov.io](https://codecov.io/) used for coverage.

## Limitations

For now only getting of current weather is supported by both providers. Forecasting for hourly/daily/weekly will be added in future.

---

## Usage

### Workflow
1. Install `rust` toolchain - https://www.rust-lang.org/tools/install
2. Download repository:
```
git clone https://github.com/andrewDubyk/weather.git
```

3. Build project:
```
cargo build --release
```

4. Configure provider:
```
cargo run weather configure <provider name> <api_key> 
```
5. Retrieve weather information:
```
cargo get London
```

### CLI

- `weather get`

```
Show weather for specified <address>

USAGE:
    weather get <address> [date]

ARGS:
    <address>    Address where to get weather information
    <date>       Show weather for the address at given date

OPTIONS:
    -h, --help    Print help information
```

- `weather configure`

```
Configure credentials for <provider>, where <provider> is some short name for a concrete weather API

USAGE:
    weather configure <provider> [api_key]

ARGS:
    <provider>    Set provider name [possible values: openweather, weatherapi]
    <api_key>     Set provider API key

OPTIONS:
    -h, --help    Print help information
```

## Documentation

Use `cargo doc --open` to generate project documentation.
