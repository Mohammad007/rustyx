//! Middleware Module
//!
//! Provides middleware functionality similar to Express middleware.

use crate::request::Request;
use crate::response::Response;

use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::info;

/// Next function type for middleware chaining
pub type Next =
    Arc<dyn Fn(Request, Response) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>;

/// Middleware function type
pub type MiddlewareFn = Box<
    dyn Fn(Request, Response, Next) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync,
>;

/// Trait for implementing middleware
#[async_trait]
pub trait Middleware: Send + Sync {
    /// Process the request and optionally call the next middleware
    async fn handle(&self, req: Request, res: Response, next: Next) -> Response;
}

/// Stack of middleware functions
pub struct MiddlewareStack {
    stack: Vec<MiddlewareFn>,
}

impl MiddlewareStack {
    /// Create a new empty middleware stack
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Push a middleware function onto the stack
    pub fn push(&mut self, middleware: MiddlewareFn) {
        self.stack.push(middleware);
    }

    /// Get the number of middleware in the stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl Default for MiddlewareStack {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Built-in Middleware
// ============================================================================

/// Logger middleware - logs all incoming requests
pub fn logger() -> impl Fn(Request, Response, Next) -> Pin<Box<dyn Future<Output = Response> + Send>>
+ Send
+ Sync
+ Clone {
    move |req: Request, res: Response, next: Next| {
        Box::pin(async move {
            let method = req.method().clone();
            let path = req.path().to_string();
            let start = std::time::Instant::now();

            let response = next(req, res).await;

            let duration = start.elapsed();
            let status = response.get_status();

            info!("{} {} {} - {:?}", method, path, status.as_u16(), duration);

            response
        })
    }
}

/// CORS middleware - adds CORS headers to all responses
pub fn cors(
    origin: &'static str,
) -> impl Fn(Request, Response, Next) -> Pin<Box<dyn Future<Output = Response> + Send>>
+ Send
+ Sync
+ Clone {
    move |req: Request, res: Response, next: Next| {
        Box::pin(async move {
            // Handle preflight requests
            if req.method() == hyper::Method::OPTIONS {
                return res
                    .status(204)
                    .cors(origin)
                    .header("access-control-max-age", "86400");
            }

            let response = next(req, res).await;
            response
                .header("access-control-allow-origin", origin)
                .header(
                    "access-control-allow-methods",
                    "GET, POST, PUT, DELETE, PATCH, OPTIONS",
                )
                .header(
                    "access-control-allow-headers",
                    "Content-Type, Authorization",
                )
        })
    }
}

/// JSON body parser middleware options
#[derive(Clone)]
pub struct JsonOptions {
    pub limit: usize,
    pub strict: bool,
}

impl Default for JsonOptions {
    fn default() -> Self {
        Self {
            limit: 1024 * 1024, // 1MB
            strict: true,
        }
    }
}

/// JSON body parser middleware
pub fn json(
    options: JsonOptions,
) -> impl Fn(Request, Response, Next) -> Pin<Box<dyn Future<Output = Response> + Send>>
+ Send
+ Sync
+ Clone {
    move |req: Request, res: Response, next: Next| {
        let _options = options.clone();
        Box::pin(async move {
            // Check content type
            if let Some(content_type) = req.content_type() {
                if !content_type.contains("application/json") {
                    // Not JSON, pass through
                    return next(req, res).await;
                }
            }

            // Continue to next middleware/handler
            next(req, res).await
        })
    }
}

/// Rate limiting middleware options
#[derive(Clone)]
pub struct RateLimitOptions {
    pub window_ms: u64,
    pub max_requests: u32,
}

impl Default for RateLimitOptions {
    fn default() -> Self {
        Self {
            window_ms: 60_000, // 1 minute
            max_requests: 100,
        }
    }
}

/// Compression middleware options
#[derive(Clone)]
pub struct CompressionOptions {
    pub level: u32,
    pub threshold: usize,
}

impl Default for CompressionOptions {
    fn default() -> Self {
        Self {
            level: 6,
            threshold: 1024, // Only compress responses > 1KB
        }
    }
}

/// Security headers middleware
pub fn helmet() -> impl Fn(Request, Response, Next) -> Pin<Box<dyn Future<Output = Response> + Send>>
+ Send
+ Sync
+ Clone {
    move |req: Request, res: Response, next: Next| {
        Box::pin(async move {
            let response = next(req, res).await;
            response
                .header("x-content-type-options", "nosniff")
                .header("x-frame-options", "DENY")
                .header("x-xss-protection", "1; mode=block")
                .header(
                    "strict-transport-security",
                    "max-age=31536000; includeSubDomains",
                )
                .header("content-security-policy", "default-src 'self'")
        })
    }
}

/// Request timeout middleware
pub fn timeout(
    duration_ms: u64,
) -> impl Fn(Request, Response, Next) -> Pin<Box<dyn Future<Output = Response> + Send>>
+ Send
+ Sync
+ Clone {
    move |req: Request, res: Response, next: Next| {
        Box::pin(async move {
            let timeout = tokio::time::Duration::from_millis(duration_ms);

            match tokio::time::timeout(timeout, next(req, res)).await {
                Ok(response) => response,
                Err(_) => Response::new()
                    .status(408)
                    .json(serde_json::json!({ "error": "Request Timeout" })),
            }
        })
    }
}
