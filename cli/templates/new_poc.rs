/*!
```rudra-poc
[target]
crate = "{{ krate }}"
version = "{{ version }}"

[report]

[[bugs]]
# UnsafeDestructor, SendSyncVariance, UnsafeDataflow
analyzer = "Manual"
# SendSyncVariance, UninitExposure, InconsistencyAmplification, PanicSafety
bug_class = "Other"
# `location` field from Rudra report or empty if Manual.
rudra_report_locations = []
```
!*/
#![forbid(unsafe_code)]

fn main() {
    println!("Hello, World!")
}
