use crate::common::method_registry::NATIVE_METHODS;
use crate::compiler::ast::{EnumVariant, Stmt};
use crate::compiler::resolutions::{DeclId, Resolutions, Symbols};
use crate::compiler::semantic::MethodSignature;
use std::collections::HashMap;

/// One exported name: what it is and, for the kinds that have one, the
/// global slot it lives in.
#[derive(Debug, Clone, PartialEq)]
pub enum Export {
    Function {
        arity: u8,
        slot: u32,
    },
    Variable {
        slot: u32,
    },
    Struct {
        fields: Vec<String>,
        methods: HashMap<String, MethodSignature>,
        slot: u32,
    },
    /// Variants compile to constants, so an enum has no slot.
    Enum {
        variants: Vec<EnumVariant>,
    },
    /// A builtin module's function: a registry row, called without a slot.
    Native {
        index: usize,
        arity: u8,
    },
}

impl Export {
    pub fn slot(&self) -> Option<u32> {
        match self {
            Export::Function { slot, .. }
            | Export::Variable { slot, .. }
            | Export::Struct { slot, .. } => Some(*slot),
            Export::Enum { .. } | Export::Native { .. } => None,
        }
    }
}

/// A module's exports, keyed by the symbol id of the exported name.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExportTable {
    exports: HashMap<u16, Export>,
}

impl ExportTable {
    /// Collects the `export`ed declarations of a compiled module, given the
    /// slot codegen assigned each declaration.
    #[allow(clippy::expect_used)]
    pub(crate) fn build(
        ast: &[Stmt],
        resolutions: &Resolutions,
        decl_slots: &HashMap<DeclId, u32>,
        struct_methods: &HashMap<DeclId, HashMap<String, MethodSignature>>,
    ) -> ExportTable {
        let mut table = ExportTable::default();
        let slot = |id| {
            *decl_slots
                .get(&resolutions.decl(id))
                .expect("codegen gives every top-level declaration a slot")
        };
        for stmt in ast {
            let Stmt::Export { declaration, .. } = stmt else {
                continue;
            };
            let mut add = |name: &str, export| {
                table.exports.insert(resolutions.symbol(name), export);
            };
            match declaration.as_ref() {
                Stmt::Fn {
                    name, params, id, ..
                } => add(
                    name,
                    Export::Function {
                        arity: params.len() as u8,
                        slot: slot(*id),
                    },
                ),
                Stmt::Val { pattern, .. } | Stmt::Var { pattern, .. } => {
                    for binding in pattern.bindings() {
                        add(
                            &binding.name,
                            Export::Variable {
                                slot: slot(binding.id),
                            },
                        );
                    }
                }
                Stmt::Struct {
                    name, fields, id, ..
                } => add(
                    name,
                    Export::Struct {
                        fields: fields.iter().map(|f| f.name.clone()).collect(),
                        methods: struct_methods
                            .get(&resolutions.decl(*id))
                            .cloned()
                            .unwrap_or_default(),
                        slot: slot(*id),
                    },
                ),
                Stmt::Enum { name, variants, .. } => add(
                    name,
                    Export::Enum {
                        variants: variants.clone(),
                    },
                ),
                _ => {}
            }
        }
        table
    }

    /// The exports of the builtin module `path` (e.g. `std/math`), one per
    /// registry row keyed by that path. `None` when interning an export
    /// name would overflow the symbol id space.
    pub(crate) fn builtin(path: &str, symbols: &mut Symbols) -> Option<ExportTable> {
        let mut table = ExportTable::default();
        for (index, (module, name, callable)) in NATIVE_METHODS.iter().enumerate() {
            if *module == path {
                let export = Export::Native {
                    index,
                    arity: callable.arity(),
                };
                table.exports.insert(symbols.intern(name)?, export);
            }
        }
        Some(table)
    }

    pub fn symbols(&self) -> impl Iterator<Item = u16> + '_ {
        self.exports.keys().copied()
    }

    pub fn get(&self, symbol: u16) -> Option<&Export> {
        self.exports.get(&symbol)
    }
}
