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
