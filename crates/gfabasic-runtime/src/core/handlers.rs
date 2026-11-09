//! Typed registry-backed dispatch for the first language-core runtime functions.
//!
//! IDs are stable registry contracts, not Rust paths. Core IDs use
//! `gfa.runtime.core.<area>.<operation>`; core handlers cannot belong to an
//! Amiga/Atari API profile. The Amiga GFA BASIC 3.5 addendum documents these
//! functions on printed pp. 13-33–13-34.
//!
//! The checked u32/u128 boundary and error results below are Rust API choices,
//! not claims about the original interpreter's numeric representation or errors.

use super::combinatorics;

/// Stable argument/result shape used to reject incompatible mappings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandlerSignature {
    Integer1ToUnsigned128,
    Integer2ToUnsigned128,
}

/// A concrete language-core operation implemented by this runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoreHandler {
    Factorial,
    Variations,
    Combinations,
}

impl CoreHandler {
    pub const fn signature(self) -> HandlerSignature {
        match self {
            Self::Factorial => HandlerSignature::Integer1ToUnsigned128,
            Self::Variations | Self::Combinations => HandlerSignature::Integer2ToUnsigned128,
        }
    }

    /// Invoke the operation with signed integer expression values.
    pub fn invoke(self, arguments: &[i64]) -> Result<u128, InvokeError> {
        let expected = match self.signature() {
            HandlerSignature::Integer1ToUnsigned128 => 1,
            HandlerSignature::Integer2ToUnsigned128 => 2,
        };
        if arguments.len() != expected {
            return Err(InvokeError::WrongArity {
                expected,
                actual: arguments.len(),
            });
        }

        let n = u32::try_from(arguments[0]).map_err(|_| InvokeError::IntegerOutOfRange {
            argument: 0,
            value: arguments[0],
        })?;
        let result = match self {
            Self::Factorial => combinatorics::factorial(n),
            Self::Variations | Self::Combinations => {
                let k = u32::try_from(arguments[1]).map_err(|_| {
                    InvokeError::IntegerOutOfRange {
                        argument: 1,
                        value: arguments[1],
                    }
                })?;
                match self {
                    Self::Variations => combinatorics::variat(n, k),
                    Self::Combinations => combinatorics::combin(n, k),
                    Self::Factorial => unreachable!(),
                }
            }
        };
        result.ok_or(InvokeError::DomainOrOverflow)
    }
}

/// Invocation failures at the Rust runtime boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvokeError {
    WrongArity { expected: usize, actual: usize },
    IntegerOutOfRange { argument: usize, value: i64 },
    /// The mathematical domain is invalid or the exact result exceeds u128.
    DomainOrOverflow,
}

/// One stable handler registration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HandlerRegistration {
    pub id: &'static str,
    pub signature: HandlerSignature,
    pub handler: CoreHandler,
    /// Core handlers have no platform profile.
    pub profile: Option<&'static str>,
}

/// A registry declaration marked implemented or partial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HandlerDeclaration {
    pub declaration_id: &'static str,
    pub handler_id: Option<&'static str>,
    pub signature: HandlerSignature,
    /// A language-core declaration has no API profile.
    pub profile: Option<&'static str>,
}

/// Problems found while reconciling declarations and runtime registrations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryError {
    MissingHandler {
        declaration_id: &'static str,
    },
    DuplicateHandler {
        handler_id: &'static str,
    },
    StaleHandler {
        handler_id: &'static str,
    },
    SignatureMismatch {
        declaration_id: &'static str,
        handler_id: &'static str,
    },
    ProfileMismatch {
        declaration_id: &'static str,
        handler_id: &'static str,
    },
    InvalidDeclarationNamespace {
        declaration_id: &'static str,
    },
    InvalidHandlerNamespace {
        handler_id: &'static str,
    },
}

/// Canonical registrations for implemented language-core operations.
pub const CORE_HANDLERS: &[HandlerRegistration] = &[
    HandlerRegistration {
        id: "gfa.runtime.core.math.fact",
        signature: HandlerSignature::Integer1ToUnsigned128,
        handler: CoreHandler::Factorial,
        profile: None,
    },
    HandlerRegistration {
        id: "gfa.runtime.core.math.variat",
        signature: HandlerSignature::Integer2ToUnsigned128,
        handler: CoreHandler::Variations,
        profile: None,
    },
    HandlerRegistration {
        id: "gfa.runtime.core.math.combin",
        signature: HandlerSignature::Integer2ToUnsigned128,
        handler: CoreHandler::Combinations,
        profile: None,
    },
];

/// Resolve exactly one registered handler by its stable ID.
pub fn resolve_handler(id: &str) -> Option<&'static HandlerRegistration> {
    let mut matches = CORE_HANDLERS.iter().filter(|entry| entry.id == id);
    let first = matches.next()?;
    if matches.next().is_some() {
        None
    } else {
        Some(first)
    }
}

