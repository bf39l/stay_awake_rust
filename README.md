# Stay Awake
Simple Rust project to keep your (company) laptop awake

The project is inspired by https://github.com/Johnson468/Stay-Awake

## Build the project
### Prerequisite
install rust by following instructions in rust website: https://www.rust-lang.org/tools/install

### Build CMD
```bash
cargo build -r # -r for release
```

## Usage
```bash
# by default the program will awake your device by every 3 minutes
./stay_awake_rust

# you can change the sleeping time by passing in a number X
./stay_awake_rust 1 # awake your device by every 1 minute
```
