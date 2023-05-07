# Common issues when writing lint config files

## Issue #1
**`Failed to parse config file`/`invalid escape sequence` errors on writing Regex-based lint rules:**
Use single quotes (`'`) instead of double quotes (`"`) for delimiting Regex rules.

```toml
[lint_example]
example_rule = { regex = "^\w{6}$" } # Incorrect delimiters
```

```toml
[lint_example]
example_rule = { regex = '^\w{6}$' } # Correct delimiters
```
