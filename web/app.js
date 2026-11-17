const SCALES = {
  pentatonic: [0, 3, 5, 7, 10],
  dorian: [0, 2, 3, 5, 7, 9, 10],
  lydian: [0, 2, 4, 6, 7, 9, 11],
  minor: [0, 2, 3, 5, 7, 8, 10],
};

const EXT_COLORS = {
  py: "#38bdf8",
  c: "#f59e0b",
  rs: "#fb923c",
  js: "#facc15",
  ts: "#60a5fa",
  md: "#94a3b8",
  css: "#f472b6",
  html: "#fb7185",
  other: "#34d399",
};

const OFFLINE_PRESETS = {
  "karpathy/micrograd": [
    { sha: "8a2f1c0", author: "Andrej Karpathy", date: "2020-04-12", ext: "py", ins: 142, del: 0, msg: "initial scalar autograd engine (Value)" },
    { sha: "9b4e31a", author: "Andrej Karpathy", date: "2020-04-13", ext: "py", ins: 98, del: 12, msg: "add MLP, Layer, Neuron modules in nn.py" },
    { sha: "c17d40b", author: "Andrej Karpathy", date: "2020-04-15", ext: "py", ins: 64, del: 8, msg: "add topological sort backward pass + relu/tanh" },
    { sha: "d32a88c", author: "Andrej Karpathy", date: "2020-04-16", ext: "ipynb", ins: 210, del: 14, msg: "demo notebook training moon classifier" },
    { sha: "e45f12d", author: "Andrej Karpathy", date: "2020-04-18", ext: "py", ins: 44, del: 19, msg: "verify gradients against PyTorch in test_engine.py" },
    { sha: "f51b77e", author: "Andrej Karpathy", date: "2020-04-20", ext: "md", ins: 86, del: 6, msg: "document backprop graph visualization in README" },
    { sha: "194c62f", author: "Contributor", date: "2020-07-09", ext: "py", ins: 29, del: 7, msg: "support __radd__, __rmul__, and __truediv__" },
    { sha: "2a6d810", author: "Andrej Karpathy", date: "2020-11-04", ext: "py", ins: 37, del: 15, msg: "clean up SGD step and zero_grad example" },
    { sha: "3c8e941", author: "Contributor", date: "2021-05-19", ext: "py", ins: 52, del: 11, msg: "add setup.py package metadata" },
    { sha: "4d1f052", author: "Andrej Karpathy", date: "2022-08-17", ext: "md", ins: 40, del: 4, msg: "link zero-to-hero autograd lecture" },
  ],
  "torvalds/linux": [
    { sha: "1da177e", author: "Linus Torvalds", date: "2005-04-16", ext: "c", ins: 670, del: 0, msg: "Linux-2.6.12-rc2 initial git repository import" },
    { sha: "4f2a90c", author: "Ingo Molnar", date: "2008-01-25", ext: "c", ins: 312, del: 118, msg: "sched: completely fair scheduler refinements" },
    { sha: "7b3c14d", author: "Greg KH", date: "2011-05-29", ext: "c", ins: 184, del: 92, msg: "usb: xhci host controller power management" },
    { sha: "9e4d22f", author: "David Miller", date: "2014-08-03", ext: "c", ins: 248, del: 77, msg: "net: lockless skb ring buffer optimization" },
    { sha: "a82f11a", author: "Thomas Gleixner", date: "2017-11-12", ext: "c", ins: 156, del: 64, msg: "x86/timers: high-resolution clocksource tickless path" },
    { sha: "b93e44c", author: "Jens Axboe", date: "2019-03-04", ext: "c", ins: 420, del: 38, msg: "io_uring: asynchronous I/O ring submission queue" },
    { sha: "c41a78b", author: "Miguel Ojeda", date: "2022-10-02", ext: "rs", ins: 390, del: 12, msg: "rust: initial Rust language support in kernel Kbuild" },
    { sha: "d72b90e", author: "Linus Torvalds", date: "2024-03-10", ext: "c", ins: 112, del: 45, msg: "mm: folio batching cleanup across page cache" },
    { sha: "e84c31f", author: "Linus Torvalds", date: "2025-01-19", ext: "c", ins: 94, del: 31, msg: "Merge tag 'vfs-6.14' into master" },
    { sha: "f95d42a", author: "Greg KH", date: "2025-02-08", ext: "md", ins: 48, del: 14, msg: "docs: update stable tree maintainer guidelines" },
  ],
  "facebook/react": [
    { sha: "75897c2", author: "Jordan Walke", date: "2013-05-29", ext: "js", ins: 520, del: 0, msg: "Initial public open-source React commit" },
    { sha: "3c4a12b", author: "Sebastian Markbage", date: "2016-07-11", ext: "js", ins: 380, del: 140, msg: "Fiber: incremental cooperative work loop scheduler" },
    { sha: "8d2b44e", author: "Dan Abramov", date: "2018-10-25", ext: "js", ins: 265, del: 48, msg: "Hooks: useState and useEffect dispatcher implementation" },
    { sha: "a19f73c", author: "Andrew Clark", date: "2020-08-10", ext: "js", ins: 192, del: 85, msg: "Concurrent lanes priority bitmask scheduler" },
    { sha: "c54e29a", author: "Sophie Alpert", date: "2021-06-08", ext: "js", ins: 148, del: 32, msg: "startTransition and selective hydration boundaries" },
    { sha: "d87a10f", author: "Lauren Tan", date: "2023-05-03", ext: "ts", ins: 230, del: 60, msg: "Server Components flight stream decoder" },
    { sha: "e91c64b", author: "Sathya Gunasekaran", date: "2024-04-12", ext: "ts", ins: 310, del: 94, msg: "React Compiler: reactive dependency memoization pass" },
    { sha: "f02d88c", author: "Rick Hanlon", date: "2025-01-22", ext: "js", ins: 118, del: 41, msg: "React 19 Actions & useOptimistic refinements" },
  ],
  "Dafenxz0/git-synth": [
    { sha: "a104f9c", author: "Dafenxz0", date: "2026-11-18", ext: "rs", ins: 124, del: 0, msg: "init git-synth Rust crate and pure-std manifest" },
    { sha: "b218e7a", author: "Dafenxz0", date: "2026-11-18", ext: "rs", ins: 141, del: 7, msg: "feat: git log --numstat commit and extension parser" },
    { sha: "c329d6f", author: "Dafenxz0", date: "2026-11-18", ext: "rs", ins: 167, del: 12, msg: "feat: scale quantization (Pentatonic/Dorian/Lydian/Minor)" },
    { sha: "d43af5e", author: "Dafenxz0", date: "2026-11-18", ext: "rs", ins: 245, del: 18, msg: "feat: polyphonic ADSR voices + 16-bit PCM WAV writer" },
    { sha: "e54be4d", author: "Dafenxz0", date: "2026-11-18", ext: "rs", ins: 140, del: 6, msg: "feat: Goertzel ASCII spectrogram terminal timeline" },
    { sha: "076dc2b", author: "Dafenxz0", date: "2026-11-18", ext: "js", ins: 337, del: 0, msg: "feat(web): Web Audio synthesizer + commit-particle piano roll" },
    { sha: "3090b18", author: "Dafenxz0", date: "2026-11-18", ext: "md", ins: 103, del: 7, msg: "ci: fmt, clippy, test, wav smoke test, GitHub Pages deploy" },
  ],
};

