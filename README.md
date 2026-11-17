# git-synth

[![CI](https://github.com/Dafenxz0/git-synth/actions/workflows/ci.yml/badge.svg)](https://github.com/Dafenxz0/git-synth/actions/workflows/ci.yml)

Turn any Git repository's commit history into generative polyphonic music and a live piano-roll visualizer.

**Live Web Audio Player:** [https://dafenxz0.github.io/git-synth/](https://dafenxz0.github.io/git-synth/)

## Two Modes

1. **Pure-Rust CLI (`src/`):** Zero external crate dependencies. Reads `git log --numstat` (or a bundled sample log), synthesizes polyphonic bass pulse, arpeggio, and detuned pad voices with ADSR envelopes and stereo panning, writes a standard 16-bit 44.1 kHz stereo `.wav` file, and prints a TrueColor ANSI Goertzel spectrogram timeline in the terminal.
2. **Interactive Web Audio Player (`web/`):** Load any public GitHub `owner/repo` (or play built-in offline presets for `karpathy/micrograd`, `torvalds/linux`, `facebook/react`, and `Dafenxz0/git-synth`) with live Web Audio synthesis, tempo/scale/voice controls, and a Canvas commit-particle piano-roll + FFT spectrum visualizer.

## How Commit Metrics Map to Music

| Git Metric | Musical Parameter |
| --- | --- |
| Commit SHA + primary file extension (`.rs`, `.py`, `.js`, `.c`, `.md`, ...) | Scale degree & melodic arpeggio motif quantized to `Pentatonic`, `Dorian`, `Lydian`, or `Minor` |
| Insertions + deletions (`--numstat` churn) | Note velocity, rhythmic duration (`1.0` / `1.5` / `2.0` beats), arpeggio density, particle burst size |
| Multi-extension commits | Adds a 7th interval to the detuned pad chord |
| Author identity + insertion/deletion balance | Constant-power stereo pan (`L` to `R`) |
| Chronological commit order | Sequential song timeline (`--reverse`) |

## CLI Usage

```bash
# Synthesize music from the current repository
cargo run --release -- --repo . --output git-synth.wav

# Choose scale, tempo, and commit limit
cargo run --release -- --repo /path/to/repo --scale dorian --tempo 128 --max-commits 64 --output repo.wav

# Run using the built-in offline sample git log
cargo run --release -- --sample --scale lydian --output sample.wav

# Run unit tests
cargo test --verbose
```

### CLI Options

- `-r, --repo <PATH>` — Target git repository path (default: `.`)
- `-o, --output <FILE.wav>` — Output 16-bit stereo PCM WAV file (default: `git-synth.wav`)
- `-s, --scale <SCALE>` — Musical scale: `pentatonic`, `dorian`, `lydian`, `minor` (default: `pentatonic`)
- `-t, --tempo <BPM>` — Tempo in BPM, `40..240` (default: `120`)
- `-n, --max-commits <N>` — Max commits to read (default: `48`)
- `--root <MIDI>` — Root MIDI pitch, `24..72` (default: `48` / `C3`)
- `--sample` — Use the bundled sample commit log
- `--no-color` — Disable ANSI TrueColor output in the terminal spectrogram

## Project Structure

- `src/git_log.rs` — `git log --numstat` execution, parser, extension extractor, and bundled sample log
- `src/music.rs` — Scale definitions, quantization, MIDI/Hz math, deterministic commit-to-score mapping
- `src/synth.rs` — ADSR envelope, bass/arpeggio/pad voices, constant-power stereo panner, soft limiter, 16-bit PCM WAV encoder
- `src/viz.rs` — Terminal Goertzel band-energy ASCII spectrogram & score timeline
- `web/` — Zero-build Web Audio synthesizer & HTML5 Canvas piano-roll visualizer for GitHub Pages
