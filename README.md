# Binarch2
`binarch2` is a tool designed to determine the architecture and endianness of a given input binary. It works by searching a binary using regular expressions defining common patterns found in function prologues and epilogues. It supports MIPS, PowerPC, ARM and x86_64.

# Usage
```bash
$ binarch --help
Tool for identifying architecture and endianness of binary files

Usage: binarch [OPTIONS] <INPUT>

Arguments:
  <INPUT>
          Input file

Options:
  -l, --log-level <LOG_LEVEL>
          Log level

          [default: info]
          [possible values: off, error, warn, info, debug, trace]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

# Example
```bash
$ binarch /bin/ls
[INFO ] X86 Little
[INFO ] DONE
````
