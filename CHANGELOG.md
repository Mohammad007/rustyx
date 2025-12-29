# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure
- Core framework implementation

## [0.1.0] - 2024-12-29

### Added
- 🎉 Initial release of RustyX
- ExpressJS-like routing (GET, POST, PUT, DELETE, PATCH)
- Request object with params, query, body parsing
- Response object with JSON, HTML, text support
- Middleware support with Next function
- Built-in middleware: logger, CORS, helmet, timeout
- Database module with multi-driver support
  - SQLite support (default)
  - MySQL support (feature flag)
  - PostgreSQL support (feature flag)
  - MongoDB support (feature flag)
- Query builder for SQL databases
- Model trait for database entities
- Controller trait for request handlers
- Route groups and API versioning helpers
- Utility functions (pagination, validation, etc.)
- Comprehensive documentation
- Example applications

### Security
- Helmet middleware for security headers
- CORS middleware for cross-origin requests
- Input validation helpers

---

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 0.1.0 | 2024-12-29 | Initial release |

[Unreleased]: https://github.com/Mohammad007/rustyx/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Mohammad007/rustyx/releases/tag/v0.1.0
