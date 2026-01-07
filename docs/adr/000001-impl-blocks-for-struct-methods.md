# ADR-000001: Impl Blocks for Struct Methods

## Status
Accepted

## Date
2026-01-07

## Context

Neon structs are currently pure data containers with no associated behavior. Users must write standalone functions to operate on struct instances, which lacks ergonomics and doesn't namespace behavior with data:

```neon
struct Point { x y }

// Current approach: standalone functions
fn distance(p1, p2) {
    val dx = p2.x - p1.x
    val dy = p2.y - p1.y
    return Math.sqrt(dx * dx + dy * dy)
}

val p1 = Point(0, 0)
val p2 = Point(3, 4)
print(distance(p1, p2))  // 5
```

This approach has drawbacks:
- Functions are not associated with their types
- No clear distinction between methods that operate on instances vs. utility functions
- Cannot use familiar `object.method()` syntax for user-defined types
- Discoverability suffers - users can't know what operations a type supports

## Decision Drivers

- **Ergonomics**: Enable `point.distance(other)` syntax familiar from other languages
- **Encapsulation**: Associate behavior with the types it operates on
- **Consistency**: User-defined structs should feel like built-in types (Array, String, etc.)
- **Simplicity**: Minimal new syntax, leverage existing infrastructure
- **Explicitness**: User controls mutation and return values

## Decision Outcome

Add Rust-style `impl` blocks that allow defining methods on structs:

```neon
struct Point { x y }

impl Point {
    // Instance method - receives instance as `self`
    fn distance(self, other) {
        val dx = other.x - self.x
        val dy = other.y - self.y
        return Math.sqrt(dx * dx + dy * dy)
    }
    
    // Mutating method - can modify instance fields via `mut self`
    fn translate(mut self, dx, dy) {
        self.x = self.x + dx
        self.y = self.y + dy
    }
    
    // Static method - no self parameter, called on type
    fn origin() {
        return Point(0, 0)
    }
}

// Usage
val p1 = Point.origin()      // static method call
val p2 = Point(3, 4)
print(p1.distance(p2))       // instance method call: 5
p1.translate(1, 1)           // mutating method call
```

### Method Types

1. **Instance methods** - First parameter is `self`, receives the instance immutably
2. **Mutating methods** - First parameter is `mut self`, can modify instance fields
3. **Static methods** - No `self` parameter, called via `Type.method()`

### Implementation Strategy

Extend `ObjStruct` to store methods alongside fields. Method lookup occurs at runtime using the existing type dispatch infrastructure. The `self` parameter is bound to the receiver instance at call time.

## Consequences

### Breaking Changes / Migration

None - this is a purely additive feature. Existing struct code continues to work unchanged.

### Performance Implications

- **Compilation**: Minor increase from parsing impl blocks and storing method definitions
- **Runtime**: Method lookup adds a HashMap access per call, comparable to existing field access
- **Memory**: Methods stored once per struct type definition, not per instance

## Implementation Plan

### Task 1: Add `Impl` Token and Scanning
**Depends on**: None
**Files**: `src/compiler/token.rs`, `src/compiler/scanner.rs`

- [ ] Add `Impl` variant to `TokenType` enum in `token.rs`
- [ ] Add `"impl"` keyword mapping in scanner's keyword detection
- [ ] Add unit tests for scanning `impl` keyword

### Task 2: Extend AST for Impl Blocks
**Depends on**: None
**Files**: `src/compiler/ast/mod.rs`

- [ ] Add `Stmt::Impl` variant:
  ```rust
  Impl {
      struct_name: String,
      methods: Vec<ImplMethod>,
      location: SourceLocation,
  }
  ```
- [ ] Add `ImplMethod` struct:
  ```rust
  pub struct ImplMethod {
      pub name: String,
      pub params: Vec<FunctionParam>,  // includes self/mut self as first param, or empty for static
      pub body: Vec<Stmt>,
      pub is_static: bool,             // true if no self parameter
      pub is_mutating: bool,           // true if `mut self`
      pub location: SourceLocation,
  }
  ```
- [ ] Add `FunctionParam` struct (reusable for regular functions later):
  ```rust
  pub struct FunctionParam {
      pub name: String,
      pub is_mutable: bool,  // for `mut self`
  }
  ```

### Task 3: Parse Impl Blocks
**Depends on**: Task 1, Task 2
**Files**: `src/compiler/parser.rs`

- [ ] Add `impl_block()` method to parse impl blocks:
  - Consume `impl` keyword
  - Parse struct name identifier
  - Consume `{`
  - Parse methods until `}`
- [ ] Add `impl_method()` method to parse individual methods:
  - Consume `fn` keyword
  - Parse method name
  - Parse parameter list, detecting `self`/`mut self` as first param
  - Set `is_static = true` if no self parameter
  - Set `is_mutating = true` if `mut self`
  - Parse method body
- [ ] Integrate into `declaration()` to handle `TokenType::Impl`
- [ ] Add parser tests for impl block syntax variations

### Task 4: Extend Symbol Table for Methods
**Depends on**: Task 2
**Files**: `src/compiler/symbol_table.rs`

