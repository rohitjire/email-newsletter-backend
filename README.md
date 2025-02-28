# Rust Email Newsletter Backend

A backend service for managing email newsletters, built using Rust.

## 📂 Project Structure
```
📂 rust-email-newsletter-backend
├── entity
│   └── src
│       ├── articles
│       ├── mod.rs
│       ├── prelude.rs
│       ├── subscriptions.rs
│       └── users.rs
├── migration
│   └── src
│       ├── lib.rs
│       ├── m20220101_000001_create_tables.rs
│       ├── m20221130_145647_create_user_tables.rs
│       ├── m20250102_221835_article_tables.rs
│       └── m20250208_132108_subscription_tables.rs
└── src
    ├── article
    │   ├── article_handlers.rs
    │   ├── article_routes.rs
    │   └── mod.rs
    ├── auth
    │   ├── auth_handlers.rs
    │   ├── auth_routes.rs
    │   └── mod.rs
    ├── email
    │   ├── email.rs
    │   └── mod.rs
    ├── health
    │   ├── health_handler.rs
    │   ├── health_routes.rs
    │   └── mod.rs
    ├── middlewares
    │   ├── auth_middlewares.rs
    │   └── mod.rs
    ├── subscription
    │   ├── subscription_handlers.rs
    │   ├── subscription_routes.rs
    │   └── mod.rs
    ├── templates
    │   └── email_template.html
    ├──testcases
    │   ├── article_handelers.rs
    │   ├── auth_handler_test.rs
    │   ├── subscription_handlers_test.rs
    │   ├── user_handlers_test.rs
    │   └── mod.rs
    ├── user
    │   ├── user_handlers.rs
    │   ├── user_routes.rs
    │   └── mod.rs
    ├── utils
    │   ├── api_responses.rs
    │   ├── app_state.rs
    │   ├── constants.rs
    │   ├── jwt.rs
    │   ├── main_error.rs
    │   └── mod.rs
    └── main.rs
```

## 🛠 Installation
```bash
git clone https://github.com/username/rust-email-newsletter-backend.git
cd rust-email-newsletter-backend
cargo build
```

## 🚀 Running the Project
```bash
cargo run
```
The server should be available at `http://localhost:8000`

## 🧩 Key Modules
- **Entity**: Database models (articles, users, subscriptions)
- **Migration**: Database migration scripts
- **Article Module**: Routes and handlers for articles
- **Auth Module**: Routes and handlers for authentication
- **Email Module**: Email services
- **Middleware**: Common middleware (e.g., authentication checks)
- **Subscription Module**: Routes and handlers for subscriptions
- **User Module**: User management
- **Utilities**: Helpers such as JWT handling, error responses, and app state

## 🤝 Contributing
1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Commit your changes (`git commit -m 'Add feature'`)
4. Push to the branch (`git push origin feature-branch`)
5. Create a Pull Request