function fnv1a(text) {
  let hash = 0x811c9dc5;
  for (let i = 0; i < text.length; i += 1) {
    hash ^= text.charCodeAt(i);
    hash = Math.imul(hash, 0x01000193);
  }
  return hash >>> 0;
}

function extOffset(ext) {
  const table = { rs: 0, py: 1, js: 2, ts: 2, c: 3, cpp: 3, go: 3, md: 4, css: 5, html: 5 };
  return ext in table ? table[ext] : fnv1a(ext) % 7;
}

function degreeToMidi(rootMidi, scaleName, degree, octaveOffset) {
  const intervals = SCALES[scaleName] || SCALES.pentatonic;
  const len = intervals.length;
  const octaves = Math.floor(degree / len) + octaveOffset;
  const interval = intervals[((degree % len) + len) % len];
  return Math.max(24, Math.min(108, rootMidi + octaves * 12 + interval));
}

function midiToHz(midi) {
  return 440 * Math.pow(2, (midi - 69) / 12);
}

function midiToName(midi) {
  const names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
  return `${names[midi % 12]}${Math.floor(midi / 12) - 1}`;
}

function buildScore(commits, scaleName, bpm) {
  const beatSec = 60 / bpm;
  const rootMidi = 48; // C3
  const intervals = SCALES[scaleName] || SCALES.pentatonic;
  const scaleLen = intervals.length;
  let cursor = 0;

  return commits.map((c, idx) => {
    const churn = (c.ins || 0) + (c.del || 0);
    const beats = churn >= 220 ? 2.0 : churn >= 70 ? 1.5 : 1.0;
    const duration = beats * beatSec;
    const seed = fnv1a(c.sha);
    const authorSeed = fnv1a(c.author || "dev");
    const baseDegree = (seed + extOffset(c.ext || "other") + idx) % scaleLen;

    const bassMidi = degreeToMidi(rootMidi, scaleName, baseDegree, -1);
    const arpCount = churn >= 90 ? 4 : 3;
    const arpNotes = [];
    for (let s = 0; s < arpCount; s += 1) {
      const shift = ((seed >>> (s * 4)) + s * 2) % (scaleLen * 2);
      arpNotes.push(degreeToMidi(rootMidi, scaleName, baseDegree + shift, 1));
    }

    const padChord = [
      degreeToMidi(rootMidi, scaleName, baseDegree, 0),
      degreeToMidi(rootMidi, scaleName, baseDegree + 2, 0),
      degreeToMidi(rootMidi, scaleName, baseDegree + 4, 0),
    ];

    const authorPan = ((authorSeed % 140) / 70) - 1;
    const diffBias = churn > 0 ? ((c.ins || 0) - (c.del || 0)) / churn : 0;
    const pan = Math.max(-0.85, Math.min(0.85, authorPan * 0.65 + diffBias * 0.25));
    const velocity = Math.max(0.38, Math.min(0.95, 0.42 + Math.log2(1 + churn) * 0.055));

    const ev = {
      index: idx,
      startSec: cursor,
      durationSec: duration,
      bassMidi,
      arpNotes,
      padChord,
      pan,
      velocity,
      commit: c,
    };
    cursor += duration;
    return ev;
  });
}

