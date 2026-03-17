# Momenta CLI Plan

## Goals

- [ ] Preserve the existing `momenta` library import style for application code.
- [ ] Make a CLI install path available so users can run `momenta` and `mt` from their shell.
- [ ] Define `momenta init` as the canonical project bootstrap command.
- [ ] Define `momenta dev` as the default local development entry point.
- [ ] Support backend selection through `momenta.toml`, with Axum SSR + hydration as the default target experience.
- [ ] Own the build pipeline in-house rather than depending on Trunk for core Momenta workflows.
- [ ] Target a low-footprint WebAssembly pipeline with small artifacts, fast startup, and fast page load as first-order product requirements.

## Packaging Strategy

- [ ] Decide whether the CLI ships from the existing `momenta` package or from a new workspace crate such as `cargo-momenta`.
- [ ] Validate Cargo/package constraints so the published crate name, library target name, and binary target names do not break the current `momenta` dependency/import experience.
- [ ] Ensure the install story is explicit for both local and global usage:
- [ ] Document `cargo install momenta` if the existing package can safely expose binaries.
- [ ] Document `cargo install cargo-momenta` plus binary names `momenta` and `mt` if a split package is required.
- [ ] Confirm how the short alias `mt` is exposed:
- [ ] Prefer a second binary target if Cargo packaging remains straightforward.
- [ ] Fall back to shell completion or manual alias documentation only if shipping both binaries from Cargo becomes awkward.
- [ ] Decide whether the CLI is always installed with the library package or offered as an optional companion package.

## Workspace Restructure

- [ ] Evaluate whether the workspace should add a dedicated CLI crate.
- [ ] If a CLI crate is added, define dependency boundaries so CLI logic depends on stable public APIs rather than internal crate details.
- [ ] If Axum SSR support becomes a first-class backend, decide whether it belongs in a separate runtime crate such as `momenta-ssr`.
- [ ] Keep framework runtime concerns, backend integrations, and CLI concerns separated so future backends are pluggable.

## CLI Surface

- [ ] Define top-level command behavior for `momenta` and `mt`.
- [ ] Define help output, version output, and error formatting.
- [ ] Define the command set:
- [ ] `momenta init`
- [ ] `momenta dev`
- [ ] `momenta build`
- [ ] `momenta format`
- [ ] `momenta check`
- [ ] `momenta start`
- [ ] Decide on the CLI framework and argument parsing approach.
- [ ] Define exit code behavior for configuration errors, missing dependencies, scaffold conflicts, and backend launch failures.

## `momenta init` Scope

- [ ] Define what `momenta init` creates in an existing directory versus a fresh directory.
- [ ] Decide whether `init` should support interactive and non-interactive modes.
- [ ] Decide whether `init` should detect an already initialized project and refuse, merge, or repair.
- [ ] Define the generated project shape:
- [ ] application source layout
- [ ] server entry point
- [ ] client entry point for hydration
- [ ] static assets/public directory
- [ ] `momenta.toml`
- [ ] Cargo workspace or single-crate structure
- [ ] Decide whether `init` also generates backend-specific starter files for Axum by default.
- [ ] Decide whether `init` should install or verify only the minimum required build dependencies for the in-house Momenta toolchain.
- [ ] Avoid scaffolding projects around Trunk as a required part of the default workflow.
- [ ] Define template/versioning strategy so starter templates stay in sync with framework releases.

## `momenta dev` Scope

- [ ] Define the default development behavior for `momenta dev`.
- [ ] Specify that the default target experience is Axum-powered SSR with hydration.
- [ ] Define how `dev` builds server and client artifacts through the Momenta build pipeline.
- [ ] Define whether `dev` runs a unified process manager or delegates to backend-specific commands.
- [ ] Define file watching and reload behavior for Rust source, frontend assets, and config changes.
- [ ] Define port allocation, host binding, and conflict resolution.
- [ ] Define logging output so users can see which backend, ports, and build steps are active.
- [ ] Decide whether `dev` should be implemented directly in the CLI or as an adapter that reads `momenta.toml` and invokes backend/toolchain runners.
- [ ] Define which dev-time build settings are configurable through `momenta.toml`.
- [ ] Ensure dev mode prioritizes incremental rebuild speed and low-latency startup.

## `momenta build` Scope

- [ ] Define what artifacts `momenta build` produces for server and client targets.
- [ ] Define build modes and environment handling for development versus production builds.
- [ ] Define output directories and how built assets are referenced by the server runtime.
- [ ] Define whether `build` always builds both server and client artifacts or supports target-specific flags.
- [ ] Define how `build` reads `momenta.toml` and backend-specific settings.
- [ ] Define which production build settings are configurable through `momenta.toml`.
- [ ] Define optimization goals for artifact size, startup time, and load performance.

## In-House Build Tooling

- [ ] Define the Momenta-owned build pipeline that replaces Trunk in the default workflow.
- [ ] Decide which parts of the toolchain are implemented directly in Rust versus delegated to tightly scoped external tooling.
- [ ] Evaluate WebAssembly build tools in Rust and adjacent ecosystems based on footprint, startup speed, build throughput, and output size.
- [ ] Define the minimum acceptable toolchain surface so users do not need a large frontend build stack for the default path.
- [ ] Decide whether asset bundling, CSS handling, and wasm optimization are built in or exposed as optional pipeline stages.
- [ ] Define how the in-house toolchain integrates with `momenta dev`, `momenta build`, `momenta check`, and `momenta start`.
- [ ] Define a fallback strategy if a selected wasm optimization tool is unavailable on a user's machine.

