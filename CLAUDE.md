# Claude Code Instructions

This file contains custom instructions for Claude Code when working on this project.

## About Neon

Neon is a dynamically-typed programming language implemented in Rust.

**Architecture**:

- `src/compiler/` - Lexer, parser, AST, semantic analysis, code generation
- `src/vm/` - Bytecode interpreter, value stack, call frames, instruction execution
- `src/common/` - Shared types, opcodes, object model, bytecode format
- `tests/` - Integration tests using embedded Neon scripts

## Neon Language Syntax

Understanding Neon's syntax is essential for implementing features and writing tests.

**Basic Syntax Example**:

```neon
// Variables (var is mutable, val is immutable)
var x = 10
val name = "Alice"

// Functions
fn add(a, b) {
    return a + b
}

// Control flow
if (x > 5) {
    print("Greater than 5")
}

// Loops
for (var i = 0; i < 10; i = i + 1) {
    print(i)
}

// For-in loops
for (item in array) {
    print(item)
}
```

**Data Types**:

```neon
// Numbers
var num = 42
var pi = 3.14

// Strings with interpolation
var greeting = "Hello ${name}"
var result = "${x} + ${y} = ${x + y}"

// Arrays
val arr = [1, 2, 3, 4, 5]
arr.push(6)
print(arr[0])      // 1
print(arr.size())  // 6

// Maps (dictionaries)
var person = {"name": "Alice", "age": 30}
person["city"] = "New York"
print(person["name"])

// Structs
struct Point {
    x
    y
}
val pt = Point(10, 20)
print(pt.x)  // 10
pt.x = 15    // Mutable fields
```

**Key Language Features**:

- **Dynamic typing** - No type declarations needed
- **First-class functions** - Functions can be passed as values
- **Closures** - Functions capture their environment
- **String interpolation** - `"${expression}"` syntax
- **For-in loops** - Iterate over arrays, maps, sets, ranges
- **Ranges** - `1..10` (exclusive), `1..=10` (inclusive)
- **Built-in collections** - Arrays, Maps, Sets
- **Structs** - User-defined data types
- **Method calls** - `.push()`, `.size()`, etc.

**Code Style**:

- No Semicolons except for for-loops
- C-style comments `//` for single line
- Blocks use curly braces `{}`
- Functions use `fn` keyword
- Variables use `var` (mutable) or `val` (immutable)

**Example - Fibonacci**:

```neon
fn fibonacci(n) {
    if (n <= 1) {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

for (var i = 0; i < 10; i = i + 1) {
    print(fibonacci(i))
}
```

## Implementation Flow

When implementing language features, changes typically flow through:

1. **Lexer/Scanner** - Tokenize new syntax
2. **Parser** - Build AST nodes
3. **Semantic Analysis** - Validate semantics (if needed)
4. **Code Generation** - Emit bytecode opcodes
5. **VM** - Execute instructions

**Module Boundaries and Dependencies**:

```
common/ ← compiler/ ← vm/ ← tests/
   ↑         ↑         ↑
   └─────────┴─────────┘
```

**Dependency Rules**:

- `common/` - No dependencies on compiler or VM (shared types, opcodes, values only)
- `compiler/` - Can use common, CANNOT use VM
- `vm/` - Can use common and compiler (for bytecode execution)
- `tests/` - Can use everything

**Never**: VM → Compiler (breaks architecture)

**Key Patterns**:

- Standard library uses macro-based argument extraction (see `src/vm/stdlib.rs`)
- VM operations use a value stack for operands
- Objects are heap-allocated with reference counting
- Tests use embedded Neon syntax with `#[test]` attributes
- Stack-based execution: push operands, execute operation, push result

## Implementation Cookbook

This section provides concrete patterns and workflows for common implementation tasks.

### Where Does New Code Go?

**Decision Tree**:

- **Built-in function** (like `print()`, `len()`) → `src/vm/stdlib.rs`
- **Language operator** (like `+`, `-`, `**`) → Parser + Codegen + VM + Opcodes
- **New value type** (like Array, Map) → `src/common/value.rs` + object model
- **Control flow** (like `while`, `for`) → Parser + Codegen + VM opcodes
- **Runtime behavior** (execution logic) → `src/vm/mod.rs`
- **New token/keyword** → `src/compiler/lexer.rs` + Parser

