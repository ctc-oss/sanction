sanction
===
apply allowlist to [grype](https://github.com/anchore/grype) scans

### usage

```
$ sanction --help
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

### examples

```
$ grype centos:7 -o json | sanction -l test/data/allow.txt
```

### intiial mvp
- [X] parse grype input
- [X] read allowlist
- [X] filter vulnerabilities
- [ ] output grype schema
- [ ] apply tags to grype schema
