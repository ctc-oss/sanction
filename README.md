sanction
===
apply allowlist to [grype](https://github.com/anchore/grype) scans

### usage

```
sanction v0.1.0
Basic allowlisting and formatting for grype scans

USAGE:
    sanction [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --allowlist <allowlist>    Path to allowlist [default: allow.txt]
    -o, --output <output>          Output mode [default: remove]
    -s, --severity <severity>      Minimum severity
```

### examples

```
S grype centos:7 -o json | sanction -l test/data/allow.txt -o md -s high
 ✔ Vulnerability DB     [no update available]
 ✔ Loaded image         
 ✔ Parsed image         
 ✔ Cataloged image      [158 packages]
 ✔ Scanned image        [660 vulnerabilities]
| A | category | package | cve | fix |
|---|----------|---------|-----|-----|
|  | High | dbus-1.10.24-14.el7_8 | [CVE-2019-12749](https://access.redhat.com/security/cve/CVE-2019-12749) | 1:1.10.24-15.el7 |
|  | High | openssl-libs-1.0.2k-19.el7 | [CVE-2020-1971](https://access.redhat.com/security/cve/CVE-2020-1971) | 1:1.0.2k-21.el7_9 |
| X | High | sqlite-3.7.17-8.el7_7.1 | [CVE-2019-5827](https://access.redhat.com/security/cve/CVE-2019-5827) |  |
|  | High | glib2-2.56.1-5.el7 | [CVE-2016-3191](https://access.redhat.com/security/cve/CVE-2016-3191) |  |
| X | High | glib2-2.56.1-5.el7 | [CVE-2015-8385](https://access.redhat.com/security/cve/CVE-2015-8385) |  |
| X | High | glibc-2.17-307.el7.1 | [CVE-2019-25013](https://access.redhat.com/security/cve/CVE-2019-25013) |  |
| X | High | glibc-common-2.17-307.el7.1 | [CVE-2019-25013](https://access.redhat.com/security/cve/CVE-2019-25013) |  |
|  | High | dbus-libs-1.10.24-14.el7_8 | [CVE-2019-12749](https://access.redhat.com/security/cve/CVE-2019-12749) | 1:1.10.24-15.el7 |
```

| A | category | package | cve | fix |
|---|----------|---------|-----|-----|
|  | High | dbus-1.10.24-14.el7_8 | [CVE-2019-12749](https://access.redhat.com/security/cve/CVE-2019-12749) | 1:1.10.24-15.el7 |
|  | High | openssl-libs-1.0.2k-19.el7 | [CVE-2020-1971](https://access.redhat.com/security/cve/CVE-2020-1971) | 1:1.0.2k-21.el7_9 |
| X | High | sqlite-3.7.17-8.el7_7.1 | [CVE-2019-5827](https://access.redhat.com/security/cve/CVE-2019-5827) |  |
|  | High | glib2-2.56.1-5.el7 | [CVE-2016-3191](https://access.redhat.com/security/cve/CVE-2016-3191) |  |
| X | High | glib2-2.56.1-5.el7 | [CVE-2015-8385](https://access.redhat.com/security/cve/CVE-2015-8385) |  |
| X | High | glibc-2.17-307.el7.1 | [CVE-2019-25013](https://access.redhat.com/security/cve/CVE-2019-25013) |  |
| X | High | glibc-common-2.17-307.el7.1 | [CVE-2019-25013](https://access.redhat.com/security/cve/CVE-2019-25013) |  |
|  | High | dbus-libs-1.10.24-14.el7_8 | [CVE-2019-12749](https://access.redhat.com/security/cve/CVE-2019-12749) | 1:1.10.24-15.el7 |



### intiial mvp
- [X] parse grype input
- [X] read allowlist
- [X] filter vulnerabilities
- [X] output markdown
- [ ] output grype schema
- [ ] apply tags to grype schema
