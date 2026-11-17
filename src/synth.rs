use crate::music::{midi_to_hz, CommitEvent};
use std::f32::consts::{FRAC_PI_4, TAU};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub const SAMPLE_RATE: u32 = 44_100;

#[derive(Debug, Clone, Copy)]
pub struct AdsrEnvelope {
    pub attack_sec: f32,
    pub decay_sec: f32,
    pub sustain_level: f32,
    pub release_sec: f32,
}

impl AdsrEnvelope {
    pub fn sustain_gain(self, t: f32) -> f32 {
        let s = self.sustain_level.clamp(0.0, 1.0);
        if t < 0.0 {
            return 0.0;
        }
        if self.attack_sec > 0.0 && t < self.attack_sec {
            return (t / self.attack_sec).clamp(0.0, 1.0);
        }
        let after_attack = t - self.attack_sec.max(0.0);
        if self.decay_sec > 0.0 && after_attack < self.decay_sec {
            let frac = after_attack / self.decay_sec;
            return (1.0 - frac * (1.0 - s)).clamp(0.0, 1.0);
        }
        s
    }

    pub fn amplitude(self, t: f32, gate_sec: f32) -> f32 {
        if t < 0.0 || gate_sec <= 0.0 {
            return 0.0;
        }
        if t <= gate_sec {
            return self.sustain_gain(t);
        }
        if self.release_sec <= 0.0 {
            return 0.0;
        }
        let release_t = t - gate_sec;
        if release_t >= self.release_sec {
            return 0.0;
        }
        let gate_level = self.sustain_gain(gate_sec);
        (gate_level * (1.0 - release_t / self.release_sec)).clamp(0.0, 1.0)
    }

    pub fn total_duration(self, gate_sec: f32) -> f32 {
        gate_sec.max(0.0) + self.release_sec.max(0.0)
    }
}

pub fn pan_gains(pan: f32) -> (f32, f32) {
    let angle = (pan.clamp(-1.0, 1.0) + 1.0) * FRAC_PI_4;
    (angle.cos(), angle.sin())
}

pub fn render_score(score: &[CommitEvent]) -> Vec<(f32, f32)> {
    if score.is_empty() {
        return Vec::new();
    }

    let tail_sec = 0.85_f32;
    let total_sec = score
        .iter()
        .map(|e| e.start_sec + e.duration_sec)
        .fold(0.0_f32, f32::max)
        + tail_sec;

    let total_samples = ((total_sec * SAMPLE_RATE as f32).ceil() as usize).max(1);
    let mut mix = vec![(0.0_f32, 0.0_f32); total_samples];

    let bass_env = AdsrEnvelope {
        attack_sec: 0.012,
        decay_sec: 0.16,
        sustain_level: 0.55,
        release_sec: 0.18,
    };
    let arp_env = AdsrEnvelope {
        attack_sec: 0.006,
        decay_sec: 0.11,
        sustain_level: 0.28,
        release_sec: 0.16,
    };
    let pad_env = AdsrEnvelope {
        attack_sec: 0.16,
        decay_sec: 0.30,
        sustain_level: 0.62,
        release_sec: 0.50,
    };

    for event in score {
        // 1. Bass pulse voice
        let bass_freq = midi_to_hz(event.bass_midi);
        let bass_gate = (event.duration_sec * 0.78).max(0.08);
        let (bl, br) = pan_gains(event.pan * 0.25);
        add_voice(
            &mut mix,
            event.start_sec,
            bass_gate,
            bass_env,
            event.velocity * 0.42,
            bl,
            br,
            |t| {
                let phase = TAU * bass_freq * t;
                0.75 * phase.sin() + 0.25 * (2.0 * phase).sin() * (-4.5 * t).exp()
            },
        );

        // 2. Polyphonic arpeggio voice
        let step_count = event.arp_notes.len().max(1);
        let step_sec = event.duration_sec / step_count as f32;
        let (al, ar) = pan_gains(event.pan);
        for (idx, &midi) in event.arp_notes.iter().enumerate() {
            let freq = midi_to_hz(midi);
            let note_start = event.start_sec + idx as f32 * step_sec;
            let note_gate = (step_sec * 0.75).max(0.05);
            add_voice(
                &mut mix,
                note_start,
                note_gate,
                arp_env,
                event.velocity * 0.34,
                al,
                ar,
                |t| {
                    let phase = TAU * freq * t;
                    let mod_index = 0.45 * (-6.0 * t).exp();
                    (phase + mod_index * (2.0 * phase).sin()).sin() * 0.8
                        + 0.2 * (2.0 * phase).sin()
                },
            );
        }

        // 3. Warm detuned pad chord voice
        let chord_norm = 1.0 / (event.pad_chord.len().max(1) as f32).sqrt();
        let pad_gate = (event.duration_sec * 0.92).max(0.12);
        let (pl, pr) = pan_gains(-event.pan * 0.35);
        for &midi in &event.pad_chord {
            let freq = midi_to_hz(midi);
            add_voice(
                &mut mix,
                event.start_sec,
                pad_gate,
                pad_env,
                event.velocity * 0.22 * chord_norm,
                pl,
                pr,
                |t| {
                    let p1 = TAU * freq * t;
                    let p2 = TAU * (freq * 1.0035) * t;
                    0.5 * p1.sin() + 0.4 * p2.sin() + 0.1 * (2.0 * p1).sin()
                },
            );
        }
    }

    // Soft master limiter so polyphonic sums remain clean and never hard-clip.
    for sample in &mut mix {
        sample.0 = (sample.0 * 0.75).tanh();
        sample.1 = (sample.1 * 0.75).tanh();
    }

    mix
}

