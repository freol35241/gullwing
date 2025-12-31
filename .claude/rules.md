# Claude Agent Rules for gullwing

These rules guide AI agent behavior when working on this codebase.

## Code Quality Rules

### MUST Follow
1. **No unsafe code** - The crate uses `#![forbid(unsafe_code)]`. Never add unsafe blocks.
2. **All clippy warnings must pass** - Run `cargo clippy --all-targets -- -D warnings` before committing.
3. **Format with rustfmt** - Run `cargo fmt` before committing.
4. **All tests must pass** - Run `cargo test` and ensure 100% pass rate.
5. **Doc tests must compile** - All `///` examples must be valid, runnable code.

### Error Handling
1. **Never use `.unwrap()` without a guard** - If you must unwrap, add an `if` check or use `unwrap_or_*`.
2. **Return `Result<T, Error>`** - All fallible operations use the crate's Result type.
3. **Map external errors** - Convert regex or parse errors to `Error::*` variants.
4. **Provide context** - Error messages should explain what went wrong.

### Documentation
1. **Document all public items** - Every `pub` function/struct/enum needs `///` docs.
2. **Include examples** - Complex APIs need `# Examples` sections.
3. **Keep README in sync** - If you change the API, update README.md examples.

## Testing Rules

### When to Add Tests
1. **New feature** → Add unit test + integration test
2. **Bug fix** → Add regression test that would have caught the bug
3. **Error path** → Add test in `tests/error_cases.rs`
4. **Public API change** → Update or add doc tests

### Test Naming
- Unit tests: `test_<function_name>_<scenario>`
- Integration tests: `test_<feature>_<behavior>`

### Property-Based Tests
When adding numeric formatting/parsing, consider adding proptest roundtrip tests in `tests/roundtrip.rs`.

## Commit Rules

### Commit Message Format
```
<type>: <short description>

[optional body]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Adding tests
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `chore`: Build process, dependencies, etc.

### Before Committing
1. Run `cargo fmt`
2. Run `cargo clippy --all-targets`
3. Run `cargo test`
4. Run `cargo doc --no-deps`

## Architecture Rules

### Adding New Type Specifiers
Follow this checklist:
1. Add variant to `TypeSpec` in `src/spec/types.rs`
2. Implement `from_char()` and `to_char()`
3. Add formatting in `src/format/writer.rs`
4. Add regex pattern in `src/parse/builder.rs`
5. Add conversion in `src/parse/matcher.rs`
6. Add tests for formatting and parsing
7. Update README.md type specifier table

### Changing Public API
1. Ensure backward compatibility where possible
2. Update all doc examples
3. Update README.md
4. Add CHANGELOG.md entry
5. Consider semver implications

## Performance Rules

1. **Don't regress benchmarks** - Run `cargo bench` and compare to baseline
2. **Reuse Formatter/Parser** - These compile patterns; reuse instances
3. **Avoid unnecessary allocations** - Use references where possible
4. **Profile before optimizing** - Don't guess at bottlenecks

## Security Rules

1. **Validate all input** - Format strings and text input must be validated
2. **No panic in production paths** - Only panic in unreachable code or tests
3. **Limit regex complexity** - Complex patterns could cause ReDoS
4. **Report vulnerabilities privately** - See SECURITY.md

## Pull Request Rules

1. **One logical change per PR** - Don't mix features with refactoring
2. **Include tests** - PRs without tests need justification
3. **Update documentation** - API changes need doc updates
4. **Pass CI** - All GitHub Actions checks must pass
5. **Describe the change** - PR description should explain what and why
