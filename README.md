sanction
===
apply allowlist to [grype](https://github.com/anchore/grype) scans

```
#! ./target/debug/sanction --help
sanction v0.0.0
allowlisting for grype scans

USAGE:
    sanction [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --allowlist <allowlist>    Path to allowlist [default: allow.txt]
    -o, --output <output>          Output mode [default: remove]
```
