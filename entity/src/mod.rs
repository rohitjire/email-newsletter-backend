//! `mod.rs` - Entry point for the `entity` module.
//! This module re-exports all entity modules, making them accessible from a single location.
//!
//! # Modules
//! - `prelude` - Common traits and types for entity interactions.
//! - `article` - Defines the `Article` entity.
//! - `subscription` - Defines the `Subscription` entity.
//! - `user` - Defines the `User` entity.

pub mod prelude;
pub mod article;
pub mod subscription;
pub mod user;
