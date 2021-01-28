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
```
!*/
#![forbid(unsafe_code)]

fn main() {
    println!("Hello, World!")
}
