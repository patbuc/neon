# Neon WebAssembly

Run Neon code in the browser using WebAssembly!

## Prerequisites

You need to install `wasm-pack`:

```bash
cargo install wasm-pack
```

## Building for WebAssembly

Run the build script:

```bash
./build-wasm.sh
```

This will:

1. Compile Neon to WebAssembly
2. Generate JavaScript bindings
3. Output everything to `wasm-pkg/`

The build produces:

- `wasm-pkg/neon_bg.wasm` - The WebAssembly binary
- `wasm-pkg/neon.js` - JavaScript module with bindings
- `wasm-pkg/neon.d.ts` - TypeScript type definitions

## Running the Demo

After building, you need to serve the files over HTTP (browsers don't allow loading wasm from `file://` URLs):

```bash
python3 -m http.server 8000

Then open http://localhost:8000/wasm-demo/index.html in your browser.

## Using in Your Own Web Application

### Basic Usage

```html
<!DOCTYPE html>
<html>
<head>
    <title>Neon in Browser</title>
</head>
<body>
    <script type="module">
        import init, { NeonVM } from './wasm-pkg/neon.js';

        async function run() {
            // Initialize the wasm module
            await init();

            // Create a new VM instance
            const vm = new NeonVM();

            // Run some Neon code
            const result = vm.interpret(`
                var message = "Hello from WebAssembly!";
                print message;
            `);

            if (result.success) {
                console.log("Output:", result.output);
            } else {
                console.error("Error:", result.error);
            }
        }

        run();
    </script>
</body>
</html>
```

### API Reference

#### `NeonVM` Class

**Constructor:**

```javascript
const vm = new NeonVM();
```

Creates a new Neon virtual machine instance. Each instance maintains its own state (globals, etc.).

**Methods:**

##### `interpret(source: string): Result`

Executes Neon source code.

**Parameters:**

- `source` - String containing Neon code

**Returns:** An object with:

- `success: boolean` - Whether execution succeeded
- `output: string | null` - Printed output (if successful)
- `error: string | null` - Error message (if failed)

**Example:**

```javascript
const result = vm.interpret(`
    var x = 10
    var y = 20
    print x + y
`);

if (result.success) {
    console.log(result.output); // "30\n"
} else {
    console.error(result.error);
}
```

#### `interpret_once(source: string): Result`

Convenience function that creates a VM, runs code once, and discards the VM.

**Example:**

```javascript
import init, {interpret_once} from './wasm-pkg/neon.js';

await init();
const result = interpret_once('print "Hello!";');
console.log(result.output); // "Hello!\n"
```

## Key Differences from Native

When running in WebAssembly:

1. **No colored output** - ANSI color codes are disabled
2. **No file I/O** - Can't read/write files from the browser
3. **All output captured** - `print` statements go to the result object, not console
4. **Stateful VM** - Create a `NeonVM` instance to maintain state across multiple executions

## Performance Notes

- First load requires downloading the wasm module (~few hundred KB)
- Subsequent runs are fast (wasm is cached)
- Execution speed is comparable to native (within 1-2x)
- Memory usage is controlled by the browser

## Troubleshooting

### Module not found error

Make sure you've run `./build-wasm.sh` and the `wasm-pkg/` directory exists.

## Examples

See `wasm-demo/index.html` for a complete example with a code editor and multiple example programs.

## Building for Production

For production use:

```bash
wasm-pack build --target web --release --out-dir wasm-pkg
```

This creates an optimized build with smaller file size and better performance.

## TypeScript Support

TypeScript definitions are automatically generated in `wasm-pkg/neon.d.ts`:

```typescript
import init, {NeonVM} from './wasm-pkg/neon.js';

async function main() {
    await init();
    const vm: NeonVM = new NeonVM();
    const result: any = vm.interpret('print "Hello!";');
    // TypeScript knows the types!
}
```
