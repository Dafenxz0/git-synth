use git_synth::git_log::{read_repo_commits, sample_commits};
use git_synth::music::{commits_to_score, Scale};
use git_synth::synth::{render_score, write_wav_file, SAMPLE_RATE};
use git_synth::viz::format_timeline;
use std::env;
use std::path::PathBuf;
use std::process;

struct CliArgs {
    repo: PathBuf,
    output: PathBuf,
    scale: Scale,
    tempo: f32,
    max_commits: usize,
    root_midi: u8,
    use_sample: bool,
    use_color: bool,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            repo: PathBuf::from("."),
            output: PathBuf::from("git-synth.wav"),
            scale: Scale::Pentatonic,
            tempo: 120.0,
            max_commits: 48,
            root_midi: 48, // C3
            use_sample: false,
            use_color: true,
        }
    }
}

fn parse_args<I>(args: I) -> Result<Option<CliArgs>, String>
where
    I: IntoIterator<Item = String>,
{
    let mut cfg = CliArgs::default();
    let mut iter = args.into_iter().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(None),
            "--sample" => cfg.use_sample = true,
            "--no-color" => cfg.use_color = false,
            "--repo" | "-r" => {
                let val = iter
                    .next()
                    .ok_or_else(|| "missing value for --repo".to_string())?;
                cfg.repo = PathBuf::from(val);
            }
            "--output" | "-o" => {
                let val = iter
                    .next()
                    .ok_or_else(|| "missing value for --output".to_string())?;
                cfg.output = PathBuf::from(val);
            }
            "--scale" | "-s" => {
                let val = iter
                    .next()
                    .ok_or_else(|| "missing value for --scale".to_string())?;
                cfg.scale = val.parse()?;
            }
            "--tempo" | "-t" => {
                let val = iter
                    .next()
                    .ok_or_else(|| "missing value for --tempo".to_string())?;
                let bpm: f32 = val
                    .parse()
                    .map_err(|_| format!("invalid --tempo '{val}'"))?;
                if !(40.0..=240.0).contains(&bpm) {
                    return Err("tempo must be between 40 and 240 BPM".to_string());
                }
                cfg.tempo = bpm;
            }
            "--max-commits" | "-n" => {
                let val = iter
                    .next()
                    .ok_or_else(|| "missing value for --max-commits".to_string())?;
                let count: usize = val
                    .parse()
                    .map_err(|_| format!("invalid --max-commits '{val}'"))?;
                if count == 0 {
                    return Err("--max-commits must be >= 1".to_string());
                }
                cfg.max_commits = count;
            }
            "--root" => {
                let val = iter
                    .next()
                    .ok_or_else(|| "missing value for --root".to_string())?;
                let midi: u8 = val
                    .parse()
                    .map_err(|_| format!("invalid --root MIDI note '{val}'"))?;
                if !(24..=72).contains(&midi) {
                    return Err("--root MIDI note must be between 24 and 72".to_string());
                }
                cfg.root_midi = midi;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(Some(cfg))
}

fn print_help() {
    println!(
        "\
git-synth — Turn any Git repository's commit history into polyphonic music

USAGE:
    git-synth [OPTIONS]

OPTIONS:
    -r, --repo <PATH>          Path to git repository (default: .)
    -o, --output <FILE.wav>    Output 16-bit stereo WAV path (default: git-synth.wav)
    -s, --scale <SCALE>        pentatonic | dorian | lydian | minor (default: pentatonic)
    -t, --tempo <BPM>          Tempo in BPM, 40..240 (default: 120)
    -n, --max-commits <N>      Maximum commits to synthesize (default: 48)
        --root <MIDI>          Root MIDI note, 24..72 (default: 48 / C3)
        --sample               Use bundled sample git log instead of reading git
        --no-color             Disable ANSI color in ASCII spectrogram
    -h, --help                 Print help"
    );
}

fn main() {
    let cfg = match parse_args(env::args()) {
        Ok(Some(cfg)) => cfg,
        Ok(None) => {
            print_help();
            return;
        }
        Err(err) => {
            eprintln!("error: {err}");
            eprintln!("run with --help for usage");
            process::exit(2);
        }
    };

    let commits = if cfg.use_sample {
        sample_commits()
    } else {
        match read_repo_commits(&cfg.repo, cfg.max_commits) {
            Ok(list) => list,
            Err(err) => {
                eprintln!("error reading repository '{}': {err}", cfg.repo.display());
                process::exit(1);
            }
        }
    };

    let score = commits_to_score(&commits, cfg.scale, cfg.tempo, cfg.root_midi);
    let samples = render_score(&score);

    if let Err(err) = write_wav_file(&cfg.output, &samples, SAMPLE_RATE) {
        eprintln!("error writing WAV '{}': {err}", cfg.output.display());
        process::exit(1);
    }

    let timeline = format_timeline(&score, &samples, cfg.scale, cfg.tempo, cfg.use_color);
    print!("{timeline}");
    println!("Wrote {}", cfg.output.display());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cli_flags_cleanly() {
        let args = vec![
            "git-synth".into(),
            "--repo".into(),
            "/tmp/repo".into(),
            "--output".into(),
            "out.wav".into(),
            "--scale".into(),
            "lydian".into(),
            "--tempo".into(),
            "132".into(),
            "--max-commits".into(),
            "20".into(),
            "--no-color".into(),
        ];

        let cfg = parse_args(args).unwrap().unwrap();
        assert_eq!(cfg.repo, PathBuf::from("/tmp/repo"));
        assert_eq!(cfg.output, PathBuf::from("out.wav"));
        assert_eq!(cfg.scale, Scale::Lydian);
        assert!((cfg.tempo - 132.0).abs() < 1e-4);
        assert_eq!(cfg.max_commits, 20);
        assert!(!cfg.use_color);
    }
}
