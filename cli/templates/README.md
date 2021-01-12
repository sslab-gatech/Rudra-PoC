# Rudra-PoC

This repository contains the list of memory safety and soundness bugs found during the Rudra project.

Contributors: See [REPORTING.md](./REPORTING.md) for the reporting guideline.

Method
* M: Manual
* D: UnsafeDestructor
* SV: SendSyncVariance
* UD: UnsafeDataflow

Bug Class
* SV: SendSyncVariance
* UE: UninitExposure
* IA: InconsistencyAmplification
* PS: PanicSafety
* O: Other

| ID | Crate | Method | Bug Class | Issue Report | RustSec ID |
| -- | ----- | ------ | --------- | ------------ | ---------- |
{% for line in lines -%}
| {{ line.poc_id }} | {{ line.krate }} | {{ line.analyzers|analyzer_join }} | {{ line.bug_classes|bug_class_join }} | {{ line.issue_url|unwrap_or("N/A") }} | {{ line.rustsec_link|unwrap_or("Not Reported Yet") }} |
{% endfor %}