// DOM Refs
const repoForm = document.getElementById("repo-form");
const repoInput = document.getElementById("repo-input");
const statusLine = document.getElementById("status-line");
const playBtn = document.getElementById("play-btn");
const stopBtn = document.getElementById("stop-btn");
const playheadReadout = document.getElementById("playhead-readout");
const scaleSelect = document.getElementById("scale-select");
const bpmSlider = document.getElementById("bpm-slider");
const bpmVal = document.getElementById("bpm-val");
const volumeSlider = document.getElementById("volume-slider");
const voiceBass = document.getElementById("voice-bass");
const voiceArp = document.getElementById("voice-arp");
const voicePad = document.getElementById("voice-pad");
const noteReadout = document.getElementById("note-readout");
const inspSha = document.getElementById("insp-sha");
const inspExt = document.getElementById("insp-ext");
const inspMsg = document.getElementById("insp-msg");
const inspAuthor = document.getElementById("insp-author");
const inspDate = document.getElementById("insp-date");
const inspPan = document.getElementById("insp-pan");
const commitCount = document.getElementById("commit-count");
const commitListEl = document.getElementById("commit-list");
const canvas = document.getElementById("visualizer");
const ctx2d = canvas.getContext("2d");

// State
let currentRepo = "karpathy/micrograd";
let rawCommits = [];
let score = [];
let isPlaying = false;
let currentIdx = 0;
let nextEventTime = 0;
let schedulerId = null;
let particles = [];
let activeFlashIdx = -1;
let activeFlashAlpha = 0;