### Adding a Standard Library Function

**Pattern** (see `src/vm/stdlib.rs`):

```rust
fn native_print(args: &[Value]) -> Result<Value, RuntimeError> {
    // Use extract_args! macro for type-safe argument extraction
    extract_args!(args, [String => s]);
    println!("{}", s);
    Ok(Value::Null)
}
```

**Steps**:

1. Add function to `src/vm/stdlib.rs`
2. Register in `VM::new()` with `self.define_native("name", native_fn)`
3. Use `extract_args!` macro for argument validation
4. Return `Result<Value, RuntimeError>`
5. Add integration test in `tests/stdlib.rs`

**Reference Examples**:

- Simple function: `native_print()`
- Multiple args: Check existing stdlib functions
- Type validation: Uses `extract_args!` macro

### Adding a Binary Operator

**Example**: Adding `**` (exponentiation)

**Step-by-step**:

1. **Lexer** (`src/compiler/lexer.rs`)
   ```rust
   // Add token type
   TokenKind::StarStar
   ```

2. **Parser** (`src/compiler/parser.rs`)
   ```rust
   // Add to precedence table and binary parsing
   fn power(&mut self) -> Result<Expr, ParseError> {
       // Parse with appropriate precedence
   }
   ```

3. **Opcodes** (`src/common/opcodes.rs`)
   ```rust
   pub enum OpCode {
       // ...
       Power,
   }
   ```

4. **Codegen** (`src/compiler/codegen.rs`)
   ```rust
   // Emit opcode in binary expression handling
   BinaryOp::Power => self.emit(OpCode::Power),
   ```

5. **VM** (`src/vm/mod.rs`)
   ```rust
   OpCode::Power => {
       let b = self.pop()?;
       let a = self.pop()?;
       let result = a.pow(&b)?;  // Implement pow() on Value
       self.push(result);
   }
   ```

6. **Test** (`tests/operators.rs`)
   ```rust
   #[test]
   fn can_exponentiate() {
       let result = run_neon("2 ** 3");
       assert_eq!(result.unwrap(), Value::Number(8.0));
   }
   ```

**Reference**: See `OpCode::Add`, `OpCode::Multiply` for similar patterns

### Adding a Control Flow Statement

**Example**: `while` loop

**Steps**:

1. **Lexer** - Add `TokenKind::While` keyword
2. **Parser** - Parse condition and body into AST node
3. **Codegen** - Emit loop with jump instructions:
   ```rust
   // Pattern: loop_start → condition → JumpIfFalse(end) → body → Jump(start) → end
   let loop_start = self.current_offset();
   self.compile_expr(condition)?;
   let exit_jump = self.emit_jump(OpCode::JumpIfFalse);
   self.compile_block(body)?;
   self.emit_loop(loop_start);
   self.patch_jump(exit_jump);
   ```
4. **VM** - Execute jump opcodes (already implemented)
5. **Test** - Add loop tests

**Reference**: See `if` statement implementation for conditional jumps

### Integration Test Pattern

**Standard Pattern** (`tests/*.rs`):

```rust
#[test]
fn test_feature_name() {
    let source = r#"
        // Neon code here
        let x = [1, 2, 3];
        x[1]
    "#;

    let result = run_neon_code(source);
    assert_eq!(result.unwrap(), Value::Number(2.0));
}

#[test]
fn test_error_case() {
    let result = run_neon_code("1 / 0");
    assert!(result.is_err());
}
```

**Testing Guidelines**:

- One test per feature/behavior
- Test both success and error cases
- Use descriptive test names (`can_parse_arrays`, `handles_division_by_zero`)
- Embedded Neon code uses raw strings `r#"..."#`
- Test output values and error conditions

### Stack Operations in VM

**Pattern**: All operations follow this flow:

```rust
// Binary operation pattern
let right = self .pop() ?;  // Pop in reverse order
let left = self .pop() ?;
let result = left.operation( & right) ?;
self .push(result);

// Unary operation pattern
let value = self .pop() ?;
let result = value.operation() ?;
self .push(result);
```

**Critical Rules**:

- Always pop operands before computing
- Pop in reverse order for binary ops (right then left)
- Always push result back to stack
- Propagate errors with `?`

### Value Type Implementation

**When adding new types** (like Array, Map):

1. **Define in** `src/common/value.rs`:
   ```rust
   pub enum Value {
       // ...
       Array(Rc<RefCell<Vec<Value>>>),
   }
   ```

2. **Implement operations**:
    - Display, PartialEq, Clone
    - Type checking methods
    - Conversion methods

3. **Add opcodes** for type-specific operations

4. **Update match expressions** everywhere Value is matched

5. **Test thoroughly** - construction, operations, edge cases

### Common Patterns Reference

**When implementing similar features, refer to these**:

| Feature Type    | Reference Implementation | Location              |
|-----------------|--------------------------|-----------------------|
| Binary operator | `OpCode::Add`            | `src/vm/mod.rs`       |
| Stdlib function | `native_print()`         | `src/vm/stdlib.rs`    |
| Control flow    | `if/else` jumps          | Codegen + VM          |
| Value type      | `Value::String`          | `src/common/value.rs` |
| Error handling  | `RuntimeError` variants  | `src/common/error.rs` |

## Rust Code Guidelines

**Error Handling**:

- NEVER use `unwrap()` or `expect()` in production code paths (compiler, VM, stdlib)
- Use `Result<T, E>` for recoverable errors
- Use proper error types from `src/common/error.rs`
- Panics are acceptable only in test code or truly unreachable branches
- Always propagate errors with `?` operator
- Return meaningful error messages

**Example - Correct Error Handling**:

```rust
// ✅ Good
fn divide(a: f64, b: f64) -> Result<f64, RuntimeError> {
    if b == 0.0 {
        return Err(RuntimeError::DivisionByZero);
    }
    Ok(a / b)
}

// ❌ Bad - never do this in production code
fn divide(a: f64, b: f64) -> f64 {
    assert!(b != 0.0);  // Panic instead of error
    a / b
}
```

**Testing Requirements**:

- All new language features MUST have integration tests in `tests/`
- VM opcodes need both unit tests (if applicable) and integration tests
- Standard library functions need test coverage
- Run `cargo test` before committing - all tests must pass
- Run `cargo clippy` and address warnings
- Test both success and error cases
- Use descriptive test names

**Performance Considerations**:

- The VM is performance-critical - avoid unnecessary allocations in hot paths
- Be mindful of clone() operations on heap objects
- Consider using references and borrowing where possible
- Stack operations should be fast (no allocations in push/pop)
- Profile performance-sensitive changes if uncertain

**Code Style**:

- Follow standard Rust conventions (rustfmt)
- Use descriptive variable names
- Keep functions focused and modular
- Document complex algorithms with comments explaining WHY, not WHAT
- Match expressions should be exhaustive
- Use early returns for error cases

### Common Mistakes to Avoid

**VM Implementation**:

- ❌ Forgetting to pop operands from stack before operation
- ❌ Popping operands in wrong order (binary ops: pop right first, then left)
- ❌ Not pushing result back to stack after operation
- ❌ Missing error propagation (always use `?`)

**Opcodes**:

- ❌ Adding opcodes without updating `OpCode::to_string()` for debugging
- ❌ Not handling new opcodes in VM's match expression
- ❌ Incorrect opcode arguments (size mismatches)

**Values**:

- ❌ Not handling all Value variants in match expressions
- ❌ Forgetting to implement Clone/PartialEq for new types
- ❌ Missing type checks before operations

**Testing**:

- ❌ Only testing success cases (always test errors too)
- ❌ Tests with `unwrap()` that could fail silently
- ❌ Missing edge cases (empty arrays, zero, null, etc.)

## Git Commit Messages

**IMPORTANT**: Never add watermarks, signatures, or co-authorship attributions to commit messages.

This project uses a verb-based, English-first commit style optimized for clarity and long-term readability.

### Non-Negotiable Rules (Watermarking & Attribution)

- Never add watermarks, signatures, or AI attributions
- DO NOT add "Generated with Claude Code" (or similar) footers
- DO NOT add Co-Authored-By: trailers
- Commit messages must appear as if written by a human contributor

### Commit Message Style

