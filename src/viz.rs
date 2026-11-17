use crate::music::{midi_to_name, CommitEvent, Scale};
use crate::synth::SAMPLE_RATE;
use std::f32::consts::TAU;

const BAND_FREQS_HZ: [f32; 10] = [
    65.4, 98.0, 130.8, 196.0, 261.6, 392.0, 523.3, 784.0, 1046.5, 1568.0,
];
const BAR_CHARS: [char; 8] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '█'];

pub fn format_timeline(
    score: &[CommitEvent],
    samples: &[(f32, f32)],
    scale: Scale,
    bpm: f32,
    use_color: bool,
) -> String {
    let mut out = String::new();
    let total_dur = samples.len() as f32 / SAMPLE_RATE as f32;

    out.push_str(&paint(
        use_color,
        "1;36",
        &format!(
            "git-synth :: {} commits | scale: {} | {:.0} BPM | {:.2}s stereo 44.1kHz PCM\n",
            score.len(),
            scale,
            bpm,
            total_dur
        ),
    ));
    out.push_str(&paint(
        use_color,
        "2;37",
        "IDX  HASH     EXT    AUTHOR          NOTES            PAN         SPECTRUM     CHURN\n",
    ));

    for event in score {
        let start_idx = ((event.start_sec * SAMPLE_RATE as f32) as usize).min(samples.len());
        let len_samples = ((event.duration_sec * SAMPLE_RATE as f32) as usize).max(1);
        let end_idx = start_idx.saturating_add(len_samples).min(samples.len());
        let slice = &samples[start_idx..end_idx];

        let spectrum = render_spectrum_bars(slice, use_color);
        let pan_bar = render_pan_meter(event.pan);
        let bass_name = midi_to_name(event.bass_midi);
        let lead_name = event
            .arp_notes
            .first()
            .map(|&m| midi_to_name(m))
            .unwrap_or_else(|| "-".to_string());
        let note_col = format!("{bass_name}/{lead_name} ({}p)", event.pad_chord.len());

        let author = truncate_str(&event.author, 14);
        let ext = truncate_str(&format!(".{}", event.commit.primary_ext), 6);
        let churn = format!("+{}/-{}", event.commit.insertions, event.commit.deletions);

        let line = format!(
            "{:>3}  {}  {:<6} {:<14}  {:<15}  {}  {}  {}\n",
            event.index + 1,
            paint(use_color, "33", event.commit.short_hash()),
            paint(use_color, ext_ansi_color(&event.commit.primary_ext), &ext),
            author,
            paint(use_color, "36", &note_col),
            pan_bar,
            spectrum,
            paint(use_color, "32", &churn),
        );
        out.push_str(&line);
    }

    out
}

fn render_spectrum_bars(slice: &[(f32, f32)], use_color: bool) -> String {
    if slice.is_empty() {
        return " ".repeat(BAND_FREQS_HZ.len());
    }

    let step = (slice.len() / 512).max(1);
    let sample_count = (slice.len() / step).max(1) as f32;
    let eff_rate = SAMPLE_RATE as f32 / step as f32;

    let mut energies = [0.0_f32; BAND_FREQS_HZ.len()];
    let mut peak = 1e-4_f32;

    for (band_idx, &freq) in BAND_FREQS_HZ.iter().enumerate() {
        let omega = TAU * freq / eff_rate;
        let coeff = 2.0 * omega.cos();
        let mut s_prev = 0.0_f32;
        let mut s_prev2 = 0.0_f32;
        for &(l, r) in slice.iter().step_by(step) {
            let mono = 0.5 * (l + r);
            let s = mono + coeff * s_prev - s_prev2;
            s_prev2 = s_prev;
            s_prev = s;
        }
        let power = (s_prev2 * s_prev2 + s_prev * s_prev - coeff * s_prev * s_prev2).max(0.0);
        let mag = power.sqrt() / sample_count;
        energies[band_idx] = mag;
        if mag > peak {
            peak = mag;
        }
    }

    let mut bars = String::new();
    for (idx, &mag) in energies.iter().enumerate() {
        let norm = (mag / peak).powf(0.55).clamp(0.0, 1.0);
        let level = ((norm * 7.0).round() as usize).min(7);
        let ch = BAR_CHARS[level];
        if use_color {
            let r = (40 + idx * 22).min(255);
            let g = (220usize.saturating_sub(idx * 12)).min(255);
            let b = (240usize.saturating_sub(idx * 18)).min(255);
            bars.push_str(&format!("\x1b[38;2;{r};{g};{b}m{ch}\x1b[0m"));
        } else {
            bars.push(ch);
        }
    }
    bars
}

fn render_pan_meter(pan: f32) -> String {
    let mut slots = ['-'; 9];
    let pos = (((pan.clamp(-1.0, 1.0) + 1.0) * 0.5) * 8.0).round() as usize;
    slots[pos.min(8)] = '*';
    let inner: String = slots.into_iter().collect();
    format!("[{inner}]")
}

fn truncate_str(s: &str, max_chars: usize) -> String {
    let count = s.chars().count();
    if count <= max_chars {
        return s.to_string();
    }
    if max_chars <= 1 {
        return s.chars().take(max_chars).collect();
    }
    let prefix: String = s.chars().take(max_chars - 1).collect();
    format!("{prefix}~")
}

fn ext_ansi_color(ext: &str) -> &'static str {
    match ext {
        "rs" => "38;5;208",
        "py" => "38;5;75",
        "js" | "ts" | "jsx" | "tsx" => "38;5;220",
        "md" | "txt" => "38;5;250",
        "css" | "html" => "38;5;171",
        _ => "38;5;114",
    }
}

fn paint(use_color: bool, code: &str, text: &str) -> String {
    if use_color {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_log::sample_commits;
    use crate::music::commits_to_score;
    use crate::synth::render_score;

    #[test]
    fn timeline_renders_without_ansi_when_disabled() {
        let commits = sample_commits();
        let score = commits_to_score(&commits[..3], Scale::Lydian, 120.0, 48);
        let audio = render_score(&score);
        let text = format_timeline(&score, &audio, Scale::Lydian, 120.0, false);

        assert!(text.contains("git-synth :: 3 commits"));
        assert!(text.contains("Lydian"));
        assert!(!text.contains("\x1b["));
    }
}
