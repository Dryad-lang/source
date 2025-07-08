// src/oak/mod.rs
//! Oak Package Manager - Sistema Modular de Gerenciamento de Pacotes
//! 
//! Este módulo fornece APIs completas para gerenciamento de pacotes Dryad,
//! permitindo que desenvolvedores criem suas próprias implementações (similar ao yarn/pnpm para npm).

pub mod config;
pub mod manager;
pub mod registry;
pub mod resolver;
pub mod api;
pub mod cli_integration;

// Re-exports principais
pub use config::{OakConfig, OakPackage, OakDependency};
pub use manager::{OakManager, OakResult, OakError, OakOptions, OakErrorCode};
pub use registry::{PackageRegistry, LocalRegistry, RemoteRegistry};
pub use resolver::{DependencyResolver, VersionResolver};
pub use api::{OakApi, ExternalApi};

/// Versão do Oak
pub const OAK_VERSION: &str = "1.0.0";

/// Tipo de resultado padrão para operações Oak
pub type Result<T> = std::result::Result<T, OakError>;
