# Change log

## [Unreleased]

### Security

- Validating user has access to the budget when they are doing any operations on items

### Changed

- Only fetch JWKs once on application startup
- Refactored state into a global container to match axum's model for how to better share different services across handles

## [0.1.0] - 2023-03-08

### Added

- CRUD endpoints for budget
- CRUD endpoints for managing items on a budget
- JWT authorization with [Auth0](https://auth0.com) as authority
- Tracing support throughout application
- Customize port to run at from environment variable
