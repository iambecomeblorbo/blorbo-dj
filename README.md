# CODEX
project: blorbo_dj
project_path: /home/andrew/blorbo_dj
generated_by: yggdrasil-cli
timestamp_unix: 1762648907
format: markdown

## INDEX
./music/scripts/icebeat.blorb: 12
./music/output/README.md: 3
./Cargo.toml: 10
./src/generator.rs: 84
./src/main.rs: 117
total_loc: 226

## FILES
<file path="./music/scripts/icebeat.blorb" lang="blorb" lines="12">
```blorb
# Blorbo's First Song: Ice Beat Sonata ❄️
C4:0.5
E4:0.5
G4:1.0
rest:0.25
C5:1.5
E5:0.5
rest:0.25
D5:1.0
B4:0.75
C5:0.75
rest:0.5
```
</file>

<file path="./music/output/README.md" lang="markdown" lines="3">
```markdown
# Placeholder

blorbo needs this placeholder this is where files output
```
</file>

<file path="./Cargo.toml" lang="toml" lines="10">
```toml
[package]
name = "blorbo_dj"
version = "0.1.0"
edition = "2021"

[dependencies]
rodio = "0.17"
crossterm = "0.27"
fundsp = "0.15"
hound = "3.5"
```
</file>

<file path="./src/generator.rs" lang="rust" lines="84">
```rust
use fundsp::hacker::*;
use hound;
use std::fs::read_to_string;
use std::collections::HashMap;
use std::process::Command;

/// Map note names like "C4" to their frequencies in Hz
fn note_to_freq(note: &str) -> Option<f64> {
    let notes = HashMap::from([
        ("C3", 130.81), ("D3", 146.83), ("E3", 164.81), ("F3", 174.61),
        ("G3", 196.00), ("A3", 220.00), ("B3", 246.94),
        ("C4", 261.63), ("D4", 293.66), ("E4", 329.63), ("F4", 349.23),
        ("G4", 392.00), ("A4", 440.00), ("B4", 493.88),
        ("C5", 523.25), ("D5", 587.33), ("E5", 659.25), ("F5", 698.46),
        ("G5", 783.99), ("A5", 880.00), ("B5", 987.77),
    ]);
    notes.get(note).copied()
}

/// Reads a .blorb file and generates a WAV file from it
pub fn generate_from_blorb(blorb_path: &str, output_wav: &str, duration_scale: f64) {
    let sample_rate: u32 = 44100;

    let contents = read_to_string(blorb_path)
        .unwrap_or_else(|_| panic!("💥 Could not read blorb file at: {}", blorb_path));

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_wav, spec)
        .expect("🐟 Couldn't open blorb output WAV file");

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            println!("⚠️  Invalid blorb line (ignored): {}", line);
            continue;
        }

        let note = parts[0];
        let duration_secs: f64 = parts[1].parse().unwrap_or(0.5) * duration_scale;
        let total_samples = (sample_rate as f64 * duration_secs) as usize;

        if note == "rest" {
            for _ in 0..total_samples {
                writer.write_sample(0i16).unwrap();
            }
        } else if let Some(freq) = note_to_freq(note) {
            let mut osc = sine_hz(freq);
            for _ in 0..total_samples {
                let sample = osc.get_stereo().0;
                let amplitude = (sample * i16::MAX as f64) as i16;
                writer.write_sample(amplitude).unwrap();
            }
        } else {
            println!("⚠️  Unknown blorb note: {}", note);
        }
    }

    println!("🎶 Done writing WAV to: {}", output_wav);
}

/// Converts WAV into MP3 using ffmpeg
pub fn convert_wav_to_mp3(wav_path: &str, mp3_path: &str) {
    let status = Command::new("ffmpeg")
        .args(["-y", "-i", wav_path, mp3_path])
        .status()
        .expect("🧊 ffmpeg failed");

    if status.success() {
        println!("🎧 MP3 exported to {}", mp3_path);
    } else {
        println!("❌ ffmpeg died. No MP3 for Blorbo.");
    }
}
```
</file>

