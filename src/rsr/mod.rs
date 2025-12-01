//! RSR (Rhodium Standard Repository) Integration
//!
//! This module provides integration between conflow and the RSR ecosystem,
//! enabling:
//! - Validation of RSR requirement configurations
//! - Compliance checking for RSR-CONFIG-002
//! - Integration hooks for RSR validator
//! - Shared schema validation
//! - Auto-remediation for failing requirements
//! - Compliance badges for CI/CD
//! - Diff reports between compliance runs
//! - Template generation for compliant configurations

pub mod badges;
pub mod compliance;
pub mod config;
pub mod diff;
pub mod hooks;
pub mod remediation;
pub mod requirements;
pub mod schemas;
pub mod templates;

// Core compliance types
pub use compliance::{
    CheckDetail, ComplianceChecker, ComplianceLevel, ComplianceReport, ComplianceStats,
    RequirementResult,
};

// Hooks for external integration
pub use hooks::{RsrHooks, RsrTrigger};

// Requirements
pub use requirements::{RsrRequirement, RsrRequirementClass, RsrRequirementRegistry};

// Schemas
pub use schemas::RsrSchemaRegistry;

// Configuration
pub use config::RsrConfig;

// Remediation
pub use remediation::{AutoRemediator, RemediationAction, RemediationResult};

// Badges
pub use badges::{BadgeGenerator, BadgeStyle};

// Diff reports
pub use diff::{ComplianceDiff, ComplianceHistory, DiffReporter};

// Templates
pub use templates::{Template, TemplateGenerator, TemplateType};