- [ ] Extend `SymbolKind::Struct` to track methods:
  ```rust
  Struct {
      fields: Vec<String>,
      methods: HashMap<String, MethodSignature>,
  }
  ```
- [ ] Add `MethodSignature` struct:
  ```rust
  pub struct MethodSignature {
      pub arity: u8,        // parameter count excluding self
      pub is_static: bool,
      pub is_mutating: bool,
  }
  ```
- [ ] Add helper methods to register and lookup methods on struct symbols

### Task 5: Semantic Analysis for Impl Blocks
**Depends on**: Task 3, Task 4
**Files**: `src/compiler/semantic.rs`

- [ ] Handle `Stmt::Impl` in declaration collection pass:
  - Verify struct exists in symbol table
  - Register each method signature in the struct's symbol
  - Error if method name conflicts with field name
  - Error if duplicate method names
- [ ] Handle `Stmt::Impl` in analysis pass:
  - Enter new scope for each method body
  - Bind `self` as immutable parameter (or mutable for `mut self`)
  - Analyze method body statements
  - Validate field access on `self` matches struct fields
- [ ] Add semantic error types:
  - "Impl block for undefined struct 'X'"
  - "Method 'X' conflicts with field name"
  - "Duplicate method 'X' in impl block"
  - "Cannot assign to immutable self"
- [ ] Add semantic analysis tests

### Task 6: Extend Value System for Struct Methods
**Depends on**: None
**Files**: `src/common/mod.rs`

- [ ] Extend `ObjStruct` to include methods:
  ```rust
  pub struct ObjStruct {
      pub name: String,
      pub fields: Vec<String>,
      pub methods: HashMap<String, Rc<ObjFunction>>,
  }
  ```
- [ ] Update `Value::new_struct()` to initialize empty methods HashMap
- [ ] Add `ObjStruct::add_method()` helper
- [ ] Add `ObjStruct::get_method()` helper

### Task 7: Code Generation for Impl Blocks
**Depends on**: Task 3, Task 6
**Files**: `src/compiler/codegen.rs`

- [ ] Handle `Stmt::Impl` in code generation:
  - Look up the struct's `Value` from locals
  - For each method, compile as `ObjFunction`
  - Add compiled function to struct's methods HashMap
- [ ] Compile method bodies with `self` as first local:
  - For instance methods: `self` is slot 0, immutable
  - For mutating methods: `self` is slot 0, mutable
  - For static methods: no `self` binding
- [ ] Handle `self` in `generate_variable_expr()`:
  - Resolve `self` to local slot 0 within method context
- [ ] Track "inside impl method" state to know when `self` is valid
- [ ] Add codegen tests

### Task 8: VM Method Dispatch for User-Defined Methods
**Depends on**: Task 6
**Files**: `src/vm/impl.rs`, `src/vm/functions.rs`

- [ ] Modify method call handling in `fn_call()`:
  - When calling a method on an instance, first check `instance.struct.methods`
  - If found, bind receiver as first argument and call the function
  - If not found, fall back to native method lookup (existing behavior)
- [ ] Handle static method calls (`Type.method()`):
  - When `GetField` on a `Struct` value (not instance), look up static method
  - If found, call without receiver binding
- [ ] For mutating methods:
  - Pass the `Rc<RefCell<ObjInstance>>` so mutations are visible
- [ ] Add VM runtime error: "Undefined method 'X' for struct 'Y'"
- [ ] Add VM tests for method dispatch

### Task 9: Integration Tests
**Depends on**: Task 8
**Files**: `tests/scripts/impl_*.n`, `tests/neon_scripts.rs`

- [ ] Create `tests/scripts/impl_basic.n`:
  - Instance methods with self
  - Field access via self
  - Return values from methods
- [ ] Create `tests/scripts/impl_mutating.n`:
  - Mutating methods with `mut self`
  - Field modification
  - Verify mutations persist
- [ ] Create `tests/scripts/impl_static.n`:
  - Static methods without self
  - Factory pattern (e.g., `Point.origin()`)
  - Called via `Type.method()` syntax
- [ ] Create `tests/scripts/impl_chaining.n`:
  - Method chaining (user returns self explicitly)
  - Mixed instance and mutating calls
- [ ] Create `tests/scripts/impl_comprehensive.n`:
  - Complex example combining all features
  - Multiple impl blocks
  - Interaction with existing features (arrays, maps, etc.)
- [ ] Register new test files in `tests/neon_scripts.rs`

## Test Plan

### Unit Tests
- [ ] Scanner: `impl` keyword tokenization
- [ ] Parser: impl block AST construction, error cases
- [ ] Semantic: method registration, self binding, error detection
- [ ] Codegen: method compilation, self as local variable
- [ ] VM: method dispatch, static vs instance calls

### Integration Tests
- [ ] `impl_basic.n`: Instance methods with self and field access
- [ ] `impl_mutating.n`: Mutating methods with `mut self`
- [ ] `impl_static.n`: Static methods and factory pattern
- [ ] `impl_chaining.n`: Method chaining with explicit returns
- [ ] `impl_comprehensive.n`: Full feature integration