// Web Audio graph
let audioCtx = null;
let masterGain = null;
let delayNode = null;
let analyser = null;
let fftBins = new Uint8Array(128);

function ensureAudio() {
  if (!audioCtx) {
    const AudioContextCtor = window.AudioContext || window.webkitAudioContext;
    audioCtx = new AudioContextCtor();
    masterGain = audioCtx.createGain();
    masterGain.gain.value = Number(volumeSlider.value) / 100 * 0.7;

    const compressor = audioCtx.createDynamicsCompressor();
    compressor.threshold.value = -14;
    compressor.knee.value = 18;
    compressor.ratio.value = 4;

    analyser = audioCtx.createAnalyser();
    analyser.fftSize = 256;
    analyser.smoothingTimeConstant = 0.82;
    fftBins = new Uint8Array(analyser.frequencyBinCount);

    // Subtle stereo delay send for depth
    delayNode = audioCtx.createDelay(1.0);
    delayNode.delayTime.value = 0.28;
    const delayFeedback = audioCtx.createGain();
    delayFeedback.gain.value = 0.25;
    const delayWet = audioCtx.createGain();
    delayWet.gain.value = 0.22;

    delayNode.connect(delayFeedback);
    delayFeedback.connect(delayNode);
    delayNode.connect(delayWet);
    delayWet.connect(compressor);

    masterGain.connect(compressor);
    compressor.connect(analyser);
    analyser.connect(audioCtx.destination);
  }
  if (audioCtx.state === "suspended") {
    audioCtx.resume();
  }
}

function playTone(freq, when, dur, type, gainPeak, pan, attack, release, sendEcho) {
  if (!audioCtx) return;
  const osc = audioCtx.createOscillator();
  const env = audioCtx.createGain();
  const panner = audioCtx.createStereoPanner();

  osc.type = type;
  osc.frequency.setValueAtTime(freq, when);
  panner.pan.setValueAtTime(pan, when);

  env.gain.setValueAtTime(0.0001, when);
  env.gain.linearRampToValueAtTime(gainPeak, when + attack);
  env.gain.exponentialRampToValueAtTime(Math.max(0.001, gainPeak * 0.35), when + dur * 0.75);
  env.gain.exponentialRampToValueAtTime(0.0001, when + dur + release);

  osc.connect(env);
  env.connect(panner);
  panner.connect(masterGain);
  if (sendEcho && delayNode) {
    panner.connect(delayNode);
  }

  osc.start(when);
  osc.stop(when + dur + release + 0.03);
}

function triggerCommitEvent(ev, when) {
  if (voiceBass.checked) {
    playTone(
      midiToHz(ev.bassMidi),
      when,
      ev.durationSec * 0.75,
      "triangle",
      ev.velocity * 0.42,
      ev.pan * 0.25,
      0.012,
      0.16,
      false
    );
  }

  if (voiceArp.checked) {
    const step = ev.durationSec / ev.arpNotes.length;
    ev.arpNotes.forEach((midi, i) => {
      playTone(
        midiToHz(midi),
        when + i * step,
        step * 0.75,
        "sine",
        ev.velocity * 0.28,
        ev.pan,
        0.008,
        0.18,
        true
      );
    });
  }

  if (voicePad.checked) {
    const chordGain = (ev.velocity * 0.16) / Math.sqrt(ev.padChord.length);
    ev.padChord.forEach((midi) => {
      playTone(
        midiToHz(midi),
        when,
        ev.durationSec * 0.9,
        "sawtooth",
        chordGain * 0.35,
        -ev.pan * 0.35,
        0.14,
        0.4,
        true
      );
      playTone(
        midiToHz(midi) * 1.003,
        when,
        ev.durationSec * 0.9,
        "triangle",
        chordGain * 0.65,
        -ev.pan * 0.35,
        0.14,
        0.4,
        false
      );
    });
  }

  // Schedule UI + particle burst aligned with audio time
  const delayMs = Math.max(0, (when - audioCtx.currentTime) * 1000);
  window.setTimeout(() => {
    if (!isPlaying) return;
    highlightCommit(ev);
    spawnCommitParticles(ev);
  }, delayMs);
}

