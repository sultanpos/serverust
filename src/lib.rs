// Clean Architecture - Simpler, More Explicit
pub mod application;
pub mod config;
pub mod crypto;
pub mod domain;
pub mod persistence;
pub mod server;
pub mod web;

// Re-exports for convenience
pub use server::create_app;
