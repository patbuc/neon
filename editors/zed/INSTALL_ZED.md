# Installing Zed Editor

Zed is not currently installed on this system. Here's how to install it:

## Linux Installation

### Option 1: Download from Official Website

Visit [zed.dev](https://zed.dev) and download the latest version for Linux.

### Option 2: Install via Script

```bash
curl https://zed.dev/install.sh | sh
```

### Option 3: Install from GitHub Releases

```bash
# Download latest release
wget https://github.com/zed-industries/zed/releases/latest/download/zed-linux-x86_64.tar.gz

# Extract
tar -xzf zed-linux-x86_64.tar.gz

# Move to /usr/local/bin or add to PATH
sudo mv zed /usr/local/bin/
```

### Option 4: Build from Source

```bash
git clone https://github.com/zed-industries/zed.git
cd zed
cargo build --release
sudo cp target/release/zed /usr/local/bin/
```

## After Installation

Once Zed is installed, install the Neon extension:

```bash
cd /home/patbuc/code/neon/editors/zed
./install-dev.sh
```

Then open Zed and test with:
```bash
zed /home/patbuc/code/neon/examples/day1_solution.n
```

## Verify Installation

Check if Zed is available:
```bash
which zed
zed --version
```

## System Requirements

- Linux (x86_64)
- glibc 2.31 or later
- GPU drivers for optimal performance (OpenGL/Vulkan)

## Alternative: Test in Browser

If you prefer not to install Zed locally, you can:
1. Push the extension to the Zed extension marketplace
2. Test using Zed's web version (if available)
3. Test using VS Code with our Tree-sitter grammar (see editors/vscode/)