#[allow(clippy::too_many_arguments)]
fn add_voice<F>(
    mix: &mut [(f32, f32)],
    start_sec: f32,
    gate_sec: f32,
    env: AdsrEnvelope,
    gain: f32,
    pan_l: f32,
    pan_r: f32,
    osc: F,
) where
    F: Fn(f32) -> f32,
{
    let start_idx = ((start_sec * SAMPLE_RATE as f32).round() as usize).min(mix.len());
    let total_samples = (env.total_duration(gate_sec) * SAMPLE_RATE as f32).ceil() as usize;
    let end_idx = start_idx.saturating_add(total_samples).min(mix.len());

    for (offset, slot) in mix[start_idx..end_idx].iter_mut().enumerate() {
        let t = offset as f32 / SAMPLE_RATE as f32;
        let amp = env.amplitude(t, gate_sec) * gain;
        let sample = osc(t) * amp;
        slot.0 += sample * pan_l;
        slot.1 += sample * pan_r;
    }
}

/// Encode stereo float samples in `[-1.0, 1.0]` as a standard 16-bit PCM WAV file.
pub fn encode_wav_bytes(samples: &[(f32, f32)], sample_rate: u32) -> Vec<u8> {
    let channels: u16 = 2;
    let bits_per_sample: u16 = 16;
    let block_align: u16 = channels * (bits_per_sample / 8);
    let byte_rate: u32 = sample_rate * u32::from(block_align);
    let data_size: u32 = (samples.len() as u32).saturating_mul(u32::from(block_align));
    let riff_size: u32 = 36u32.saturating_add(data_size);

    let mut buf = Vec::with_capacity(44 + data_size as usize);
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&riff_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes()); // PCM subchunk size
    buf.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat = 1 (PCM)
    buf.extend_from_slice(&channels.to_le_bytes());
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    buf.extend_from_slice(&byte_rate.to_le_bytes());
    buf.extend_from_slice(&block_align.to_le_bytes());
    buf.extend_from_slice(&bits_per_sample.to_le_bytes());
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_size.to_le_bytes());

    for &(left, right) in samples {
        let l_i16 = (left.clamp(-1.0, 1.0) * 32767.0).round() as i16;
        let r_i16 = (right.clamp(-1.0, 1.0) * 32767.0).round() as i16;
        buf.extend_from_slice(&l_i16.to_le_bytes());
        buf.extend_from_slice(&r_i16.to_le_bytes());
    }
    buf
}

pub fn write_wav_file(path: &Path, samples: &[(f32, f32)], sample_rate: u32) -> io::Result<()> {
    let bytes = encode_wav_bytes(samples, sample_rate);
    let mut file = File::create(path)?;
    file.write_all(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git_log::sample_commits;
    use crate::music::{commits_to_score, Scale};

    #[test]
    fn adsr_envelope_stays_within_unit_bounds() {
        let env = AdsrEnvelope {
            attack_sec: 0.05,
            decay_sec: 0.10,
            sustain_level: 0.6,
            release_sec: 0.20,
        };
        let gate = 0.40;

        assert_eq!(env.amplitude(-0.01, gate), 0.0);
        assert_eq!(env.amplitude(0.0, gate), 0.0);

        let peak = env.amplitude(0.05, gate);
        assert!((peak - 1.0).abs() < 1e-4);

        let sustain = env.amplitude(0.25, gate);
        assert!((sustain - 0.6).abs() < 1e-4);

        let mid_release = env.amplitude(0.50, gate);
        assert!((mid_release - 0.3).abs() < 1e-4);

        assert_eq!(env.amplitude(0.65, gate), 0.0);

        // Sweep across entire timeline including short-gate interruption
        for step in 0..=100 {
            let t = step as f32 * 0.01;
            let amp = env.amplitude(t, 0.03);
            assert!((0.0..=1.0).contains(&amp), "amp {amp} out of bounds at {t}");
        }
    }

    #[test]
    fn wav_header_and_riff_structure_are_valid() {
        let samples = vec![(0.0, 0.0), (0.5, -0.5), (-1.0, 1.0), (0.25, -0.25)];
        let bytes = encode_wav_bytes(&samples, SAMPLE_RATE);

        assert_eq!(bytes.len(), 44 + samples.len() * 4);
        assert_eq!(&bytes[0..4], b"RIFF");
        let riff_size = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        assert_eq!(riff_size, (bytes.len() - 8) as u32);
        assert_eq!(&bytes[8..12], b"WAVE");
        assert_eq!(&bytes[12..16], b"fmt ");
        let fmt_size = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        assert_eq!(fmt_size, 16);
        let audio_format = u16::from_le_bytes(bytes[20..22].try_into().unwrap());
        assert_eq!(audio_format, 1);
        let channels = u16::from_le_bytes(bytes[22..24].try_into().unwrap());
        assert_eq!(channels, 2);
        let rate = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
        assert_eq!(rate, SAMPLE_RATE);
        let bits = u16::from_le_bytes(bytes[34..36].try_into().unwrap());
        assert_eq!(bits, 16);
        assert_eq!(&bytes[36..40], b"data");
        let data_size = u32::from_le_bytes(bytes[40..44].try_into().unwrap());
        assert_eq!(data_size, (samples.len() * 4) as u32);
    }

    #[test]
    fn rendered_score_is_audible_and_bounded() {
        let commits = sample_commits();
        let score = commits_to_score(&commits[..4], Scale::Pentatonic, 120.0, 48);
        let samples = render_score(&score);

        assert!(!samples.is_empty());
        let peak = samples
            .iter()
            .fold(0.0_f32, |acc, &(l, r)| acc.max(l.abs()).max(r.abs()));
        assert!(peak > 0.05 && peak <= 1.0, "unexpected peak: {peak}");
    }
}
