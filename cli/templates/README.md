# Rudra-PoC

This repository contains the list of memory safety and soundness bugs found in Rust
by [Rudra](https://github.com/sslab-gatech/Rudra) project.

You can find all new bugs found during the research under `poc/` directory.
All unreported but valid bugs are listed in `unreported/` directory (independently fixed, maintainers are already aware of the issue, etc.).
Note that this list includes manually found bugs and the bugs from earlier experimental pattern (UnsafeDestructor) that wasn't included in the Rudra paper.
These are not included in the number of bugs found by Rudra in the paper but left here for the completeness.

Analyzer
* M: Manual
* D: UnsafeDestructor
* SV: SendSyncVariance
* UD: UnsafeDataflow

Bug Class
* SV: SendSyncVariance
* UE: UninitExposure
* HO: HigherOrderInvariant
* PS: PanicSafety
* O: Other

| ID | Crate | Bugs | Issue Report | RustSec ID |
| -- | ----- | ---- | ------------ | ---------- |
{% for line in lines -%}
| {{ line.poc_id }} | {{ line.krate }} | {{ line.bugs|bug_join }} | {{ line.issue_url|unwrap_or("N/A") }} | {{ line.rustsec_link|unwrap_or("Not Reported Yet") }} |
{% endfor %}
