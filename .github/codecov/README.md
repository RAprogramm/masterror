# Codecov Configuration

This directory contains all Codecov-related configuration for the masterror project.

## Files

### codecov.yml
Main Codecov configuration file with settings for:
- **Code Coverage**: Coverage reporting, thresholds, and PR comments
- **Test Analytics**: Test performance tracking and flaky test detection
- **GitHub Integration**: Annotations and status checks

## Features Enabled

### 1. Code Coverage
- Target: 95% coverage for project and patches
- Threshold: 1% drop allowed before failing
- Reports: LCOV format uploaded via OIDC
- Comments: Condensed format on pull requests

### 2. Test Analytics
- JUnit XML test results ingestion
- Test execution time tracking
- Failure rate monitoring
- Flaky test detection (threshold: 2 failures)
- PR notifications for flaky tests

## CI Integration

Coverage and test results are uploaded from `.github/workflows/reusable-ci.yml`:

```yaml
# Code Coverage (OIDC)
- uses: codecov/codecov-action@v5
  with:
    files: ./lcov.info
    use_oidc: true

# Test Analytics
- uses: codecov/test-results-action@v1
  with:
    files: ./target/nextest/ci/junit.xml
```

## Links

- [Codecov Dashboard](https://codecov.io/gh/RAprogramm/masterror)
- [Test Analytics](https://app.codecov.io/gh/RAprogramm/masterror/tests)
- [Coverage Documentation](https://docs.codecov.com/docs/codecov-yaml)
- [Test Analytics Documentation](https://docs.codecov.com/docs/test-analytics)