#### Format

**<Verb> <what>** \[<context>\]

Examples:

- Add array support to parser
- Refactor cache invalidation logic
- Remove legacy OAuth flow
- Fix crash on empty input

Rules:

- No type prefixes (feat:, fix:, refactor:, etc.)
- No scopes or taxonomy in the subject line
- One clear intent per commit
- Use imperative, present tense
- Keep the subject line concise and professional

### Intent Is Expressed by the Verb

We do not encode commit types via prefixes or labels.
The verb itself communicates intent.

Common patterns:

- Feature (new behavior)
  Add, Introduce, Implement, Support
- Fix (bug or incorrect behavior)
  Fix, Correct, Handle
- Refactor (no behavior change)
  Refactor, Simplify, Restructure
- Cleanup (removal / polish)
  Remove, Clean up, Trim
- Meta (docs, tooling, infra, deps)
  Update, Adjust, Bump, Document

If behavior changes, do not call it a refactor.

### Commit Body (Optional, Use Sparingly)

Most commits do not require a body.

Use one only when the reasoning or trade-offs are not obvious.

Example:

Add array support to parser

Implements array literal parsing and validation.
This enables nested array expressions in the AST.

Guidelines:

- Explain why, not line-by-line changes
- Do not list files or obvious diffs
- Keep it short and factual

#### Explicitly Disallowed

- ❌ Type-prefixed subjects (feat:, fix:, etc.)
- ❌ chore
- ❌ File lists or mechanical summaries
- ❌ AI watermarks, signatures, or co-author attributions
- ❌ Noisy or verbose commit messages

#### Correct Example

Add array support to parser

Implements array literal parsing and validation.

#### Incorrect Examples (DO NOT DO THIS)

feat: Add array support to parser

Add array support to parser

🤖 Generated with Claude Code

Co-Authored-By: Claude <noreply@anthropic.com>

Add array support to parser

Changes:

- Modified parser.rs
- Updated ast.rs
- Added tests

### Final Rule

If git log reads like clear, professional English written by a human,
the commit message is correct.

## Contribution Workflow

**Before Implementation**:

- For complex features, consider using `/plan-feature` to create a detailed plan first
- For architectural changes or multiple implementation approaches, ask questions before coding
- Understand existing patterns by reading similar implementations in the codebase
- Check the Implementation Cookbook above for relevant patterns

**During Implementation**:

- Follow the Implementation Cookbook patterns for the feature type
- Keep commits atomic and focused on a single change
- Run tests frequently: `cargo test`
- Check for warnings: `cargo clippy`
- Ensure code compiles at each step
- Verify no `unwrap()` or `expect()` in production code

**Before Committing**:

- All tests must pass (`cargo test`)
- No clippy warnings unless explicitly justified
- Code follows Rust conventions
- Commit message follows guidelines above
- No watermarks or AI attribution in commits

## Claude Code Orchestration

This project has a sophisticated multi-agent orchestration system in `.claude/`:

**Available Commands**:

- `/build-feature "description"` - Fully automated feature development (planning → implementation → testing → PR)
- `/plan-feature "description"` - Create detailed implementation plan
- `/implement-task N` - Implement specific task from plan
- `/run-tests` - Execute test suite with analysis
- `/create-pr` - Create GitHub pull request
- `/review-pr` - Automated code review

**When to Use**:

- Use `/plan-feature` for complex features before implementation
- Use `/build-feature` for end-to-end automation
- Manual workflow gives more control: plan → implement → test → PR

See `.claude/ORCHESTRATION.md` for complete documentation.

## Summary

**Code Quality**:

- ✅ No `unwrap()` in production code
- ✅ All features have tests
- ✅ Code passes `cargo test` and `cargo clippy`
- ✅ Follow existing patterns in codebase
- ✅ Proper error handling with Result types

**Commit Messages**:

- ✅ Clean, professional messages focused on intent
- ✅ Verb-based subjects without type prefixes
- ❌ No watermarks or attribution footers
- ❌ No file lists or obvious details from git diff

**Implementation**:

- ✅ Follow Implementation Cookbook patterns
- ✅ Respect module boundaries
- ✅ Reference canonical examples
- ✅ Test both success and error cases
