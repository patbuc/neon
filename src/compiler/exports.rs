use crate::compiler::ast::Stmt;
use crate::compiler::resolutions::{DeclId, Resolutions};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportKind {
    Function { arity: u8 },
    Variable { mutable: bool },
}

/// One exported name: what it is and the global slot it lives in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Export {
    pub kind: ExportKind,
    pub slot: u32,
}

/// A module's exports, keyed by the symbol id of the exported name.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExportTable {
    exports: HashMap<u16, Export>,
}

impl ExportTable {
    /// Collects the `export`ed `fn`/`val`/`var` declarations of a compiled
    /// module, given the slot codegen assigned each declaration.
    pub(crate) fn build(
        ast: &[Stmt],
        resolutions: &Resolutions,
        decl_slots: &HashMap<DeclId, u32>,
    ) -> ExportTable {
        let mut table = ExportTable::default();
        let mut add = |name: &str, id, kind| {
            let slot = decl_slots[&resolutions.decl(id)];
            table
                .exports
                .insert(resolutions.symbol(name), Export { kind, slot });
        };
        for stmt in ast {
            let Stmt::Export { declaration, .. } = stmt else {
                continue;
            };
            match declaration.as_ref() {
                Stmt::Fn {
                    name, params, id, ..
                } => add(
                    name,
                    *id,
                    ExportKind::Function {
                        arity: params.len() as u8,
                    },
                ),
                Stmt::Val { pattern, .. } | Stmt::Var { pattern, .. } => {
                    let mutable = matches!(declaration.as_ref(), Stmt::Var { .. });
                    for binding in pattern.bindings() {
                        add(&binding.name, binding.id, ExportKind::Variable { mutable });
                    }
                }
                _ => {}
            }
        }
        table
    }

    pub fn get(&self, symbol: u16) -> Option<&Export> {
        self.exports.get(&symbol)
    }
}
