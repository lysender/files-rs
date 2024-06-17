mod error;
mod files;

// Re-export error types for convenience
pub use self::error::{Error, Result};

fn main() {
    println!("Hello, world!");
}
