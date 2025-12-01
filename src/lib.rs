//! # conflow - Configuration Flow Orchestrator
//!
//! `conflow` intelligently orchestrates CUE, Nickel, and configuration validation workflows.
//!
//! ## Features
//!
//! - **Intelligent analysis** - Recommends CUE vs Nickel based on complexity
//! - **Pipeline orchestration** - Chain tools with dependency management
//! - **Smart caching** - Only re-run what changed
//! - **Educational** - Learn why certain tools fit certain problems
//!
//! ## Quick Start
//!
//! ```bash
//! # Initialize a new project
//! conflow init my-project
//!
//! # Analyze existing configs
//! conflow analyze config.yaml
//!
//! # Run pipeline
//! conflow run
//! ```

pub mod analyzer;
pub mod cache;
pub mod cli;
pub mod errors;
pub mod executors;
pub mod pipeline;
pub mod utils;

// Re-export commonly used types
pub use errors::{ConflowError, ConflowResult};
pub use pipeline::{Pipeline, Stage};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
