# Quiet Texture (Memoriae)

A lightweight VST3/CLAP tone-shaping plugin built with [nih-plug](https://github.com/robbert-vdh/nih-plug). It blends a gentle tilt filter with adjustable gain and dry/wet control, keeping the DSP simple to avoid linker errors on Windows builds.

## Features
- Gain stage with logarithmic smoothing (-24 dB to +24 dB)
- Stereo-safe tilt filter to add shimmer or darken the sound
- Dry/wet mix control
- Zero external native dependencies beyond `nih_plug`

## Building
Requires Rust 1.75+ and Cargo.

### Desktop builds
- **Windows (MSVC recommended):**
  ```powershell
  cargo build --release --target x86_64-pc-windows-msvc
  ```
- **macOS/Linux:**
  ```bash
  cargo build --release
  ```

The resulting plugin bundle is placed under `target/<profile>/`. You can copy the `.vst3` bundle into your VST3 directory or load the CLAP binary with a compatible host.

#### If `nih_plug` cannot be found on Windows
- Make sure Git is installed (the dependency is pulled directly from the upstream repository).
- Update to the latest stable Rust (1.75+).
- Clear any legacy vendored `~/.cargo/config` that overrides the crates.io source.
- If resolution still fails, ensure `Cargo.toml` keeps `nih_plug` as a git dependency, then run `cargo update` before rebuilding:
  ```powershell
  cargo update
  cargo build --release --target x86_64-pc-windows-msvc
  ```
  The repository-pinned dependency in `Cargo.toml` avoids the "no matching package named `nih_plug` found" registry error.

### Notes on linker stability
- The DSP uses only Rust standard library math; no platform-specific intrinsics are required.
- The crate type includes `cdylib` to ensure the correct exports for plugin formats.
- `nih_plug` handles the necessary VST3/CLAP entry points, so no manual linker flags are needed.

## Parameters
- **Output:** Linear output gain in decibels.
- **Tilt:** Tone bias; negative values soften highs, positive values brighten.
- **Mix:** Crossfades between dry input and processed output.

## License
MIT
