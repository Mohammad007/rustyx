//! # RustyX
//!
//! A fast, minimalist web framework for Rust inspired by ExpressJS.
//!
//! RustyX provides an ExpressJS-like interface for building web APIs in Rust,
//! with built-in ORM support for MongoDB, MySQL, SQLite, and PostgreSQL.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use rustyx::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let app = RustyX::new();
//!
//!     app.get("/", |_req, res| async move {
//!         res.json(json!({ "message": "Hello, World!" }))
//!     });
//!
//!     app.listen(3000).await?;
//!     Ok(())
//! }
//! ```

pub mod app;
pub mod controllers;
pub mod db;
pub mod error;
pub mod middleware;
pub mod models;
pub mod request;
pub mod response;
pub mod router;
pub mod routes;
pub mod utils;

// Re-exports for convenience
pub use app::RustyX;
pub use error::{Error, Result};
pub use middleware::{Middleware, MiddlewareFn, Next};
pub use request::Request;
pub use response::Response;
pub use router::Router;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::app::RustyX;
    pub use crate::controllers::Controller;
    pub use crate::db::prelude::*;
    pub use crate::error::{Error, Result};
    pub use crate::middleware::{Middleware, MiddlewareFn, Next};
    pub use crate::models::Model;
    pub use crate::request::Request;
    pub use crate::response::Response;
    pub use crate::router::Router;
    pub use async_trait::async_trait;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{json, Value};
    pub use tracing::{debug, error, info, trace, warn};
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
