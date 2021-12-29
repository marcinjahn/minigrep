# minigrep

A simplified grep implementation in Rust based on the Rust book

## Usage

```sh
cargo run <query> <file_path> [-c]
```

- `<query>` - the text that you're looking for
- `<file_path>` - a path to the file minigrep should look for the query in
- `-c` - optional parameter; when it's set, the search is case-insensitive.
  The same option can be set with the `CASE_INSENSITIVE` environment
  variable (set to any value).
