use crate::compiler::ast::NodeId;
use std::collections::{HashMap, HashSet};

/// Identity of a declaration (a `val`/`var`/`fn`/`struct`/loop variable/parameter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclId(pub u32);

/// Where a name use lives at runtime, from the point of view of the function containing the use.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Res {
    /// A slot of the current function (includes the script's top-level names used from the script itself).
    Local(DeclId),
    /// A script top-level name (scope depth 0 of the script) used from inside a function.
    Global(DeclId),
    /// Index into the current function's upvalue array.
    Upvalue(u32),
    /// Index into `stdlib::BUILTIN_VALUES`.
    Builtin(u32),
}

/// A local of the immediately enclosing function, or one of its upvalues.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Capture {
    Local(DeclId),
    Upvalue(u32),
}

/// What a function captures and how its parameters are named.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunctionResolution {
    pub params: Vec<DeclId>,
    pub upvalues: Vec<Capture>,
}

/// Every name resolution the semantic pass made, keyed by AST node id.
#[derive(Debug, Default)]
pub struct Resolutions {
    uses: HashMap<NodeId, Res>,
    natives: HashMap<NodeId, usize>,
    decls: HashMap<NodeId, DeclId>,
    functions: HashMap<NodeId, FunctionResolution>,
    captured: HashSet<DeclId>,
}

impl Resolutions {
    pub(crate) fn record_use(&mut self, id: NodeId, res: Res) {
        self.uses.insert(id, res);
    }

    pub(crate) fn record_native(&mut self, id: NodeId, index: usize) {
        self.natives.insert(id, index);
    }

    pub(crate) fn record_decl(&mut self, id: NodeId, decl: DeclId) {
        self.decls.insert(id, decl);
    }

    pub(crate) fn record_function(&mut self, id: NodeId, resolution: FunctionResolution) {
        self.functions.insert(id, resolution);
    }

    pub(crate) fn mark_captured(&mut self, decl: DeclId) {
        self.captured.insert(decl);
    }

    /// The resolution of an `Expr::Variable` or `Expr::Assign` node.
    pub fn res(&self, id: NodeId) -> Res {
        *self
            .uses
            .get(&id)
            .unwrap_or_else(|| panic!("no resolution recorded for {:?}", id))
    }

    /// The `DeclId` a declaration node was assigned.
    pub fn decl(&self, id: NodeId) -> DeclId {
        *self
            .decls
            .get(&id)
            .unwrap_or_else(|| panic!("no declaration recorded for {:?}", id))
    }

    /// The captures and parameter `DeclId`s of a function node.
    pub fn function(&self, id: NodeId) -> &FunctionResolution {
        self.functions
            .get(&id)
            .unwrap_or_else(|| panic!("no function resolution recorded for {:?}", id))
    }

    /// The method-registry index a call dispatches to, if it was resolved as a native.
    #[allow(dead_code)]
    pub fn native(&self, id: NodeId) -> Option<usize> {
        self.natives.get(&id).copied()
    }

    /// Whether some nested function captures this declaration.
    pub fn is_captured(&self, decl: DeclId) -> bool {
        self.captured.contains(&decl)
    }
}
