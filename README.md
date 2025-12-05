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
