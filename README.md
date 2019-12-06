# args-to-json
Lots of debuggers accept the command to run and its arguments as a JSON array. Writing that array by reformatting from your shell's syntax to JSON is annoying.

args-to-json is a small, portable (any OS, any shell) tool that reads in all arguments and prints them out as a list of strings in JSON.

args-to-json works well with tools like `clip` or `xclip` too -- to copy your last command as JSON in Bash onto your clipboard, run

```bash
args-to-json !! | xclip
```

The contents of your clipboard can then be pasted directly into a document.

## Installation

### Windows
Binaries are available from the [Releases](https://github.com/LPGhatguy/args-to-json/releases) section on GitHub.

### With Rust
Install from crates.io:

```bash
cargo install args-to-json
```

## Detailed Usage
```bash
args-to-json <...args>
```

## License
args-to-json is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.