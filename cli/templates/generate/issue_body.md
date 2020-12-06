## Issue Description

((Fill issue description here))

## Reproduction

Below is an example program that exhibits undefined behavior using safe APIs of `{{ data.krate }}`.

((Optional description about the PoC construction))

```rust
{{ data.poc_code }}
```

Output:
```
{{ data.poc_output }}
```

### Tested Environment

* Crate: {{ data.krate }}
* Version: {{ data.version }}
* OS: {{ data.os_version }}
* Rustc version: {{ data.rustc_version }}
{%- if !data.cargo_flags.is_empty() %}
* Cargo flags: {{ data.cargo_flags|join(" ") }}
{%- endif -%}
{%- if !data.peer_dependencies.is_empty() %}
* 3rd party dependencies:
{%- for peer in data.peer_dependencies %}
  * `{{ peer }}`
{%- endfor -%}
{%- endif -%}
