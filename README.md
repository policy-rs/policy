# PolicyCode Framework

The PolicyCode framework is inspired by:

- https://en.wikipedia.org/wiki/Geek_Code
- https://nvd.nist.gov/vuln-metrics/cvss/v3-calculator

Rust SecurityPolicy is specified and implemented first.

## PolicyCode Philosophy

See [POLICY.md](POLICY.md)

## PolicyCode Parsing 0.1.0

- This must be able to be embedded inside ""'s
- UTF-8 with " (d'34) character reserved - no quoting supported

K/V Assignments A=foo, A=foo+bar, A=foo%20bar (RFC 3986), A=url (RFC 1738), A=TIMEexp
```regex
^([a-zA-Z0-9]+)=([^\s]+|\"([^\"]+\")$
```

Self-Expressions A, A+, A-, A++, A--
```regex
^([a-zA-Z0-9]+)([\-]*|[\+]*)$
```

## Source of Truth

The Rust implementation and tests serves as the Source of Truth

$ cargo add policy

## Security PolicyCode

The initial need was to develop a PolicyCode for security policies.

### Rust SecurityPolicyCode

| Version | Full String Spec                      |
| :---    | :---                                  |
| 0.1.0   | 0.1.0 = "v=0.1.0 E=e@mail F  |

| Alpha      | Version | Definition |
| :--        | :---    | :---       |
| v          | 0.1.0   | SemVer Version of the SecurityPolicyCode      |
| E=..       | 0.1.0   | E-Mail for security correspondence            |
| F          | 0.1.0   | Neutral public Full Disclosure Disposition    |
| F-         | 0.1.0   | Negative public [Full Disclosure] Disposition |
| F+         | 0.1.0   | Positive public [Full Disclosure] Disposition |
| PuBP="url" | 0.1.0   | Public bountry program                        |
| PrBP="url" | 0.1.0   | Private bounty program                        |
| Ap=TIME    | 0.1.0   | Advance Private Disclosure program            |

[Full disclosure]: https://en.wikipedia.org/wiki/Full_disclosure_(computer_security)
