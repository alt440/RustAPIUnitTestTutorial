# Automated tests in Rust
Working with integration tests present some differences from unit tests:

  1. There is no need to use #[cfg(test)] because integration tests are always ran in a testing context
  2. Each integration test file gets compiled as a separate crate, which can have a negative impact on compile times for tests. Grouping similar tests in a single file can help reduce this impact
  3. Subdirectories in tests/ get ignored and aren't built as integration tests. This means all integration tests must be present at the root of the tests/ directory
(source: https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/#Integration-testing )

An integration test has been added under tests/integration_test.rs, and a unit test has been added under src/jwt/mod.rs as examples.
