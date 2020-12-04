# Rudra-PoC

This repository contains the list of memory safety and soundness bugs found during the Rudra project.

See [DEV.md](./DEV.md) for the technical detail.

| ID | Crate | Method | Issue URL | RustSec ID |
| -- | ----- | ------ | --------- | ---------- |
{% for line in lines -%}
| {{ line.poc_id }} | {{ line.krate }} | {{ line.analyzers|unordered_list }} | {{ line.issue_url|na }} | {{ line.rustsec_id|na }} |
{% endfor %}
