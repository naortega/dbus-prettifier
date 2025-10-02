# DBus Prettifier

I noticed when working with `qdbus` that I would get long and unformatted
outputs that are extremely difficult to read. This small program will prettify
a file with the output so that it is much more readable.

## Usage

To run the program, simply run it as follows:

```console
qdbus-prettifier <in-file> [out-file]
```

If `<in-file>` is defined as `-` this tells the program to read from `stdin`
allowing input to be piped. If the `[out-file]` option is omitted then output is
sent to `stdout`.

For more information, check out the `-h` option.

## Building & Installation

To compile the program, install the Rust suite, namely the compiler and Cargo.
Once installed you can run `cargo build --release` to generate a release build
of the project which will be found at `target/release/dbus-prettifier`.

To install the program you can use the `install` script. By default this will
install to `/usr/local`, but this can be changed by setting the `PREFIX`
variable prior to the command. For example:

```console
PREFIX=/usr ./install
```

Similarly, to uninstall simply run the `uninstall` script. It uses the same
`PREFIX` variable.

## License

This program is licensed under the terms & conditions of the [Zlib
License](LICENSE).
