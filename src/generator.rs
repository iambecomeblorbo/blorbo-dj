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
