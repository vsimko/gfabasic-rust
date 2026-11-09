//! Typed, profile-aware registrations for generated-program API handlers.
//!
//! The checked-in TOML API registry is the source of declaration requirements.
//! A generated Rust table mirrors those requirements; CI checks that it is
//! current. Runtime registrations remain explicit typed function pointers.

use std::collections::BTreeMap;

mod generated;
pub use generated::API_HANDLER_REQUIREMENTS;

/// The API namespace that owns a handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiProfile {
    LanguageCore,
    Amiga,
    AtariSt,
}

/// Implementation state copied from the canonical API registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplementationStatus {
    Implemented,
    Partial,
    NotImplemented,
    UnsafeDisabled,
}

impl ImplementationStatus {
    fn is_callable(self) -> bool {
        matches!(self, Self::Implemented | Self::Partial)
    }
}

/// The exact ABI shape of a runtime handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerSignature {
    IntegerToUnsigned,
    TwoIntegersToUnsigned,
    Unsupported,
}

/// Typed handler target. No dynamically typed argument list or fallback exists.
#[derive(Clone, Copy)]
pub enum RuntimeHandler {
    IntegerToUnsigned(fn(u32) -> Option<u128>),
    TwoIntegersToUnsigned(fn(u32, u32) -> Option<u128>),
}

impl RuntimeHandler {
    pub fn signature(self) -> HandlerSignature {
        match self {
            Self::IntegerToUnsigned(_) => HandlerSignature::IntegerToUnsigned,
            Self::TwoIntegersToUnsigned(_) => HandlerSignature::TwoIntegersToUnsigned,
        }
    }
}

/// One explicit registration. Duplicate IDs are retained and rejected at validation.
#[derive(Clone, Copy)]
pub struct HandlerRegistration {
    pub id: &'static str,
    pub profile: ApiProfile,
    pub handler: RuntimeHandler,
}

/// Registry facts generated from one canonical API declaration.
#[derive(Debug, Clone, Copy)]
pub struct HandlerRequirement<'a> {
    pub id: &'a str,
    pub handler_id: Option<&'a str>,
    pub profile: ApiProfile,
    pub status: ImplementationStatus,
    pub signature: HandlerSignature,
}

/// A successful resolution contains the concrete typed function pointer.
#[derive(Debug, Clone, Copy)]
pub enum ResolvedHandler {
    IntegerToUnsigned(fn(u32) -> Option<u128>),
    TwoIntegersToUnsigned(fn(u32, u32) -> Option<u128>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    UnknownDeclaration(String),
    UnsupportedDeclaration(String),
    InvalidIdNamespace(String),
    Missing(String),
    Duplicate(String),
    Stale(String),
    ProfileMismatch(String),
    SignatureMismatch(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownDeclaration(id) => write!(formatter, "unknown API declaration '{id}'"),
            Self::UnsupportedDeclaration(id) => {
                write!(formatter, "API declaration '{id}' is not callable")
            }
            Self::InvalidIdNamespace(id) => {
                write!(formatter, "invalid API ID namespace for '{id}'")
            }
            Self::Missing(id) => write!(formatter, "missing runtime handler for '{id}'"),
            Self::Duplicate(id) => write!(formatter, "duplicate runtime handler for '{id}'"),
            Self::Stale(id) => write!(formatter, "stale runtime handler registration '{id}'"),
            Self::ProfileMismatch(id) => write!(formatter, "API profile mismatch for '{id}'"),
            Self::SignatureMismatch(id) => {
                write!(formatter, "handler signature mismatch for '{id}'")
            }
        }
    }
}

impl std::error::Error for RegistryError {}

