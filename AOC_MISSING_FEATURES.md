# Advent of Code Missing Features Analysis

## Executive Summary

Based on comprehensive analysis of the Neon codebase and typical Advent of Code requirements, here are the critical missing features needed to successfully solve AoC puzzles.

## Priority 1: Essential Features (Must Have)

### 1. File I/O Operations
**Why**: Every AoC challenge provides input as a text file that must be read
**Current Status**: ❌ Not implemented
**Required Capabilities**:
- Read entire file as string: `File.read(path)`
- Read file as array of lines: `File.readLines(path)`
- Write to file: `File.write(path, content)`

### 2. Command-Line Arguments
**Why**: Need to specify input file path when running program
**Current Status**: ❌ Not implemented
**Required Capabilities**:
- Access CLI args: `args[0]`, `args[1]`, etc.
- Get argument count: `args.length()`

### 3. For-Each Loops (Collection Iteration)
**Why**: AoC constantly requires iterating over arrays/lines
**Current Status**: ❌ Not implemented (only C-style for loops exist)
**Required Syntax**:
```neon
for (element in array) {
    print element
}

for (line in lines) {
    // process line
}
```

### 4. String Methods - Extended
**Why**: Heavy text parsing in AoC
**Current Status**: ⚠️ Partial - has `.split()`, `.substring()`, `.replace()`
**Still Missing**:
- `.trim()` - Remove whitespace
- `.startsWith(prefix)` - Check prefix
- `.endsWith(suffix)` - Check suffix
- `.indexOf(substring)` - Find position
- `.charAt(index)` - Get character at index
- `.toUpperCase()` / `.toLowerCase()` - Case conversion

### 5. Array Methods - Extended
**Why**: Extensive array manipulation in AoC
**Current Status**: ⚠️ Partial - has `.push()`, `.pop()`, `.contains()`
**Still Missing**:
- `.map(fn)` - Transform elements
- `.filter(fn)` - Select elements
- `.reduce(fn, initial)` - Fold/aggregate
- `.sort()` - Sort array
- `.reverse()` - Reverse array
- `.slice(start, end)` - Extract subarray
- `.join(delimiter)` - Join to string
- `.indexOf(element)` - Find index
- `.sum()` - Sum numeric array
- `.min()` / `.max()` - Min/max of array

### 6. Break/Continue Statements
**Why**: Early loop termination in search algorithms
**Current Status**: ❌ Not implemented
**Required Syntax**:
```neon
while (true) {
    if (condition) {
        break  // Exit loop
    }
    if (skip_condition) {
        continue  // Next iteration
    }
}
```

## Priority 2: Highly Recommended

### 7. Range Operator
**Why**: Common pattern in AoC (e.g., "for each number 1 to 100")
**Current Status**: ❌ Not implemented
**Required Syntax**:
```neon
for (i in 1..10) {  // Exclusive end
    print i
}

for (i in 1..=10) {  // Inclusive end
    print i
}
```

### 8. String Interpolation
**Why**: Easier debugging and output formatting
**Current Status**: ❌ Not implemented
**Required Syntax**:
```neon
val name = "Alice"
val score = 42
print "Player ${name} scored ${score} points"
```

### 9. Tuple Type
**Why**: Return multiple values from functions (common in grid problems)
**Current Status**: ❌ Not implemented
**Required Syntax**:
```neon
fn get_position() {
    return (10, 20)
}

val (x, y) = get_position()
```

### 10. Integer Division Operator
**Why**: AoC often needs integer math (not float)
**Current Status**: ⚠️ Only has `/` which returns float
**Required**:
- `//` operator for integer division
- Or: `Math.floor(a / b)` works as workaround

### 11. Character Type and Operations
**Why**: Many AoC puzzles work with individual characters
**Current Status**: ⚠️ Strings exist but no char type
**Required**:
- Access char from string: `str[0]` or `str.charAt(0)`
- Convert char to ASCII: `char.toCode()` or `charCode("a")`
- Convert ASCII to char: `fromCharCode(97)`

### 12. Regex Support
**Why**: Complex parsing tasks (not all AoC needs this)
**Current Status**: ❌ Not implemented
**Required**:
- Pattern matching: `Regex.match(pattern, text)`
- Pattern replacement: `Regex.replace(pattern, replacement, text)`
- Regex methods on strings: `str.match(pattern)`

## Priority 3: Nice to Have

