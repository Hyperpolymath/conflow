# conflow - Configuration Flow Orchestrator

Intelligently orchestrate CUE, Nickel, and configuration validation workflows.

## Why conflow?

**Problem:** You have configuration files and you're not sure whether to use CUE, Nickel, or both.

**Solution:** conflow analyzes your configs, recommends the right tool, and orchestrates the entire pipeline.

```bash
# Instead of:
nickel export config.ncl > temp.json
cue vet schema.cue temp.json
cue export schema.cue --out yaml > deploy.yaml
rm temp.json

# Just:
conflow run
```

## Features

- **Intelligent analysis** - Recommends CUE vs Nickel based on complexity
- **Pipeline orchestration** - Chain tools with dependency management
- **Smart caching** - Only re-run what changed
- **Educational** - Learn why certain tools fit certain problems
- **Type-safe** - Catch errors before deployment

## Quick Start

```bash
# Install
cargo install conflow

# Initialize
conflow init my-project

# Analyze existing configs
conflow analyze config.yaml

# Run pipeline
conflow run
```

## Example Pipeline

```yaml
# .conflow.yaml
version: "1"
name: "k8s-deployment"

stages:
  - name: "generate"
    tool:
      type: nickel
      command: export
      file: config.ncl
    output: generated/config.json

  - name: "validate"
    tool:
      type: cue
      command: vet
      schemas: [schemas/k8s.cue]
    input:
      from_stage: generate
    depends_on: [generate]

  - name: "export"
    tool:
      type: cue
      command: export
      out_format: yaml
    input:
      from_stage: generate
    depends_on: [validate]
    output: deploy/k8s.yaml
```

```bash
$ conflow run
✓ generate (0.08s)
✓ validate (0.05s)
✓ export (0.03s)

Pipeline completed in 0.16s
```

## When to Use What?

### Use CUE when:
- ✅ Validating configuration
- ✅ Expressing constraints
- ✅ Merging configurations
- ✅ Simple transformations

### Use Nickel when:
- ✅ Generating configurations
- ✅ Complex logic needed
- ✅ Functions and abstraction
- ✅ DRY configuration

### Use Both when:
- ✅ Nickel generates → CUE validates
- ✅ Complex generation + strict validation

## Commands

```bash
conflow init [--template <name>]  # Initialize project
conflow analyze <files>           # Analyze config files
conflow run [--stage <name>]      # Execute pipeline
conflow watch                     # Watch mode
conflow validate                  # Validate pipeline
conflow graph [--format <fmt>]    # Show pipeline graph
conflow cache stats               # Cache statistics
conflow cache clear               # Clear cache
```

## Templates

```bash
conflow init --template cue-validation     # Simple CUE validation
conflow init --template nickel-generation  # Nickel config generation
conflow init --template full-pipeline      # Generate → validate → export
conflow init --template kubernetes         # Kubernetes manifests
```

## Development

```bash
# Using Nix
nix develop

# Using just
just build      # Build
just test       # Run tests
just check      # Run all checks
just install    # Install locally
```

## License

MIT OR Apache-2.0
