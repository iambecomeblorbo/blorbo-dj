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