#[derive(Default)]
pub struct HandlerRegistry {
    registrations: BTreeMap<String, Vec<HandlerRegistration>>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, registration: HandlerRegistration) {
        self.registrations
            .entry(registration.id.to_owned())
            .or_default()
            .push(registration);
    }

    /// Construct the explicit built-in handler table.
    pub fn builtins() -> Self {
        use crate::core::combinatorics::{combin, factorial, variat};

        let mut registry = Self::new();
        registry.register(HandlerRegistration {
            id: "gfa.language.math.fact",
            profile: ApiProfile::LanguageCore,
            handler: RuntimeHandler::IntegerToUnsigned(factorial),
        });
        registry.register(HandlerRegistration {
            id: "gfa.language.math.variat",
            profile: ApiProfile::LanguageCore,
            handler: RuntimeHandler::TwoIntegersToUnsigned(variat),
        });
        registry.register(HandlerRegistration {
            id: "gfa.language.math.combin",
            profile: ApiProfile::LanguageCore,
            handler: RuntimeHandler::TwoIntegersToUnsigned(combin),
        });
        registry
    }

    /// Resolve only declarations explicitly marked implemented or partial.
    /// Unsupported declarations are rejected even if a matching handler exists.
    pub fn resolve(
        &self,
        requirement: HandlerRequirement<'_>,
    ) -> Result<ResolvedHandler, RegistryError> {
        if !requirement.status.is_callable() {
            return Err(RegistryError::UnsupportedDeclaration(
                requirement.id.to_owned(),
            ));
        }

        let expected_prefix = match requirement.profile {
            ApiProfile::LanguageCore => "gfa.language.",
            ApiProfile::Amiga => "gfa.amiga.",
            ApiProfile::AtariSt => "gfa.atari-st.",
        };
        if !requirement.id.starts_with(expected_prefix) {
            return Err(RegistryError::InvalidIdNamespace(requirement.id.to_owned()));
        }

        let handler_id = requirement
            .handler_id
            .ok_or_else(|| RegistryError::Missing(requirement.id.to_owned()))?;
        if !handler_id.starts_with(expected_prefix) {
            return Err(RegistryError::InvalidIdNamespace(handler_id.to_owned()));
        }
        let matches = self
            .registrations
            .get(handler_id)
            .ok_or_else(|| RegistryError::Missing(requirement.id.to_owned()))?;
        if matches.len() != 1 {
            return Err(RegistryError::Duplicate(requirement.id.to_owned()));
        }

        let registration = matches[0];
        if registration.profile != requirement.profile {
            return Err(RegistryError::ProfileMismatch(requirement.id.to_owned()));
        }
        if registration.handler.signature() != requirement.signature {
            return Err(RegistryError::SignatureMismatch(requirement.id.to_owned()));
        }

        Ok(match registration.handler {
            RuntimeHandler::IntegerToUnsigned(function) => {
                ResolvedHandler::IntegerToUnsigned(function)
            }
            RuntimeHandler::TwoIntegersToUnsigned(function) => {
                ResolvedHandler::TwoIntegersToUnsigned(function)
            }
        })
    }

    /// Validate every registry declaration and reject all stale registrations.
    pub fn validate<'a>(
        &self,
        requirements: impl IntoIterator<Item = HandlerRequirement<'a>>,
    ) -> Result<(), RegistryError> {
        let requirements: Vec<_> = requirements.into_iter().collect();
        for requirement in &requirements {
            if requirement.status.is_callable() {
                self.resolve(*requirement)?;
            } else if requirement.handler_id.is_some() {
                return Err(RegistryError::UnsupportedDeclaration(
                    requirement.id.to_owned(),
                ));
            }
        }

        for id in self.registrations.keys() {
            if !requirements.iter().any(|requirement| {
                requirement.status.is_callable() && requirement.handler_id == Some(id.as_str())
            }) {
                return Err(RegistryError::Stale(id.clone()));
            }
        }
        Ok(())
    }
}

/// Validate the concrete built-ins against all declarations in the canonical registry.
pub fn validate_builtin_registry() -> Result<(), RegistryError> {
    HandlerRegistry::builtins().validate(API_HANDLER_REQUIREMENTS.iter().copied())
}

