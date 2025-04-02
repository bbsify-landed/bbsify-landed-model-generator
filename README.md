# 3D Model Generator

A powerful Rust library for programmatically generating 3D models through composable transformations and plugins.

## Features

- **Transformation Pipeline**: Apply sequences of geometric transformations to create complex 3D models
- **Plugin System**: Extend functionality through custom plugins
- **High Performance**: Written in Rust for memory safety and performance
- **File Format Support**: Export to common 3D file formats (.obj, .stl, .gltf)
- **Procedural Generation**: Create models algorithmically through code

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
model-generator = "0.1.0"
```

## Usage

```rust
use model_generator::{Model, Transform};
use model_generator::primitives::Cube;
use model_generator::transforms::{Scale, Rotate};

fn main() {
    // Create a base primitive
    let mut model = Cube::new().build();
    
    // Apply transformations
    model.apply(Scale::new(2.0, 1.0, 1.0))
         .apply(Rotate::around_y(45.0));
    
    // Export the model
    model.export_obj("elongated_rotated_cube.obj").unwrap();
}
```

## Plugin System

Create custom transformations by implementing the `Transform` trait:

```rust
use model_generator::{Model, Transform, Result};

struct CustomTransform {
    // Your transform parameters
}

impl Transform for CustomTransform {
    fn apply(&self, model: &mut Model) -> Result<()> {
        // Transform the model
        // ...
        Ok(())
    }
}
```

## Architecture

The library is built around these core components:

- **Primitives**: Basic shapes (cubes, spheres, cylinders) as starting points
- **Model**: The core data structure representing 3D meshes
- **Transforms**: Operations that modify models (scale, rotate, extrude)
- **Plugin System**: Interface for creating custom operations

## Building from Source

```bash
git clone https://github.com/bbsify-landed/bbsify-landed-model-generator.git
cd bbsify-landed-model-generator
cargo build --release
```

## Cargo Commands

Here are some useful Cargo commands for development:

```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run the project
cargo run

# Run a specific example
cargo run --example <example_name>

# Run tests
cargo test

# Generate documentation
cargo doc --open

# Check code for errors without building
cargo check

# Format code
cargo fmt

# Check for potential improvements
cargo clippy
```

## Examples

Find more examples in the `/examples` directory:

- Basic primitives
- Complex transformations
- Procedural generation techniques
- Plugin development

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request 

## Continuous Integration and Deployment

This project uses GitHub Actions for continuous integration and deployment:

### Workflows

1. **Rust CI** (`rust.yml`)
   - Runs on pull requests and pushes to main
   - Runs tests on multiple platforms (Linux, Windows, macOS)
   - Performs code linting with Clippy
   - Checks code formatting
   - Builds the project

2. **Benchmarks** (`benchmark.yml`)
   - Runs on pushes to main
   - Executes performance benchmarks
   - Stores benchmark results as artifacts

3. **Documentation** (`docs.yml`)
   - Runs on pushes to main
   - Generates Rust documentation
   - Publishes documentation to GitHub Pages

### Status Badges

[![Rust CI](https://github.com/bbsify-landed/bbsify-landed-model-generator/actions/workflows/rust.yml/badge.svg)](https://github.com/bbsify-landed/bbsify-landed-model-generator/actions/workflows/rust.yml)
[![Documentation](https://github.com/bbsify-landed/bbsify-landed-model-generator/actions/workflows/docs.yml/badge.svg)](https://github.com/bbsify-landed/bbsify-landed-model-generator/actions/workflows/docs.yml)
[![Benchmarks](https://github.com/bbsify-landed/bbsify-landed-model-generator/actions/workflows/benchmark.yml/badge.svg)](https://github.com/bbsify-landed/bbsify-landed-model-generator/actions/workflows/benchmark.yml)

## License

This project is licensed under the MIT License - see the LICENSE file for details. 