function schedulerTick() {
  if (!isPlaying || !audioCtx || score.length === 0) return;
  const lookAhead = 0.12;
  while (nextEventTime < audioCtx.currentTime + lookAhead) {
    if (currentIdx >= score.length) {
      currentIdx = 0;
    }
    const ev = score[currentIdx];
    triggerCommitEvent(ev, nextEventTime);
    nextEventTime += ev.durationSec;
    currentIdx += 1;
  }
}

function startPlayback() {
  if (score.length === 0) return;
  ensureAudio();
  isPlaying = true;
  playBtn.textContent = "Pause";
  playBtn.classList.add("playing");
  nextEventTime = audioCtx.currentTime + 0.03;
  if (schedulerId) window.clearInterval(schedulerId);
  schedulerId = window.setInterval(schedulerTick, 25);
  schedulerTick();
}

function pausePlayback() {
  isPlaying = false;
  playBtn.textContent = "Play";
  playBtn.classList.remove("playing");
  if (schedulerId) {
    window.clearInterval(schedulerId);
    schedulerId = null;
  }
}

function stopPlayback() {
  pausePlayback();
  currentIdx = 0;
  if (score.length > 0) {
    highlightCommit(score[0]);
  }
  updateReadout();
}

function updateReadout() {
  const total = String(score.length).padStart(2, "0");
  const cur = String(Math.min(currentIdx + 1, score.length)).padStart(2, "0");
  playheadReadout.textContent = `${cur} / ${total}`;
}

function highlightCommit(ev) {
  activeFlashIdx = ev.index;
  activeFlashAlpha = 1.0;

  inspSha.textContent = ev.commit.sha;
  inspExt.textContent = `.${ev.commit.ext}`;
  inspMsg.textContent = ev.commit.msg;
  inspAuthor.textContent = ev.commit.author;
  inspDate.textContent = ev.commit.date;
  inspPan.textContent = `PAN ${ev.pan >= 0 ? "+" : ""}${ev.pan.toFixed(2)}`;
  noteReadout.textContent = `${midiToName(ev.bassMidi)} | ${ev.arpNotes.map(midiToName).join(" ")}`;
  updateReadout();

  const items = commitListEl.children;
  for (let i = 0; i < items.length; i += 1) {
    items[i].classList.toggle("active", i === ev.index);
  }
  const activeEl = items[ev.index];
  if (activeEl) {
    activeEl.scrollIntoView({ block: "nearest" });
  }
}

function spawnCommitParticles(ev) {
  const w = canvas.clientWidth || 800;
  const h = canvas.clientHeight || 400;
  const rollHeight = h * 0.74;
  const color = EXT_COLORS[ev.commit.ext] || EXT_COLORS.other;
  const playheadX = w * 0.24;

  ev.arpNotes.forEach((midi, idx) => {
    const y = midiToCanvasY(midi, rollHeight);
    const count = 4 + Math.min(8, Math.floor((ev.commit.ins + ev.commit.del) / 40));
    for (let p = 0; p < count; p += 1) {
      const angle = (Math.random() - 0.5) * Math.PI * 1.4;
      const speed = 1.1 + Math.random() * 2.6;
      particles.push({
        x: playheadX + idx * 8,
        y,
        vx: Math.cos(angle) * speed - 0.6,
        vy: Math.sin(angle) * speed,
        radius: 1.8 + Math.random() * 2.5,
        alpha: 0.95,
        color,
      });
    }
  });
}