### 13. Lambda/Anonymous Functions
**Why**: Cleaner map/filter/reduce operations
**Current Status**: ❌ Not implemented
**Required Syntax**:
```neon
val doubled = arr.map(fn(x) { return x * 2 })
// or
val doubled = arr.map(fn(x) => x * 2)
```

### 14. Default Function Parameters
**Why**: Convenience for optional parameters
**Current Status**: ❌ Not implemented
**Required Syntax**:
```neon
fn greet(name, greeting = "Hello") {
    print greeting + " " + name
}
```

### 15. Switch/Match Statements
**Why**: Cleaner branching for multiple cases
**Current Status**: ❌ Not implemented
**Required Syntax**:
```neon
match value {
    1 => print "one"
    2 => print "two"
    _ => print "other"
}
```

### 16. Error Handling (Try/Catch)
**Why**: Graceful handling of parse errors, file not found, etc.
**Current Status**: ❌ Not implemented
**Required Syntax**:
```neon
try {
    val num = str.toInt()
    print num
} catch (e) {
    print "Error: " + e
}
```

### 17. Multi-dimensional Array Support
**Why**: Grid-based AoC problems
**Current Status**: ⚠️ Can use nested arrays but no special syntax
**Required**:
- Works now: `val grid = [[1,2], [3,4]]`
- Access: `grid[0][1]`
- Helper: `Array.fill(rows, cols, value)`

### 18. Type Conversion Improvements
**Current Status**: ⚠️ Has `.toInt()`, `.toFloat()`, `.toBool()`
**Missing**:
- Array to Set: `set.fromArray(arr)` or `Set(arr)`
- Array from range: `Array.range(1, 10)`

## Priority 4: Advanced (Can Wait)

### 19. Module System
**Why**: Organize helper functions across multiple files
**Current Status**: ❌ Not implemented
**Required**: `import`, `export` keywords

### 20. Recursion Optimization
**Current Status**: ❌ Unknown if tail-call optimization exists
**Required**: Tail-call optimization for recursive algorithms

### 21. BigInt Support
**Why**: Some AoC puzzles have very large numbers
**Current Status**: ❌ Only has f64 numbers
**Required**: Arbitrary precision integers

### 22. Memoization/Caching
**Why**: Dynamic programming problems
**Current Status**: ❌ Must implement manually
**Required**: Built-in memoization decorator or helper

## Minimal Viable Feature Set for AoC

To solve **most** Advent of Code puzzles, you need at minimum:

1. ✅ **File.read()** / **File.readLines()** - Read input files
2. ✅ **Command-line arguments** - Specify input file
3. ✅ **For-each loops** - Iterate collections easily
4. ✅ **String: .trim(), .startsWith(), .indexOf()** - Basic parsing
5. ✅ **Array: .map(), .filter(), .sort(), .join()** - Data transformation
6. ✅ **Break/Continue** - Loop control

With these 6 features, you can solve approximately 70-80% of AoC puzzles.

## Recommended Implementation Order

Based on dependencies and impact:

1. **File I/O** - Most fundamental, blocks everything else
2. **For-each loops** - Makes iteration much cleaner
3. **Break/Continue** - Simple syntax addition, high utility
4. **String methods (trim, startsWith, indexOf)** - Small additions, high utility
5. **Array methods (map, filter, sort, join)** - Requires lambda support OR named function references
6. **Command-line arguments** - Small feature, enables actual puzzle solving
7. **Range operator** - Nice syntactic sugar
8. **Lambda functions** - Enables functional programming patterns
9. **String interpolation** - Quality of life
10. **Tuple type** - More advanced data structure
11. **Regex** - Complex, not always needed
12. **Everything else** - As needed

## Current Neon Strengths for AoC

What Neon **already has** that's great for AoC:

✅ Arrays, Maps, Sets with good methods
✅ String split, substring, replace
✅ Math library (abs, floor, ceil, sqrt, min, max)
✅ For loops (C-style)
✅ If/else conditionals
✅ Functions
✅ Structs (can model complex data)
✅ Number/string/boolean conversions

## Conclusion

**Priority 1 features are blockers** - without them, you cannot even start AoC.

**Priority 2 features are highly recommended** - you'll be frustrated without them.

**Priority 3-4 are quality of life** - you can work around their absence, but it's tedious.

Focus on implementing Priority 1 features first to get Neon to "AoC-ready" status.
