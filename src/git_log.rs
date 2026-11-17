use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitRecord {
    pub hash: String,
    pub timestamp: u64,
    pub author: String,
    pub subject: String,
    pub insertions: u32,
    pub deletions: u32,
    pub files_changed: u32,
    pub primary_ext: String,
    pub extensions: Vec<String>,
}

impl CommitRecord {
    pub fn short_hash(&self) -> &str {
        if self.hash.len() >= 7 {
            &self.hash[..7]
        } else {
            &self.hash
        }
    }

    pub fn total_churn(&self) -> u32 {
        self.insertions.saturating_add(self.deletions)
    }
}

const COMMIT_MARKER: &str = "@@@COMMIT@@@";

pub fn read_repo_commits(repo_path: &Path, max_commits: usize) -> Result<Vec<CommitRecord>, String> {
    let limit = max_commits.max(1).to_string();
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("log")
        .arg("-n")
        .arg(&limit)
        .arg("--reverse")
        .arg("--numstat")
        .arg(format!("--format={COMMIT_MARKER}%n%H%n%at%n%an%n%s"))
        .output()
        .map_err(|err| format!("failed to execute git: {err}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git log failed: {}", stderr.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let commits = parse_git_log(&stdout);
    if commits.is_empty() {
        return Err("no commits found in repository".to_string());
    }
    Ok(commits)
}

pub fn sample_commits() -> Vec<CommitRecord> {
    parse_git_log(SAMPLE_GIT_LOG)
}

pub fn parse_git_log(raw: &str) -> Vec<CommitRecord> {
    let mut commits = Vec::new();
    for block in raw.split(COMMIT_MARKER) {
        let trimmed = block.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(record) = parse_commit_block(trimmed) {
            commits.push(record);
        }
    }
    commits
}

fn parse_commit_block(block: &str) -> Option<CommitRecord> {
    let mut lines = block.lines();
    let hash = lines.next()?.trim().to_string();
    let timestamp = lines.next()?.trim().parse::<u64>().ok()?;
    let author = lines.next()?.trim().to_string();
    let subject = lines.next().unwrap_or("").trim().to_string();

    if hash.is_empty() {
        return None;
    }

    let mut insertions: u32 = 0;
    let mut deletions: u32 = 0;
    let mut files_changed: u32 = 0;
    let mut ext_weights: HashMap<String, u32> = HashMap::new();
    let mut extensions: Vec<String> = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(3, '\t');
        let Some(ins_str) = parts.next() else {
            continue;
        };
        let Some(del_str) = parts.next() else {
            continue;
        };
        let Some(path_str) = parts.next() else {
            continue;
        };

        let ins = ins_str.trim().parse::<u32>().unwrap_or(0);
        let del = del_str.trim().parse::<u32>().unwrap_or(0);
        insertions = insertions.saturating_add(ins);
        deletions = deletions.saturating_add(del);
        files_changed = files_changed.saturating_add(1);

        let ext = extract_extension(path_str);
        let weight = ins.saturating_add(del).max(1);
        *ext_weights.entry(ext.clone()).or_insert(0) += weight;
        if !extensions.contains(&ext) {
            extensions.push(ext);
        }
    }

    let primary_ext = ext_weights
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        .map(|(ext, _)| ext)
        .unwrap_or_else(|| "txt".to_string());

    Some(CommitRecord {
        hash,
        timestamp,
        author,
        subject,
        insertions,
        deletions,
        files_changed,
        primary_ext,
        extensions,
    })
}

pub fn extract_extension(path: &str) -> String {
    let cleaned = if let Some((_, after)) = path.rsplit_once("=>") {
        after.trim().trim_end_matches('}').trim()
    } else {
        path.trim()
    };

    let filename = cleaned.rsplit('/').next().unwrap_or(cleaned);
    match filename.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() && !ext.is_empty() => ext.to_ascii_lowercase(),
        _ => "other".to_string(),
    }
}

