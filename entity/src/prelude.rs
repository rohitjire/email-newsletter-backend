//! `prelude.rs` - Re-exports common entity definitions.
//! This module provides easy access to all entity structs, simplifying imports.
//!
//! # Re-exported Entities
//! - `Article` - Represents the `Article` entity.
//! - `Subscription` - Represents the `Subscription` entity.
//! - `User` - Represents the `User` entity.

pub use super::article::Entity as Article;
pub use super::subscription::Entity as Subscription;
pub use super::user::Entity as User;
