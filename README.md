# Rust Email Newsletter Backend

A backend service for managing email newsletters, built using Rust.

## 📂 Project Structure
```
📂 rust-email-newsletter-backend
├── entity
│   └── src
│       ├── article.rs
│       ├── mod.rs
│       ├── prelude.rs
│       ├── subscription.rs
│       └── user.rs
├── migration
│   └── src
│       ├── lib.rs
│       ├── m20220101_000001_create_table.rs
│       ├── m20221130_145647_create_user_table.rs
│       ├── m20250102_221835_article_tables.rs
│       └── m20250208_132108_subscription_table.rs
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
    │   └── mod.rs
    ├── health
    ├── middlewares
    ├── subscription
    ├── user
    └── utils
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

## 🤝 Contributing
1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Commit your changes (`git commit -m 'Add feature'`)
4. Push to the branch (`git push origin feature-branch`)
5. Create a Pull Request

## 📜 License
This project is licensed under the MIT License.
