# Neo N3 FaaS Platform Tests

This directory contains unit tests for the Neo N3 FaaS platform.

## Test Structure

- `neo-source/`: Tests for Neo N3 blockchain integration
- `js-runtime/`: Tests for JavaScript runtime
- `oracle-services/`: Tests for Oracle services
- `tee-services/`: Tests for TEE services
- `service-api/`: Tests for Service API

## Running Tests

To run all tests:

```bash
cargo test --all-features
```

To run tests for a specific component:

```bash
cargo test -p r3e-event --all-features
```

## Writing Tests

When writing tests, follow these guidelines:

1. Use descriptive test names
2. Test both success and failure cases
3. Use mock objects when appropriate
4. Keep tests independent and isolated
5. Follow Rust's testing conventions with #[cfg(test)] modules

## Test Dependencies

The tests use the following dependencies:

- `tokio`: For async testing
- `mockall`: For creating mock objects
- `assert_matches`: For more expressive assertions
- `test-log`: For capturing logs during tests