/// Reconcile implemented/partial declarations with the complete registration set.
///
/// Unsupported and not-implemented declarations must be omitted from
/// `declarations`.
pub fn validate_handler_registry(
    declarations: &[HandlerDeclaration],
    registrations: &[HandlerRegistration],
) -> Result<(), Vec<RegistryError>> {
    let mut errors = Vec::new();
    for declaration in declarations {
        if !declaration.declaration_id.starts_with("gfa.language.") {
            errors.push(RegistryError::InvalidDeclarationNamespace {
                declaration_id: declaration.declaration_id,
            });
        }
        let Some(id) = declaration.handler_id else {
            errors.push(RegistryError::MissingHandler {
                declaration_id: declaration.declaration_id,
            });
            continue;
        };
        if !id.starts_with("gfa.runtime.core.") {
            errors.push(RegistryError::InvalidHandlerNamespace { handler_id: id });
        }
        let matches: Vec<_> = registrations
            .iter()
            .filter(|entry| entry.id == id)
            .collect();
        match matches.as_slice() {
            [] => errors.push(RegistryError::MissingHandler {
                declaration_id: declaration.declaration_id,
            }),
            [entry] => {
                if entry.signature != declaration.signature {
                    errors.push(RegistryError::SignatureMismatch {
                        declaration_id: declaration.declaration_id,
                        handler_id: id,
                    });
                }
                if entry.profile != declaration.profile || declaration.profile.is_some() {
                    errors.push(RegistryError::ProfileMismatch {
                        declaration_id: declaration.declaration_id,
                        handler_id: id,
                    });
                }
            }
            _ => errors.push(RegistryError::DuplicateHandler { handler_id: id }),
        }
    }
    for entry in registrations {
        if !entry.id.starts_with("gfa.runtime.core.") {
            errors.push(RegistryError::InvalidHandlerNamespace { handler_id: entry.id });
        }
        if !declarations
            .iter()
            .any(|declaration| declaration.handler_id == Some(entry.id))
        {
            errors.push(RegistryError::StaleHandler { handler_id: entry.id });
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FACT: HandlerDeclaration = HandlerDeclaration {
        declaration_id: "gfa.language.math.fact",
        handler_id: Some("gfa.runtime.core.math.fact"),
        signature: HandlerSignature::Integer1ToUnsigned128,
        profile: None,
    };
    const VARIAT: HandlerDeclaration = HandlerDeclaration {
        declaration_id: "gfa.language.math.variat",
        handler_id: Some("gfa.runtime.core.math.variat"),
        signature: HandlerSignature::Integer2ToUnsigned128,
        profile: None,
    };
    const COMBIN: HandlerDeclaration = HandlerDeclaration {
        declaration_id: "gfa.language.math.combin",
        handler_id: Some("gfa.runtime.core.math.combin"),
        signature: HandlerSignature::Integer2ToUnsigned128,
        profile: None,
    };

    #[test]
    fn current_declarations_resolve_once_and_execute_manual_example() {
        assert_eq!(
            validate_handler_registry(&[FACT, VARIAT, COMBIN], CORE_HANDLERS),
            Ok(())
        );
        assert_eq!(
            resolve_handler(FACT.handler_id.unwrap())
                .unwrap()
                .handler
                .invoke(&[6]),
            Ok(720)
        );
        assert_eq!(
            resolve_handler(VARIAT.handler_id.unwrap())
                .unwrap()
                .handler
                .invoke(&[6, 2]),
            Ok(30)
        );
        assert_eq!(
            resolve_handler(COMBIN.handler_id.unwrap())
                .unwrap()
                .handler
                .invoke(&[6, 2]),
            Ok(15)
        );
    }

    #[test]
    fn rejects_missing_duplicate_and_stale_registrations() {
        assert!(
            validate_handler_registry(&[FACT], &[])
                .unwrap_err()
                .contains(&RegistryError::MissingHandler {
                    declaration_id: FACT.declaration_id
                })
        );
        let mut duplicate = CORE_HANDLERS.to_vec();
        duplicate.push(CORE_HANDLERS[0]);
        assert!(
            validate_handler_registry(&[FACT, VARIAT, COMBIN], &duplicate)
                .unwrap_err()
                .contains(&RegistryError::DuplicateHandler {
                    handler_id: FACT.handler_id.unwrap()
                })
        );
        assert!(
            validate_handler_registry(&[VARIAT, COMBIN], CORE_HANDLERS)
                .unwrap_err()
                .contains(&RegistryError::StaleHandler {
                    handler_id: FACT.handler_id.unwrap()
                })
        );
    }

    #[test]
    fn rejects_signature_and_profile_mismatches() {
        let bad_signature = [HandlerDeclaration {
            signature: HandlerSignature::Integer2ToUnsigned128,
            ..FACT
        }];
        assert!(
            validate_handler_registry(&bad_signature, CORE_HANDLERS)
                .unwrap_err()
                .contains(&RegistryError::SignatureMismatch {
                    declaration_id: FACT.declaration_id,
                    handler_id: FACT.handler_id.unwrap()
                })
        );
        let bad_profile = [HandlerDeclaration {
            profile: Some("amiga"),
            ..FACT
        }];
        assert!(
            validate_handler_registry(&bad_profile, CORE_HANDLERS)
                .unwrap_err()
                .contains(&RegistryError::ProfileMismatch {
                    declaration_id: FACT.declaration_id,
                    handler_id: FACT.handler_id.unwrap()
                })
        );
    }

    #[test]
    fn unsupported_declaration_has_no_fallback_and_bad_arguments_fail() {
        let unsupported = [HandlerDeclaration {
            handler_id: None,
            ..FACT
        }];
        assert!(validate_handler_registry(&unsupported, &[]).is_err());
        assert!(resolve_handler("gfa.runtime.core.math.not-implemented").is_none());
        assert_eq!(
            CoreHandler::Factorial.invoke(&[]),
            Err(InvokeError::WrongArity {
                expected: 1,
                actual: 0
            })
        );
        assert_eq!(
            CoreHandler::Variations.invoke(&[2, 3]),
            Err(InvokeError::DomainOrOverflow)
        );
        assert_eq!(
            CoreHandler::Factorial.invoke(&[-1]),
            Err(InvokeError::IntegerOutOfRange {
                argument: 0,
                value: -1
            })
        );
        assert_eq!(
            CoreHandler::Factorial.invoke(&[35]),
            Err(InvokeError::DomainOrOverflow)
        );
    }
}
