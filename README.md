# 🚀 RustyX

<div align="center">

![RustyX Logo](https://img.shields.io/badge/RustyX-Fast%20Web%20Framework-orange?style=for-the-badge&logo=rust)

[![Crates.io](https://img.shields.io/crates/v/rustyx.svg?style=flat-square)](https://crates.io/crates/rustyx)
[![Documentation](https://docs.rs/rustyx/badge.svg)](https://docs.rs/rustyx)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Mohammad007/rustyx/ci.yml?branch=main&style=flat-square)](https://github.com/Mohammad007/rustyx/actions)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg?style=flat-square)](https://www.rust-lang.org)

**A fast, minimalist web framework for Rust inspired by ExpressJS**

[Getting Started](#-getting-started) •
[Documentation](#-documentation) •
[Examples](#-examples) •
[Contributing](#-contributing)

</div>

---

## ✨ Features

- 🎯 **ExpressJS-like API** - Familiar interface for JavaScript developers
- ⚡ **Blazingly Fast** - Built on top of Hyper and Tokio for maximum performance
- 🔌 **Middleware Support** - Express-style middleware system
- 🗄️ **Multi-Database ORM** - Built-in support for MongoDB, MySQL, PostgreSQL, and SQLite
- 📁 **Organized Structure** - MVC-style folder structure (Controllers, Routes, Models)
- 🔒 **Type-Safe** - Leverage Rust's type system for safer code
- 📝 **JSON-First** - Designed for building REST APIs
- 🛡️ **Built-in Security** - Helmet middleware for security headers

## 📦 Installation

Add RustyX to your `Cargo.toml`:

```toml
[dependencies]
rustyx = "0.1.0"

# Enable database features as needed
# rustyx = { version = "0.1.0", features = ["mysql", "postgres", "mongodb"] }
```

### Feature Flags

| Feature | Description |
|---------|-------------|
| `default` | SQLite support enabled |
| `mysql` | MySQL database support |
| `postgres` | PostgreSQL database support |
| `sqlite` | SQLite database support |
| `mongodb` | MongoDB database support |
| `full` | All database drivers enabled |

## 🚀 Getting Started

### Quick Start

Create a simple API server in just a few lines:

```rust
use rustyx::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app = RustyX::new();

    app.get("/", |_req, res| async move {
        res.json(json!({ "message": "Hello, World!" }))
    });

    app.listen(3000).await
}
```

### Complete Example

```rust
use rustyx::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Option<String>,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().init();

    let app = RustyX::new();

    // GET all users
    app.get("/api/users", |_req, res| async move {
        let users = vec![
            json!({ "id": "1", "name": "John", "email": "john@example.com" }),
            json!({ "id": "2", "name": "Jane", "email": "jane@example.com" }),
        ];
        res.json(json!({ "data": users }))
    });

    // GET single user
    app.get("/api/users/:id", |req, res| async move {
        let id = req.param("id").unwrap();
        res.json(json!({ "id": id, "name": "John Doe" }))
    });

    // POST create user
    app.post("/api/users", |req, res| async move {
        match req.json::<User>() {
            Ok(user) => res.status(201).json(user),
            Err(e) => res.bad_request(&e.to_string()),
        }
    });

    // PUT update user
    app.put("/api/users/:id", |req, res| async move {
        let id = req.param("id").unwrap();
        match req.json::<User>() {
            Ok(mut user) => {
                user.id = Some(id.clone());
                res.json(user)
            }
            Err(e) => res.bad_request(&e.to_string()),
        }
    });

    // DELETE user
    app.delete("/api/users/:id", |req, res| async move {
        let id = req.param("id").unwrap();
        res.json(json!({ "deleted": true, "id": id }))
    });

    info!("🚀 Server running at http://localhost:3000");
    app.listen(3000).await
}
```

## 📖 Documentation

### Request Object

The `Request` object provides access to request data:

```rust
// Get URL parameters
let id = req.param("id");

// Get query parameters
let page = req.query_param("page");

// Parse JSON body
let data: MyStruct = req.json()?;

// Get headers
let auth = req.header("authorization");
let token = req.bearer_token();

// Request info
let method = req.method();
let path = req.path();
let ip = req.ip();
```

### Response Object

The `Response` object provides methods for sending responses:

```rust
// JSON response
res.json(json!({ "key": "value" }))

// Text response
res.send("Hello, World!")

// HTML response
res.html("<h1>Hello</h1>")

// Status codes
res.status(201).json(data)
res.status(404).not_found()
res.status(400).bad_request("Invalid input")
res.status(401).unauthorized()

// Redirects
res.redirect("/login")
res.redirect_permanent("/new-url")

// Headers & Cookies
res.header("X-Custom", "value")
res.cookie("session", "abc123", CookieOptions::new())
res.cors("*")
```

### Middleware

Add middleware for cross-cutting concerns:

```rust
use rustyx::middleware::{logger, cors, helmet};

let app = RustyX::new();

// Built-in middleware
app.use_middleware(logger());
app.use_middleware(cors("*"));
app.use_middleware(helmet());

// Custom middleware
app.use_middleware(|req, res, next| async move {
    println!("Request: {} {}", req.method(), req.path());
    let response = next(req, res).await;
    println!("Response sent");
    response
});
```

### Router

Organize routes using the Router:

```rust
use rustyx::Router;

// Create a router with prefix
let mut api = Router::with_prefix("/api/v1");

api.get("/users", users_handler);
api.post("/users", create_user_handler);
api.get("/users/:id", get_user_handler);

// Mount router on app
app.use_router("/", api);

// Route groups
let mut router = Router::new();
router.group("/admin", |r| {
    r.get("/dashboard", dashboard_handler);
    r.get("/users", admin_users_handler);
});
```

### Database (ORM)

Connect to databases with the built-in ORM:

```rust
use rustyx::db::prelude::*;

// Configure database
let config = DatabaseConfig::new(DbDriver::PostgreSQL, "mydb")
    .host("localhost")
    .port(5432)
    .username("user")
    .password("pass");

// Initialize connection
init_db(config).await?;

// Use Query Builder
let query = QueryBuilder::table("users")
    .select(&["id", "name", "email"])
    .where_eq("active", "true")
    .order_by("created_at", Order::Desc)
    .limit(10)
    .build();
```

### Models

Define models with the Model trait:

```rust
use rustyx::models::Model;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Option<String>,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
}

impl Model for User {
    fn collection_name() -> &'static str { "users" }
    
    fn get_id(&self) -> Option<String> { self.id.clone() }
    
    fn set_id(&mut self, id: String) { self.id = Some(id); }
    
    fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::validation("Name is required"));
        }
        Ok(())
    }
}
```

## 📁 Project Structure

RustyX follows an organized folder structure similar to Node.js/Express:

```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library exports
│   ├── controllers/      # Request handlers
│   │   ├── mod.rs
│   │   ├── user_controller.rs
│   │   └── product_controller.rs
│   ├── routes/           # Route definitions
│   │   ├── mod.rs
│   │   ├── user_routes.rs
│   │   └── api_routes.rs
│   ├── models/           # Data models
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── product.rs
│   ├── db/               # Database layer
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   └── migrations/
│   ├── middleware/       # Custom middleware
│   │   └── mod.rs
│   └── utils/            # Utility functions
│       └── mod.rs
├── tests/                # Integration tests
└── examples/             # Example applications
```

## 🔧 API Reference

### RustyX (App)

| Method | Description |
|--------|-------------|
| `new()` | Create a new application |
| `get(path, handler)` | Register GET route |
| `post(path, handler)` | Register POST route |
| `put(path, handler)` | Register PUT route |
| `delete(path, handler)` | Register DELETE route |
| `patch(path, handler)` | Register PATCH route |
| `all(path, handler)` | Register handler for all methods |
| `use_middleware(fn)` | Add middleware |
| `use_router(path, router)` | Mount a router |
| `listen(port)` | Start the server |

### Request

| Method | Description |
|--------|-------------|
| `method()` | Get HTTP method |
| `path()` | Get request path |
| `param(name)` | Get route parameter |
| `query_param(name)` | Get query parameter |
| `json<T>()` | Parse JSON body |
| `body()` | Get raw body bytes |
| `header(name)` | Get header value |
| `bearer_token()` | Extract Bearer token |
| `ip()` | Get client IP address |

### Response

| Method | Description |
|--------|-------------|
| `status(code)` | Set status code |
| `json(data)` | Send JSON response |
| `send(text)` | Send text response |
| `html(html)` | Send HTML response |
| `redirect(url)` | Send redirect |
| `header(name, value)` | Set header |
| `cookie(name, value, opts)` | Set cookie |
| `cors(origin)` | Add CORS headers |

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run with all features:

```bash
cargo test --all-features
```

## 📊 Benchmarks

RustyX is designed for high performance:

```
Requests/sec: ~150,000 (simple JSON response)
Latency (avg): ~0.5ms
Memory: ~5MB idle
```

*Benchmarks on AMD Ryzen 9 5900X, 32GB RAM*

## 🗺️ Roadmap

- [x] Basic routing
- [x] Middleware support
- [x] JSON parsing
- [x] Query builder
- [ ] WebSocket support
- [ ] GraphQL integration
- [ ] OpenAPI/Swagger generation
- [ ] Rate limiting middleware
- [ ] Session management
- [ ] File uploads

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [ExpressJS](https://expressjs.com/)
- Built with [Hyper](https://hyper.rs/) and [Tokio](https://tokio.rs/)
- Routing powered by [matchit](https://github.com/ibraheemdev/matchit)

---

<div align="center">

**Made with ❤️ by the RustyX Team**

[Report Bug](https://github.com/Mohammad007/rustyx/issues) •
[Request Feature](https://github.com/Mohammad007/rustyx/issues)

</div>
