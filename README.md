sanction
===
Apply allowlist to [grype](https://github.com/anchore/grype) scans

![alt](doc/img.png)

### usage

```
sanction v0.1.0
Basic allowlisting and formatting for grype scans

USAGE:
    sanction [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --pretty     Pretty printing for markdown and json
    -V, --version    Prints version information

OPTIONS:
    -l, --allowlist <allowlist>    Path to allowlist [default: allow.txt]
    -m, --mode <mode>              Allow mode [default: tag]
    -o, --output <output>          Output mode [default: json]
    -s, --severity <severity>      Minimum severity
```

### intiial mvp
- [X] parse grype input
- [X] read allowlist
- [X] filter vulnerabilities
- [X] output markdown
- [ ] output grype schema
- [ ] apply tags to grype schema
