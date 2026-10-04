use crate::common::static_type::StaticType;
use crate::compiler::resolutions::{DeclId, Symbols};
use crate::compiler::semantic::MethodSignature;
use crate::compiler::symbol_table::Symbol;
use std::collections::{HashMap, HashSet};

/// Everything one REPL line's compile leaves behind for the next line to
/// resolve against: the top-level names it declared, the symbol ids it
/// interned, and where each global lives in the script frame. A file
/// compiles against the empty `GlobalEnv::default()`.
#[derive(Default, Clone)]
pub(crate) struct GlobalEnv {
    pub(crate) globals: HashMap<String, Symbol>,
    pub(crate) types: HashMap<String, Option<StaticType>>,
    pub(crate) struct_methods: HashMap<String, HashMap<String, MethodSignature>>,
    pub(crate) symbols: Symbols,
    pub(crate) next_decl_id: u32,
    pub(crate) decl_slots: HashMap<DeclId, u32>,
    pub(crate) slot_count: u32,
    pub(crate) immutable: HashSet<DeclId>,
}

impl GlobalEnv {
    /// Rolls back a line that compiled but failed at runtime: keeps this
    /// env's names, types, struct methods and immutability, but takes
    /// `after`'s slot count, next decl id and symbols, which are append-only.
    ///
    /// A name's static type is forgotten (set to unknown) if the failed
    /// line recorded a different one for it - it may have assigned, or
    /// redeclared and then rolled back, that global before erroring, so
    /// the old type can no longer be trusted.
    pub(crate) fn after_runtime_error(self, after: &GlobalEnv) -> GlobalEnv {
        let types = self
            .types
            .iter()
            .map(|(name, ty)| {
                if after.types.get(name) == Some(ty) {
                    (name.clone(), ty.clone())
                } else {
                    (name.clone(), None)
                }
            })
            .collect();
        GlobalEnv {
            types,
            slot_count: after.slot_count,
            next_decl_id: after.next_decl_id,
            symbols: after.symbols.clone(),
            ..self
        }
    }
}
