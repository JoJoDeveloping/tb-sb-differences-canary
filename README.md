# tb-sb-differences-canary

This repository collects code that has exhibits the difference between Tree and Stacked Borrows.
When its test suite is run under miri, different tests will fail depending on which aliasing model is used, and which flags are used with this aliasing model.

This crate is intended as a "sanity check" when running studies that compare different aliasing model across the wider Rust ecosystem.
By injecting this crate into the set of code to be tested, we can gain confidence that such tests are measuring the right thing.

PRs for other litmus tests welcome.
