# neutrino-shared

Core library crate shared across all Neutrino backend services.

## Contents

- `src/api_error.rs` — unified `ApiError` type returned by all endpoints
- `src/errors.rs` — `AppError` / `AppResult` aliases
- `src/auth/` — JWT validation helpers and `AuthenticatedUser` extractor
- `src/drive_client.rs` — typed HTTP client for the Drive service
- `src/logger.rs` — structured tracing initializer

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
shared = { git = "https://github.com/YOUR_ORG/neutrino-shared", branch = "main" }
```

Pin to a specific tag for production:

```toml
shared = { git = "https://github.com/YOUR_ORG/neutrino-shared", tag = "v0.1.0" }
```