function midiToCanvasY(midi, rollHeight) {
  const minMidi = 33;
  const maxMidi = 86;
  const norm = (midi - minMidi) / (maxMidi - minMidi);
  return rollHeight - 16 - Math.max(0, Math.min(1, norm)) * (rollHeight - 32);
}

function rebuildScore(keepIndex) {
  const scale = scaleSelect.value;
  const bpm = Number(bpmSlider.value);
  score = buildScore(rawCommits, scale, bpm);
  if (!keepIndex) currentIdx = 0;
  renderCommitQueue();
  if (score.length > 0) {
    highlightCommit(score[Math.min(currentIdx, score.length - 1)]);
  }
}

function renderCommitQueue() {
  commitCount.textContent = `${score.length} commits`;
  commitListEl.innerHTML = "";
  score.forEach((ev) => {
    const li = document.createElement("li");
    li.className = "commit-item";
    li.innerHTML = `
      <div class="commit-top mono">
        <span class="commit-sha">${ev.commit.sha}</span>
        <span>.${ev.commit.ext} · ${midiToName(ev.bassMidi)}</span>
      </div>
      <div class="commit-title"></div>
    `;
    li.querySelector(".commit-title").textContent = ev.commit.msg;
    li.addEventListener("click", () => {
      currentIdx = ev.index;
      highlightCommit(ev);
      if (!isPlaying) startPlayback();
      else if (audioCtx) nextEventTime = audioCtx.currentTime + 0.02;
    });
    commitListEl.appendChild(li);
  });
}

function setActivePresetChip(repoName) {
  document.querySelectorAll(".btn-chip").forEach((btn) => {
    btn.classList.toggle("active", btn.dataset.preset.toLowerCase() === repoName.toLowerCase());
  });
}

function loadPreset(repoName, reason) {
  const list = OFFLINE_PRESETS[repoName] || OFFLINE_PRESETS["karpathy/micrograd"];
  currentRepo = repoName;
  repoInput.value = repoName;
  setActivePresetChip(repoName);
  rawCommits = list.map((c) => ({ ...c }));
  rebuildScore(false);
  statusLine.textContent = reason || `Loaded offline preset (${repoName})`;
}

