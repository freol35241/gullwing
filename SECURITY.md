# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.9.x   | :white_check_mark: |
| < 0.9   | :x:                |

## Security Considerations

gullwing is a parsing and formatting library that processes format strings and text input. Here are important security considerations:

### Input Validation

- **Format strings**: The library validates format specifications and rejects malformed patterns with descriptive errors
- **Text parsing**: Uses compiled regex patterns; invalid regex patterns are caught at `Parser::new()` time
- **No arbitrary code execution**: The library only performs string formatting and parsing operations

### Potential Risks

1. **Regex Denial of Service (ReDoS)**: While the library uses the `regex` crate which has good ReDoS protections, extremely complex format patterns with many fields could have performance implications. Consider limiting pattern complexity in untrusted contexts.

2. **Memory usage**: Parsing very long strings or creating formatters with many fields will allocate memory proportionally. The library does not impose limits on these.

3. **Integer overflow**: The library uses standard Rust integer types with proper error handling for conversions.

### Safe Practices

- The library uses `#![forbid(unsafe_code)]` - no unsafe Rust code is used
- All public APIs return `Result` types for proper error handling
- No panic paths in production code (all `unwrap()` calls are guarded)

## Reporting a Vulnerability

If you discover a security vulnerability in gullwing, please report it responsibly:

1. **Do NOT** open a public GitHub issue for security vulnerabilities
2. Email the maintainer directly at: freol@outlook.com
3. Include:
   - A description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Any suggested fixes (optional)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix timeline**: Depends on severity
  - Critical: Patch release within 48 hours
  - High: Patch release within 1 week
  - Medium/Low: Next regular release

### Disclosure Policy

- We will coordinate with you on disclosure timing
- Credit will be given in the release notes (unless you prefer anonymity)
- We follow responsible disclosure practices

## Security Updates

Security updates will be announced via:
- GitHub Security Advisories
- CHANGELOG.md entries marked with `[SECURITY]`
- Release notes on crates.io
