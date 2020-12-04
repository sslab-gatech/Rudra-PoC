# Rudra-PoC

This repository contains the list of memory safety and soundness bugs found during the Rudra project.

Contributors: See [REPORTING.md](./REPORTING.md) for the reporting guideline.

| ID | Crate | Method | Issue Report | RustSec ID |
| -- | ----- | ------ | ------------ | ---------- |
{% for line in lines -%}
| {{ line.poc_id }} | {{ line.krate }} | {{ line.analyzers|unordered_list }} | {{ line.issue_url|unwrap_or("N/A") }} | {{ line.rustsec_link|unwrap_or("Not Reported Yet") }} |
{% endfor %}
