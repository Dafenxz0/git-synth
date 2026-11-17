use crate::git_log::CommitRecord;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scale {
    Pentatonic,
    Dorian,
    Lydian,
    Minor,
}

impl Scale {
    pub fn intervals(self) -> &'static [u8] {
        match self {
            Scale::Pentatonic => &[0, 3, 5, 7, 10],
            Scale::Dorian => &[0, 2, 3, 5, 7, 9, 10],
            Scale::Lydian => &[0, 2, 4, 6, 7, 9, 11],
            Scale::Minor => &[0, 2, 3, 5, 7, 8, 10],
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Scale::Pentatonic => "Pentatonic",
            Scale::Dorian => "Dorian",
            Scale::Lydian => "Lydian",
            Scale::Minor => "Minor",
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Scale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "pentatonic" | "penta" => Ok(Scale::Pentatonic),
            "dorian" => Ok(Scale::Dorian),
            "lydian" => Ok(Scale::Lydian),
            "minor" | "aeolian" => Ok(Scale::Minor),
            other => Err(format!(
                "unknown scale '{other}' (expected: pentatonic, dorian, lydian, minor)"
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommitEvent {
    pub index: usize,
    pub start_sec: f32,
    pub duration_sec: f32,
    pub bass_midi: u8,
    pub arp_notes: Vec<u8>,
    pub pad_chord: Vec<u8>,
    pub pan: f32,
    pub velocity: f32,
    pub commit: CommitRecord,
}

pub fn quantize_midi(raw_midi: i32, root_midi: u8, scale: Scale) -> u8 {
    let clamped = raw_midi.clamp(24, 108);
    let root_pc = i32::from(root_midi % 12);
    let intervals = scale.intervals();

    let mut best_midi = clamped;
    let mut best_dist = i32::MAX;

    for octave in 1..=9 {
        for &interval in intervals {
            let candidate = octave * 12 + root_pc + i32::from(interval);
            if !(24..=108).contains(&candidate) {
                continue;
            }
            let dist = (candidate - clamped).abs();
            if dist < best_dist {
                best_dist = dist;
                best_midi = candidate;
            }
        }
    }

    best_midi as u8
}

pub fn degree_to_midi(root_midi: u8, scale: Scale, degree: usize, octave_offset: i32) -> u8 {
    let intervals = scale.intervals();
    let len = intervals.len();
    let octaves = (degree / len) as i32 + octave_offset;
    let interval = i32::from(intervals[degree % len]);
    let raw = i32::from(root_midi) + octaves * 12 + interval;
    raw.clamp(24, 108) as u8
}

pub fn midi_to_hz(midi: u8) -> f32 {
    440.0 * 2.0_f32.powf((f32::from(midi) - 69.0) / 12.0)
}

pub fn midi_to_name(midi: u8) -> String {
    const NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let pc = usize::from(midi % 12);
    let octave = i32::from(midi / 12) - 1;
    format!("{}{octave}", NAMES[pc])
}

pub fn fnv1a_hash(text: &str) -> u32 {
    let mut hash: u32 = 0x811c_9dc5;
    for byte in text.as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

fn extension_degree_offset(ext: &str) -> usize {
    match ext {
        "rs" => 0,
        "py" => 1,
        "js" | "ts" | "jsx" | "tsx" => 2,
        "c" | "cpp" | "h" | "hpp" | "go" => 3,
        "md" | "txt" | "rst" => 4,
        "css" | "scss" | "html" | "svg" => 5,
        "toml" | "json" | "yaml" | "yml" => 6,
        _ => (fnv1a_hash(ext) as usize) % 7,
    }
}

pub fn commits_to_score(
    commits: &[CommitRecord],
    scale: Scale,
    bpm: f32,
    root_midi: u8,
) -> Vec<CommitEvent> {
    let safe_bpm = bpm.clamp(40.0, 240.0);
    let beat_sec = 60.0 / safe_bpm;
    let mut cursor_sec = 0.0_f32;
    let mut events = Vec::with_capacity(commits.len());

    for (idx, commit) in commits.iter().enumerate() {
        let churn = commit.total_churn();
        let beats = if churn >= 200 {
            2.0
        } else if churn >= 60 {
            1.5
        } else {
            1.0
        };
        let duration_sec = beats * beat_sec;

        let hash_seed = fnv1a_hash(&commit.hash);
        let author_seed = fnv1a_hash(&commit.author);
        let ext_offset = extension_degree_offset(&commit.primary_ext);
        let scale_len = scale.intervals().len();

        let base_degree = ((hash_seed as usize) + ext_offset + idx) % scale_len;
        let bass_midi = degree_to_midi(root_midi, scale, base_degree, -1);

        let arp_count = if commit.files_changed >= 3 || churn >= 80 {
            4
        } else {
            3
        };
        let mut arp_notes = Vec::with_capacity(arp_count);
        for step in 0..arp_count {
            let step_shift = ((hash_seed >> (step * 4)) as usize + step * 2) % (scale_len * 2);
            let note = degree_to_midi(root_midi, scale, base_degree + step_shift, 1);
            arp_notes.push(note);
        }

        let mut pad_chord = vec![
            degree_to_midi(root_midi, scale, base_degree, 0),
            degree_to_midi(root_midi, scale, base_degree + 2, 0),
            degree_to_midi(root_midi, scale, base_degree + 4, 0),
        ];
        if commit.extensions.len() > 1 {
            pad_chord.push(degree_to_midi(root_midi, scale, base_degree + 6, 0));
        }

        // Author determines home stereo position, nudged by insertion/deletion ratio.
        let author_pan = ((author_seed % 140) as f32 / 70.0) - 1.0; // [-1.0, ~1.0]
        let diff_bias = if churn > 0 {
            (commit.insertions as f32 - commit.deletions as f32) / churn as f32
        } else {
            0.0
        };
        let pan = (author_pan * 0.65 + diff_bias * 0.25).clamp(-0.85, 0.85);

        let intensity = (1.0 + churn as f32).log2() / 10.0;
        let velocity = (0.38 + intensity * 0.57).clamp(0.38, 0.95);

        events.push(CommitEvent {
            index: idx,
            start_sec: cursor_sec,
            duration_sec,
            bass_midi,
            arp_notes,
            pad_chord,
            pan,
            velocity,
            commit: commit.clone(),
        });

        cursor_sec += duration_sec;
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_log::sample_commits;

    #[test]
    fn scale_quantization_snaps_to_scale_pitch_classes() {
        let root = 48; // C3
        for scale in [Scale::Pentatonic, Scale::Dorian, Scale::Lydian, Scale::Minor] {
            let root_pc = root % 12;
            let allowed_pcs: Vec<u8> = scale
                .intervals()
                .iter()
                .map(|i| (root_pc + i) % 12)
                .collect();

            for raw in 30..=96 {
                let q = quantize_midi(raw, root, scale);
                assert!((24..=108).contains(&q));
                assert!(
                    allowed_pcs.contains(&(q % 12)),
                    "note {q} not in {scale:?} for root {root}"
                );
            }
        }
    }

    #[test]
    fn midi_to_hz_matches_a4_concert_pitch() {
        let a4 = midi_to_hz(69);
        assert!((a4 - 440.0).abs() < 1e-4);
        let a3 = midi_to_hz(57);
        assert!((a3 - 220.0).abs() < 1e-4);
    }

    #[test]
    fn commits_map_to_bounded_events() {
        let commits = sample_commits();
        let score = commits_to_score(&commits, Scale::Dorian, 120.0, 48);
        assert_eq!(score.len(), commits.len());
        for event in &score {
            assert!(event.duration_sec > 0.0);
            assert!((-0.85..=0.85).contains(&event.pan));
            assert!((0.38..=0.95).contains(&event.velocity));
            assert!(!event.arp_notes.is_empty());
            assert!(event.pad_chord.len() >= 3);
        }
    }
}
