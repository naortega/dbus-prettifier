# DBus Prettifier

I noticed that when working with `qdbus` that I would get long and unformatted
outputs that are extremely difficult to read. This small program will prettify a
file with the output so that it is much  more readable.

## Usage

To run the program, simply run it as follows:

```console
$ qdbus-prettifier <in-file> <out-file>
```

## Building

Requirements:

- C compiler that supports GNU99 (GCC recommended).
- (GNU) Make

Build with `make`:

```console
$ make
cc  -std=gnu99 -Wall -Wextra -Wfatal-errors -Werror -O2 -DNDEBUG   -c -o src/main.o src/main.c
cc -o dbus-prettifier src/main.o
```

Install with `make install`:

```console
$ sudo make install
install -Dm755 dbus-prettifier /usr/local/bin/
```

## License

This program is licensed under the terms & conditions of the [Zlib
License](LICENSE).