async function fetchGitHubRepo(ownerRepo) {
  const cleaned = ownerRepo.trim().replace(/^https?:\/\/github\.com\//i, "").replace(/\.git$/, "").replace(/\/+$/, "");
  const parts = cleaned.split("/");
  if (parts.length !== 2 || !parts[0] || !parts[1]) {
    statusLine.textContent = "Enter repository as owner/repo";
    return;
  }

  const slug = `${parts[0]}/${parts[1]}`;
  currentRepo = slug;
  setActivePresetChip(slug);
  statusLine.textContent = `Fetching ${slug} from GitHub API...`;

  try {
    const [commitsRes, langsRes] = await Promise.all([
      fetch(`https://api.github.com/repos/${slug}/commits?per_page=36`),
      fetch(`https://api.github.com/repos/${slug}/languages`),
    ]);

    if (!commitsRes.ok) {
      throw new Error(`GitHub API ${commitsRes.status}`);
    }

    const commitJson = await commitsRes.json();
    const langsJson = langsRes.ok ? await langsRes.json() : {};
    const exts = mapLanguagesToExts(langsJson);

    if (!Array.isArray(commitJson) || commitJson.length === 0) {
      throw new Error("No commits returned");
    }

    rawCommits = commitJson
      .slice()
      .reverse()
      .map((item, idx) => {
        const sha = (item.sha || "0000000").slice(0, 7);
        const msg = (item.commit?.message || "").split("\n")[0] || "commit";
        const author = item.commit?.author?.name || item.author?.login || "developer";
        const date = (item.commit?.author?.date || "").slice(0, 10) || "----";
        const seed = fnv1a(sha + msg);
        const ext = exts[(seed + idx) % exts.length] || "other";
        const ins = 18 + (seed % 240);
        const del = 4 + ((seed >>> 8) % 90);
        return { sha, author, date, ext, ins, del, msg };
      });

    rebuildScore(false);
    statusLine.textContent = `Live GitHub data: ${slug} (${rawCommits.length} commits)`;
  } catch (err) {
    if (OFFLINE_PRESETS[slug]) {
      loadPreset(slug, `GitHub API unavailable (${err.message}); using offline preset`);
    } else {
      statusLine.textContent = `Failed to load ${slug} (${err.message}); try a preset`;
    }
  }
}

function mapLanguagesToExts(langs) {
  const langMap = {
    Rust: "rs",
    Python: "py",
    JavaScript: "js",
    TypeScript: "ts",
    C: "c",
    "C++": "c",
    HTML: "html",
    CSS: "css",
    Shell: "other",
  };
  const keys = Object.keys(langs || {});
  if (keys.length === 0) return ["rs", "py", "js", "md"];
  return keys.slice(0, 5).map((k) => langMap[k] || "other");
}

// Canvas Renderer
function resizeCanvas() {
  const dpr = window.devicePixelRatio || 1;
  const rect = canvas.getBoundingClientRect();
  canvas.width = Math.max(1, Math.floor(rect.width * dpr));
  canvas.height = Math.max(1, Math.floor(rect.height * dpr));
  ctx2d.setTransform(dpr, 0, 0, dpr, 0, 0);
}

function drawFrame() {
  const w = canvas.clientWidth || 800;
  const h = canvas.clientHeight || 400;
  const rollHeight = h * 0.74;
  const specTop = rollHeight + 6;
  const specHeight = h - specTop - 8;
  const playheadX = w * 0.24;

  ctx2d.fillStyle = "#06080c";
  ctx2d.fillRect(0, 0, w, h);

  // Subtle horizontal pitch grid
  ctx2d.strokeStyle = "#111824";
  ctx2d.lineWidth = 1;
  for (let m = 36; m <= 84; m += 6) {
    const y = midiToCanvasY(m, rollHeight);
    ctx2d.beginPath();
    ctx2d.moveTo(0, y);
    ctx2d.lineTo(w, y);
    ctx2d.stroke();
  }

  // Playhead line
  ctx2d.strokeStyle = isPlaying ? "rgba(56, 189, 248, 0.7)" : "rgba(148, 163, 184, 0.3)";
  ctx2d.lineWidth = 1.5;
  ctx2d.beginPath();
  ctx2d.moveTo(playheadX, 0);
  ctx2d.lineTo(playheadX, rollHeight);
  ctx2d.stroke();

  // Scrolling Piano-Roll commit blocks
  const pxPerCommit = Math.max(38, Math.min(72, (w * 0.72) / Math.max(1, Math.min(score.length, 14))));
  const focusIdx = activeFlashIdx >= 0 ? activeFlashIdx : currentIdx;

  score.forEach((ev) => {
    const rel = ev.index - focusIdx;
    const x = playheadX + rel * pxPerCommit;
    if (x < -pxPerCommit || x > w + pxPerCommit) return;

    const isCurrent = ev.index === activeFlashIdx;
    const extColor = EXT_COLORS[ev.commit.ext] || EXT_COLORS.other;

    // Pad chord ribbon
    ctx2d.globalAlpha = isCurrent ? 0.36 : 0.14;
    ctx2d.fillStyle = "#34d399";
    ev.padChord.forEach((midi) => {
      const y = midiToCanvasY(midi, rollHeight);
      ctx2d.fillRect(x, y - 3, pxPerCommit * 0.85, 6);
    });

    // Bass pulse anchor
    ctx2d.globalAlpha = isCurrent ? 0.95 : 0.45;
    ctx2d.fillStyle = "#f59e0b";
    const bassY = midiToCanvasY(ev.bassMidi, rollHeight);
    ctx2d.fillRect(x, bassY - 4, pxPerCommit * 0.55, 8);

    // Arpeggio step blocks
    const stepW = (pxPerCommit * 0.82) / ev.arpNotes.length;
    ctx2d.fillStyle = extColor;
    ev.arpNotes.forEach((midi, sIdx) => {
      const y = midiToCanvasY(midi, rollHeight);
      ctx2d.globalAlpha = isCurrent ? 1.0 : 0.68;
      ctx2d.fillRect(x + sIdx * stepW, y - 3, Math.max(5, stepW - 3), 6);
    });
  });

  ctx2d.globalAlpha = 1;

  // Particles
  particles = particles.filter((p) => p.alpha > 0.03);
  for (const p of particles) {
    p.x += p.vx;
    p.y += p.vy;
    p.alpha *= 0.93;
    ctx2d.globalAlpha = p.alpha;
    ctx2d.fillStyle = p.color;
    ctx2d.beginPath();
    ctx2d.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
    ctx2d.fill();
  }
  ctx2d.globalAlpha = 1;

  if (activeFlashAlpha > 0) {
    activeFlashAlpha = Math.max(0, activeFlashAlpha - 0.04);
  }

  // Divider
  ctx2d.strokeStyle = "#1c2536";
  ctx2d.lineWidth = 1;
  ctx2d.beginPath();
  ctx2d.moveTo(0, rollHeight);
  ctx2d.lineTo(w, rollHeight);
  ctx2d.stroke();

  // Live FFT Frequency Spectrum
  if (analyser && isPlaying) {
    analyser.getByteFrequencyData(fftBins);
  } else {
    for (let i = 0; i < fftBins.length; i += 1) {
      fftBins[i] = Math.max(0, fftBins[i] - 4);
    }
  }

  const barCount = 48;
  const barW = (w - 24) / barCount;
  for (let i = 0; i < barCount; i += 1) {
    const val = fftBins[i] / 255;
    const bh = Math.max(2, val * specHeight);
    const bx = 12 + i * barW;
    const by = specTop + specHeight - bh;
    const hue = 195 - (i / barCount) * 145;
    ctx2d.fillStyle = `hsla(${hue}, 88%, 58%, 0.85)`;
    ctx2d.fillRect(bx, by, Math.max(2, barW - 3), bh);
  }

  window.requestAnimationFrame(drawFrame);
}

// Event bindings
repoForm.addEventListener("submit", (e) => {
  e.preventDefault();
  stopPlayback();
  fetchGitHubRepo(repoInput.value);
});

document.querySelectorAll(".btn-chip").forEach((btn) => {
  btn.addEventListener("click", () => {
    stopPlayback();
    loadPreset(btn.dataset.preset);
  });
});

playBtn.addEventListener("click", () => {
  if (isPlaying) pausePlayback();
  else startPlayback();
});

stopBtn.addEventListener("click", stopPlayback);

scaleSelect.addEventListener("change", () => rebuildScore(true));

bpmSlider.addEventListener("input", () => {
  bpmVal.textContent = bpmSlider.value;
  rebuildScore(true);
});

volumeSlider.addEventListener("input", () => {
  if (masterGain && audioCtx) {
    masterGain.gain.setTargetAtTime((Number(volumeSlider.value) / 100) * 0.7, audioCtx.currentTime, 0.02);
  }
});

canvas.addEventListener("click", (e) => {
  if (score.length === 0) return;
  const rect = canvas.getBoundingClientRect();
  const clickX = e.clientX - rect.left;
  const w = rect.width;
  const playheadX = w * 0.24;
  const pxPerCommit = Math.max(38, Math.min(72, (w * 0.72) / Math.max(1, Math.min(score.length, 14))));
  const focusIdx = activeFlashIdx >= 0 ? activeFlashIdx : currentIdx;
  const delta = Math.round((clickX - playheadX) / pxPerCommit);
  const target = Math.max(0, Math.min(score.length - 1, focusIdx + delta));
  currentIdx = target;
  highlightCommit(score[target]);
  if (!isPlaying) startPlayback();
  else if (audioCtx) nextEventTime = audioCtx.currentTime + 0.02;
});

window.addEventListener("resize", resizeCanvas);

// Boot
resizeCanvas();
loadPreset("karpathy/micrograd");
window.requestAnimationFrame(drawFrame);