const SAMPLE_GIT_LOG: &str = "\
@@@COMMIT@@@
a104f9c2d81e4b3a9f0c11223344556677889901
1700000100
Ada Lovelace
feat: initialize core synthesizer crate and cargo manifest
42\t0\tCargo.toml
18\t0\tREADME.md
64\t0\tsrc/main.rs
@@@COMMIT@@@
b218e7a1c4093d2e8a1b22334455667788990012
1700003700
Grace Hopper
feat: implement git log numstat parser
115\t4\tsrc/git_log.rs
22\t3\tsrc/main.rs
@@@COMMIT@@@
c329d6f0b5182c1d7f2a33445566778899001123
1700009200
Ken Thompson
feat: add musical scale quantization and voice mapping
148\t12\tsrc/music.rs
19\t5\tsrc/lib.rs
@@@COMMIT@@@
d43af5e9a6271b0c6e3b44556677889900112234
1700016400
Barbara Liskov
feat: polyphonic ADSR oscillator and PCM 16-bit WAV writer
210\t18\tsrc/synth.rs
35\t9\tsrc/main.rs
@@@COMMIT@@@
e54be4d897360a9b5d4c55667788990011223345
1700024800
Margaret Hamilton
test: add RIFF header and envelope boundary tests
96\t6\tsrc/synth.rs
44\t0\tsrc/music.rs
@@@COMMIT@@@
f65cd3c788459f8a4c5d66778899001122334456
1700039200
Linus Torvalds
refactor: optimize stereo pan law and soft limiter
38\t41\tsrc/synth.rs
14\t11\tsrc/viz.rs
@@@COMMIT@@@
076dc2b679548e793b6e77889900112233445567
1700051200
Ada Lovelace
feat(web): build Web Audio polyphonic engine and piano roll
185\t0\tweb/app.js
94\t0\tweb/styles.css
58\t0\tweb/index.html
@@@COMMIT@@@
187eb1a560637d682a7f88990011223344556678
1700068400
Grace Hopper
fix(web): handle offline presets when API rate-limits
74\t19\tweb/app.js
12\t4\tweb/index.html
@@@COMMIT@@@
298fa09451726c57198099001122334455667789
1700081000
Ken Thompson
feat(viz): render TrueColor ANSI ASCII spectrogram timeline
128\t14\tsrc/viz.rs
26\t8\tsrc/main.rs
@@@COMMIT@@@
3090b18342815b4608910011223344556677889a
1700095600
Barbara Liskov
ci: add fmt, clippy, test, wav smoke and pages workflow
72\t0\t.github/workflows/ci.yml
31\t7\tREADME.md
@@@COMMIT@@@
41a1c27233904a35f7a2112233445566778899ab
1700112000
Margaret Hamilton
docs: document commit-to-music harmonic mapping
64\t12\tREADME.md
11\t3\tsrc/music.rs
@@@COMMIT@@@
52b2d361240f3924e6b32233445566778899aabc
1700128400
Linus Torvalds
perf: precompute wav sample buffers and clamp peak levels
53\t29\tsrc/synth.rs
8\t2\tCargo.toml
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multi_commit_numstat_with_binary_and_renames() {
        let raw = "\
@@@COMMIT@@@
1234567890abcdef1234567890abcdef12345678
1700000000
Alice
feat: add parser
10\t2\tsrc/main.rs
-\t-\tassets/logo.png
5\t1\tsrc/{old.rs => new.rs}
@@@COMMIT@@@
abcdef1234567890abcdef1234567890abcdef12
1700000500
Bob
chore: empty merge commit
";

        let commits = parse_git_log(raw);
        assert_eq!(commits.len(), 2);

        let first = &commits[0];
        assert_eq!(first.short_hash(), "1234567");
        assert_eq!(first.timestamp, 1_700_000_000);
        assert_eq!(first.author, "Alice");
        assert_eq!(first.subject, "feat: add parser");
        assert_eq!(first.insertions, 15);
        assert_eq!(first.deletions, 3);
        assert_eq!(first.files_changed, 3);
        assert_eq!(first.primary_ext, "rs");
        assert_eq!(first.extensions, vec!["rs", "png"]);

        let second = &commits[1];
        assert_eq!(second.total_churn(), 0);
        assert_eq!(second.files_changed, 0);
        assert_eq!(second.primary_ext, "txt");
    }

    #[test]
    fn extracts_file_extensions_cleanly() {
        assert_eq!(extract_extension("src/synth.rs"), "rs");
        assert_eq!(extract_extension("web/App.TSX"), "tsx");
        assert_eq!(extract_extension("Makefile"), "other");
        assert_eq!(extract_extension(".gitignore"), "other");
        assert_eq!(extract_extension("src/{a.py => b.rs}"), "rs");
    }

    #[test]
    fn sample_commits_are_populated() {
        let commits = sample_commits();
        assert!(commits.len() >= 10);
        assert!(commits.iter().all(|c| !c.hash.is_empty()));
    }
}
