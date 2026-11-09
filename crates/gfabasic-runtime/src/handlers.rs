//! Typed, profile-aware registrations for generated-program API handlers.
//!
//! A registration is deliberately separate from registry evidence and support
//! metadata: registering a function does not by itself claim that a GFA BASIC
//! declaration is implemented or conformant.

use std::collections::BTreeMap;

/// The API namespace that owns a handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiProfile {
    LanguageCore,
    Amiga,
}

/// Stable declaration state copied from the validated API registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplementationStatus {
    Implemented,
    Partial,
    Planned,
    Research,
    NotImplemented,
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

/// Registry facts required to resolve one declaration.
#[derive(Debug, Clone, Copy)]
pub struct HandlerRequirement<'a> {
    pub id: &'a str,
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
    UnsupportedDeclaration(String),
    InvalidIdNamespace(String),
    Missing(String),
    Duplicate(String),
    ProfileMismatch(String),
    SignatureMismatch(String),
}

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
        };
        if !requirement.id.starts_with(expected_prefix) {
            return Err(RegistryError::InvalidIdNamespace(requirement.id.to_owned()));
        }

        let matches = self
            .registrations
            .get(requirement.id)
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
            RuntimeHandler::IntegerToUnsigned(f) => ResolvedHandler::IntegerToUnsigned(f),
            RuntimeHandler::TwoIntegersToUnsigned(f) => ResolvedHandler::TwoIntegersToUnsigned(f),
        })
    }

    /// Validate the complete set of declarations that claim runtime support.
    pub fn validate<'a>(
        &self,
        requirements: impl IntoIterator<Item = HandlerRequirement<'a>>,
    ) -> Result<(), RegistryError> {
        for requirement in requirements {
            self.resolve(requirement)?;
        }
        Ok(())
    }
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
        let error = HandlerRegistry::new()
            .resolve(requirement(ImplementationStatus::Implemented))
            .unwrap_err();
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
    fn rejects_stale_or_wrong_namespace_ids() {
        let mut req = requirement(ImplementationStatus::Implemented);
        req.id = "gfa.amiga.math.fact";
        assert_eq!(
            registry().resolve(req).unwrap_err(),
            RegistryError::Missing("gfa.amiga.math.fact".into())
        );
        req.id = "fact";
        assert_eq!(
            registry().resolve(req).unwrap_err(),
            RegistryError::InvalidIdNamespace("fact".into())
        );
    }

    #[test]
    fn rejects_profile_mismatch() {
        let mut registry = HandlerRegistry::new();
        registry.register(HandlerRegistration {
            id: "gfa.language.math.fact",
            profile: ApiProfile::Amiga,
            handler: RuntimeHandler::IntegerToUnsigned(unary),
        });
        assert_eq!(
            registry.resolve(requirement(ImplementationStatus::Implemented)).unwrap_err(),
            RegistryError::ProfileMismatch("gfa.language.math.fact".into())
        );
    }

    #[test]
    fn rejects_signature_mismatch() {
        let mut req = requirement(ImplementationStatus::Implemented);
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
                .resolve(requirement(ImplementationStatus::Research))
                .unwrap_err(),
            RegistryError::UnsupportedDeclaration("gfa.language.math.fact".into())
        );
    }

    #[test]
    fn binary_handlers_have_a_distinct_typed_signature() {
        let mut registry = HandlerRegistry::new();
        registry.register(HandlerRegistration {
            id: "gfa.language.math.variat",
            profile: ApiProfile::LanguageCore,
            handler: RuntimeHandler::TwoIntegersToUnsigned(binary),
        });
        let resolved = registry
            .resolve(HandlerRequirement {
                id: "gfa.language.math.variat",
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
}