<file path="./src/main.rs" lang="rust" lines="117">
```rust
mod generator;

use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, path::Path,  time::Duration};
use crossterm::event::{self, Event, KeyCode};
use std::io::{self, Write};

fn main() {
    println!("🧊 Welcome to Blorbo DJ Terminal 🎧");
    println!("press [1] lofi | [2] techno | [3] fresh blorb | [d] compile .blorb | [4] play .blorb | [q] cry");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut current_sink: Option<Sink> = None;

    loop {
        if event::poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                let key = key_event.code;
                handle_keypress(key, &stream_handle, &mut current_sink);
            }
        }
    }
}

// 🐧 Handles keypress logic for Blorbo’s little DJ flippers
fn handle_keypress(
    key: KeyCode,
    stream_handle: &rodio::OutputStreamHandle,
    current_sink: &mut Option<Sink>,
) {
    match key {
        KeyCode::Char('1') => {
            blorbo_play("🎵 vibe: LOFI ICE CAPS", "music/static/blorbo_lofi.mp3", stream_handle, current_sink);
        }
        KeyCode::Char('2') => {
            blorbo_play("🎶 vibe: TECHNO GROTTO", "music/static/blorbo_techno.mp3", stream_handle, current_sink);
        }
        KeyCode::Char('3') => {
            blorbo_play("✨ vibe: BLORBO'S CUSTOM TRACK", "music/output/blorbo_theme.mp3", stream_handle, current_sink);
        }
        KeyCode::Char('4') => {
            blorbo_play("🧊 Playing DSL .blorb track", "music/output/dsl_track.mp3", stream_handle, current_sink);
        }
        KeyCode::Char('d') => {
            println!("📜 Compiling song1.blorb...");
            compile_blorb();
        }
        KeyCode::Char('q') => {
            println!("🐧 Blorbo exits. Ice cracks. Stars blink.");
            std::process::exit(0);
        }
        _ => {}
    }
}

// ❄️ Plays a given track path with some dramatic text
fn blorbo_play(message: &str, path: &str, stream_handle: &rodio::OutputStreamHandle, current_sink: &mut Option<Sink>) {
    println!("{}", message);
    if let Some(ref sink) = current_sink {
        sink.stop();
    }
    *current_sink = load_track(path, stream_handle);
}



fn compile_blorb() {
    print!("🎹 Enter filename from 'music/scripts/ (e.g. icebeat.blorb): ");
    io::stdout().flush().unwrap();

    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim(); // e.g., "antarctica.blorb"

    if filename.is_empty() || !filename.ends_with(".blorb") {
        println!("❌ You must enter a real .blorb file, not a dream.");
        return;
    }

    let input_path = format!("music/scripts/{}", filename);
    let file_stem = Path::new(&input_path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();

    let output_wav = format!("music/output/{}.wav", file_stem);
    let output_mp3 = format!("music/output/{}.mp3", file_stem);

    if !Path::new(&input_path).exists() {
        println!("❌ Blorbo can’t find this file: {}", input_path);
        return;
    }

    println!("📜 Compiling '{}' into frosty waveform magic...", filename);

    generator::generate_from_blorb(&input_path, &output_wav, 1.0);
    generator::convert_wav_to_mp3(&output_wav, &output_mp3);

    println!("✅ Success! Your MP3 is at: {}", output_mp3);
}



// 🐟 Loads a track from file into a sink
fn load_track(file_path: &str, stream_handle: &rodio::OutputStreamHandle) -> Option<Sink> {
    if !Path::new(file_path).exists() {
        println!("⚠️  Missing file: {file_path}");
        return None;
    }

    let file = File::open(Path::new(file_path)).expect("🐟 blorbo couldn't catch this track");
    let source = Decoder::new(BufReader::new(file)).unwrap();

    let sink = Sink::try_new(stream_handle).unwrap();
    sink.append(source);
    Some(sink)
}
```
</file>

