# PolicyCode Framework

The PolicyCode framework is inspired by:

- https://en.wikipedia.org/wiki/Geek_Code
- https://nvd.nist.gov/vuln-metrics/cvss/v3-calculator

## PolicyCode Philosophy

- KISS - Keep it Super Simple and keep making it simpler (as possible)
- a single line and short as possible
- Source of Truth is implemented in Code in this repository
- Express the specifics in the code and leave the high level in spec
- Use new alphanumerics if the existing meaning changes
- Alphanumeric can be either Value Assignment OR Self-Expression
- No specs for anything that is not needed Now(tm)
- Don't be afraid of breaking changes and use SemVer
- Don't be afraid of the Regex - be expressive with it - less words more regex
- Make something up in the meanwhile - rather than analysis-paralysis
- Don't sweat it.

## PolicyCode Parsing 0.1.0

K/V Assignments A=foo, A=foo+bar, A=foo%20bar (RFC 3986), A="url" (RFC 1738), A=TIMEexp
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
