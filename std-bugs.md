# Std Bugs

List of bugs found in the standard library that are not handled by the PoC structure yet.

## 0-day

* API soundness issue in join() implementation of [Borrow<str>] #80335
* Heap buffer overflow in `read_to_end_with_reservation()` #80894

## 1-day

* String::retain allows safely creating invalid (non-utf8) strings when abusing panic #78498
* Double drop in Vec::drain_filter #60977
    * It was nightly only feature at the time of finding

## Manual

* Soundness issue in `Zip::next()` specialization #81740