/// Resolve a compiler API call by its canonical declaration ID.
pub fn resolve_builtin_handler(id: &str) -> Result<ResolvedHandler, RegistryError> {
    let requirement = API_HANDLER_REQUIREMENTS
        .iter()
        .find(|requirement| requirement.id == id)
        .copied()
        .ok_or_else(|| RegistryError::UnknownDeclaration(id.to_owned()))?;
    HandlerRegistry::builtins().resolve(requirement)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unary(value: u32) -> Option<u128> {
        Some(u128::from(value) + 1)
    }

    fn binary(left: u32, right: u32) -> Option<u128> {
        Some(u128::from(left) + u128::from(right))
    }

    fn requirement(status: ImplementationStatus) -> HandlerRequirement<'static> {
        HandlerRequirement {
            id: "gfa.language.math.fact",
            handler_id: Some("gfa.language.math.fact"),
            profile: ApiProfile::LanguageCore,
            status,
            signature: HandlerSignature::IntegerToUnsigned,
        }
    }

    fn registry() -> HandlerRegistry {
        let mut registry = HandlerRegistry::new();
        registry.register(HandlerRegistration {
            id: "gfa.language.math.fact",
            profile: ApiProfile::LanguageCore,
            handler: RuntimeHandler::IntegerToUnsigned(unary),
        });
        registry
    }

    #[test]
    fn resolves_exactly_one_typed_handler() {
        let resolved = registry()
            .resolve(requirement(ImplementationStatus::Implemented))
            .unwrap();
        match resolved {
            ResolvedHandler::IntegerToUnsigned(handler) => assert_eq!(handler(4), Some(5)),
            ResolvedHandler::TwoIntegersToUnsigned(_) => panic!("wrong handler type"),
        }
    }

    #[test]
    fn rejects_missing_handler() {
        let mut req = requirement(ImplementationStatus::Implemented);
        req.handler_id = Some("gfa.language.math.fact");
        let error = HandlerRegistry::new().resolve(req).unwrap_err();
        assert_eq!(
            error,
            RegistryError::Missing("gfa.language.math.fact".into())
        );
    }

    #[test]
    fn rejects_duplicate_handler_ids() {
        let mut registry = registry();
        registry.register(HandlerRegistration {
            id: "gfa.language.math.fact",
            profile: ApiProfile::LanguageCore,
            handler: RuntimeHandler::IntegerToUnsigned(unary),
        });
        assert_eq!(
            registry
                .resolve(requirement(ImplementationStatus::Implemented))
                .unwrap_err(),
            RegistryError::Duplicate("gfa.language.math.fact".into())
        );
    }

    #[test]
    fn validation_rejects_unreferenced_stale_registrations() {
        let mut registry = registry();
        registry.register(HandlerRegistration {
            id: "gfa.language.math.old_fact",
            profile: ApiProfile::LanguageCore,
            handler: RuntimeHandler::IntegerToUnsigned(unary),
        });
        assert_eq!(
            registry.validate([requirement(ImplementationStatus::Implemented)]),
            Err(RegistryError::Stale("gfa.language.math.old_fact".into()))
        );
    }

    #[test]
    fn rejects_wrong_namespace_profile_and_signature() {
        let mut req = requirement(ImplementationStatus::Implemented);
        req.id = "gfa.amiga.math.fact";
        assert_eq!(
            registry().resolve(req).unwrap_err(),
            RegistryError::InvalidIdNamespace("gfa.amiga.math.fact".into())
        );

        req = requirement(ImplementationStatus::Implemented);
        req.id = "gfa.amiga.math.fact";
        req.handler_id = Some("gfa.amiga.math.fact");
        req.profile = ApiProfile::Amiga;
        let mut wrong_profile = HandlerRegistry::new();
        wrong_profile.register(HandlerRegistration {
            id: "gfa.amiga.math.fact",
            profile: ApiProfile::LanguageCore,
            handler: RuntimeHandler::IntegerToUnsigned(unary),
        });
        assert_eq!(
            wrong_profile.resolve(req).unwrap_err(),
            RegistryError::ProfileMismatch("gfa.amiga.math.fact".into())
        );

        req = requirement(ImplementationStatus::Implemented);
        req.signature = HandlerSignature::TwoIntegersToUnsigned;
        assert_eq!(
            registry().resolve(req).unwrap_err(),
            RegistryError::SignatureMismatch("gfa.language.math.fact".into())
        );
    }

    #[test]
    fn unsupported_declarations_cannot_use_a_fallback_handler() {
        assert_eq!(
            registry()
                .resolve(requirement(ImplementationStatus::NotImplemented))
                .unwrap_err(),
            RegistryError::UnsupportedDeclaration("gfa.language.math.fact".into())
        );
        assert_eq!(
            registry()
                .resolve(requirement(ImplementationStatus::UnsafeDisabled))
                .unwrap_err(),
            RegistryError::UnsupportedDeclaration("gfa.language.math.fact".into())
        );
    }

    #[test]
    fn binary_handlers_have_distinct_typed_signatures() {
        let mut registry = HandlerRegistry::new();
        registry.register(HandlerRegistration {
            id: "gfa.language.math.variat",
            profile: ApiProfile::LanguageCore,
            handler: RuntimeHandler::TwoIntegersToUnsigned(binary),
        });
        let resolved = registry
            .resolve(HandlerRequirement {
                id: "gfa.language.math.variat",
                handler_id: Some("gfa.language.math.variat"),
                profile: ApiProfile::LanguageCore,
                status: ImplementationStatus::Partial,
                signature: HandlerSignature::TwoIntegersToUnsigned,
            })
            .unwrap();
        match resolved {
            ResolvedHandler::TwoIntegersToUnsigned(handler) => assert_eq!(handler(2, 3), Some(5)),
            ResolvedHandler::IntegerToUnsigned(_) => panic!("wrong handler type"),
        }
    }

    #[test]
    fn generated_registry_and_builtin_handlers_are_in_sync() {
        validate_builtin_registry().unwrap();
    }

    #[test]
    fn compiler_lookup_resolves_only_declared_callable_handlers() {
        match resolve_builtin_handler("gfa.language.math.fact").unwrap() {
            ResolvedHandler::IntegerToUnsigned(handler) => assert_eq!(handler(5), Some(120)),
            ResolvedHandler::TwoIntegersToUnsigned(_) => panic!("wrong handler type"),
        }
        assert_eq!(
            resolve_builtin_handler("gfa.language.matrix.mat-one").unwrap_err(),
            RegistryError::UnsupportedDeclaration("gfa.language.matrix.mat-one".into())
        );
        assert_eq!(
            resolve_builtin_handler("gfa.language.unknown.call").unwrap_err(),
            RegistryError::UnknownDeclaration("gfa.language.unknown.call".into())
        );
    }
}
