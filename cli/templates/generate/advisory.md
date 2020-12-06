```toml
# Remove all comments before submission!
[advisory]
id = "RUSTSEC-0000-0000"
package = "{{ krate }}"
date = "{{ original_issue_date }}"
{%- if original_issue_url.is_some() %}
url = "{{ original_issue_url.as_ref().unwrap() }}"
{%- endif %}
# Optional: Categories this advisory falls under. Valid categories are:
# "code-execution", "crypto-failure", "denial-of-service", "file-disclosure"
# "format-injection", "memory-corruption", "memory-exposure", "privilege-escalation"
categories = ["memory-corruption"]

[versions]
# Versions which include fixes for this vulnerability
patched = []
# Versions which were never vulnerable (optional)
# unaffected = ["< 1.1.0"]

# Optional: metadata which narrows the scope of what this advisory affects
# Note: qualified paths are not supported yet - https://github.com/RustSec/advisory-db/issues/512
[affected]
functions = { "crate_name::MyStruct::vulnerable_fn" = ["< 1.2.3, >= 1.2.0"] }
```

# {{ original_issue_title }}

((Replace this example text with the issue summary))

Affected versions of this crate did not properly check for integer overflow when allocating a buffer in `MyBuffer::with_capacity()` (bug description/location/root cause).

This can result in a memory corruption (consequence of the bug) when large integer is given to the parameter (trigger condition).

The flaw was corrected in commit abc123 by using `saturating_mul()` when calculating the buffer size (fix description).