## `momenta format` Scope

- [ ] Define `momenta format` as the single formatting entry point for Rust code and RSX syntax.
- [ ] Decide whether `format` wraps `rustfmt`, a custom RSX formatter, or a composed pipeline.
- [ ] Define which files and directories are formatted by default.
- [ ] Define whether `format` supports check-only and write modes.
- [ ] Define failure behavior when formatting tools are missing or when RSX formatting encounters invalid syntax.

## `momenta check` Scope

- [ ] Define what `momenta check` validates by default.
- [ ] Decide whether `check` covers config validation, Rust compilation, client compilation, formatting status, and backend readiness.
- [ ] Define whether `check` is purely local validation or may invoke backend-specific build steps.
- [ ] Define output and exit behavior so it is usable in CI and pre-commit workflows.

## `momenta start` Scope

- [ ] Define `momenta start` as the command that runs previously built artifacts.
- [ ] Define how `start` locates the built server executable and client assets.
- [ ] Define whether `start` requires a prior `momenta build` or can trigger a guarded fallback build.
- [ ] Define runtime environment handling for host, port, and production configuration.
- [ ] Define backend-specific startup hooks and failure reporting.

## Configuration Design

- [ ] Define the initial `momenta.toml` schema.
- [ ] Add a backend selector key with Axum as the default.
- [ ] Define server settings such as host, port, and environment.
- [ ] Define client build settings such as entry file, output directory, and hydration toggle.
- [ ] Define build settings such as optimization level, wasm pipeline options, output targets, and asset processing behavior.
- [ ] Define dev settings such as watch paths, reload behavior, startup behavior, and extra commands.
- [ ] Define backend-specific config sections so alternate backends can extend the file without breaking shared keys.
- [ ] Define validation rules and user-facing error messages for malformed config.
- [ ] Decide how config defaults are resolved when keys are omitted.
- [ ] Ensure `momenta.toml` is the single source of truth for backend, build, and dev behavior.

## Backend Abstraction

- [ ] Define what a backend adapter must provide to the CLI.
- [ ] Specify the minimum contract for Axum support:
- [ ] SSR request handling
- [ ] serving client assets
- [ ] hydration bootstrapping expectations
- [ ] development server lifecycle hooks
- [ ] Define how future backends register capabilities and custom config.
- [ ] Decide whether backend adapters are compile-time features, separate crates, or external executables.

## Hydration and SSR Readiness

- [ ] Audit current runtime capabilities to verify what already exists for server-side rendering.
- [ ] Audit current runtime capabilities to verify what is still missing for hydration.
- [ ] Separate what can ship immediately in the CLI plan from what requires framework/runtime work.
- [ ] Define the minimum hydration/runtime milestones required before promising Axum SSR + hydration as a supported default.
- [ ] Avoid documenting or releasing `momenta dev` as fully supported until runtime guarantees are real.

## Templates and Developer Experience

- [ ] Decide whether starter templates live inside the CLI crate, a templates directory, or a separate repository.
- [ ] Define how templates are tested against the current workspace crates.
- [ ] Ensure starter output uses current recommended crate names, imports, and feature flags.
- [ ] Ensure starter output is aligned with the in-house build tool and does not assume Trunk-specific files or commands.
- [ ] Provide clear post-init guidance for first run, backend switching, and production build flow.

## Documentation

- [ ] Update installation documentation to explain library dependency usage separately from CLI installation usage.
- [ ] Add a CLI quickstart covering `momenta init`, `momenta dev`, `momenta build`, `momenta format`, `momenta check`, and `momenta start`.
- [ ] Document how `momenta.toml` selects and configures backends.
- [ ] Document how `momenta.toml` configures build and dev behavior.
- [ ] Document the in-house build pipeline and why it replaces Trunk in the default workflow.
- [ ] Document the difference between framework crates and the executable tooling.
- [ ] Document how the `mt` alias is installed and supported.

## Testing and Verification

- [ ] Add tests for CLI argument parsing and command dispatch.
- [ ] Add tests for config parsing and validation.
- [ ] Add scaffold snapshot tests for `momenta init` output.
- [ ] Add integration tests covering `momenta dev` config resolution and backend command selection.
- [ ] Add integration tests covering in-house build pipeline behavior and `momenta.toml` build/dev options.
- [ ] Add end-to-end verification for the default Axum starter once SSR and hydration support are ready.
- [ ] Verify the install experience on a clean machine for both local and global CLI workflows.

## Release and Adoption

- [ ] Decide versioning and release coordination between framework crates and the CLI package.
- [ ] Define a migration note for existing users who already depend on `momenta` as a library only.
- [ ] Decide whether the first release should ship in phases:
- [ ] Phase 1: packaging and `init`
- [ ] Phase 2: config and backend abstraction
- [ ] Phase 3: default Axum SSR + hydration workflow
- [ ] Define release criteria so the CLI does not over-promise features that the runtime has not implemented yet.
