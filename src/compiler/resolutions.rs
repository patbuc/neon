use crate::compiler::ast::NodeId;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

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

/// Interns field, method, and type names into dense `u16` ids shared by
/// every chunk of one compile. Seeded with `BUILTIN_TYPE_NAMES` so builtin
/// type `BUILTIN_TYPE_NAMES[i]` always has symbol id `i` - the VM relies on
/// that to recognize builtin types by id.
#[derive(Debug)]
struct Symbols {
    ids: HashMap<Rc<str>, u16>,
    names: Vec<Rc<str>>,
}

impl Default for Symbols {
    fn default() -> Self {
        let mut symbols = Symbols {
            ids: HashMap::new(),
            names: Vec::new(),
        };
        for name in crate::common::method_registry::BUILTIN_TYPE_NAMES {
            symbols
                .intern(name)
                .expect("BUILTIN_TYPE_NAMES is far smaller than the u16 id space");
        }
        symbols
    }
}

impl Symbols {
    /// Interns `name`, returning its id. An already-interned name returns
    /// its existing id; a new name once 65,536 names are interned returns
    /// `None`.
    fn intern(&mut self, name: &str) -> Option<u16> {
        if let Some(&id) = self.ids.get(name) {
            return Some(id);
        }
        let id = u16::try_from(self.names.len()).ok()?;
        let name: Rc<str> = Rc::from(name);
        self.names.push(name.clone());
        self.ids.insert(name, id);
        Some(id)
    }

    fn names(&self) -> Rc<[Rc<str>]> {
        Rc::from(self.names.as_slice())
    }

    fn id(&self, name: &str) -> Option<u16> {
        self.ids.get(name).copied()
    }
}

/// Every name resolution the semantic pass made, keyed by AST node id.
#[derive(Debug, Default)]
pub struct Resolutions {
    uses: HashMap<NodeId, Res>,
    natives: HashMap<NodeId, usize>,
    decls: HashMap<NodeId, DeclId>,
    functions: HashMap<NodeId, FunctionResolution>,
    captured: HashSet<DeclId>,
    checked: HashSet<NodeId>,
    immutable: HashSet<DeclId>,
    symbols: Symbols,
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

    /// Marks a declaration that can't be reassigned.
    pub(crate) fn mark_immutable(&mut self, decl: DeclId) {
        self.immutable.insert(decl);
    }

    /// Marks a name use as one that can run before the hoisted `fn` it
    /// resolves to has bound its slot, so codegen must emit a runtime
    /// initialization check for it.
    pub(crate) fn mark_checked(&mut self, id: NodeId) {
        self.checked.insert(id);
    }

    /// See `Symbols::intern`.
    pub(crate) fn intern_symbol(&mut self, name: &str) -> Option<u16> {
        self.symbols.intern(name)
    }

    /// Every interned field, method, and type name, indexed by symbol id,
    /// for embedding into a compiled chunk.
    pub fn symbol_names(&self) -> Rc<[Rc<str>]> {
        self.symbols.names()
    }

    /// The id `name` was interned under. Panics if it was never interned -
    /// every field, method, and type name codegen looks up here must have
    /// gone through `intern_symbol` during semantic analysis first.
    pub fn symbol(&self, name: &str) -> u16 {
        self.symbols
            .id(name)
            .unwrap_or_else(|| panic!("no symbol interned for {:?}", name))
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
    pub fn native(&self, id: NodeId) -> Option<usize> {
        self.natives.get(&id).copied()
    }

    /// Whether some nested function captures this declaration.
    pub fn is_captured(&self, decl: DeclId) -> bool {
        self.captured.contains(&decl)
    }

    /// Whether this name use needs a runtime initialization check.
    pub fn is_checked(&self, id: NodeId) -> bool {
        self.checked.contains(&id)
    }

    /// Whether this declaration can't be reassigned.
    pub fn is_immutable(&self, decl: DeclId) -> bool {
        self.immutable.contains(&decl)
    }
}
