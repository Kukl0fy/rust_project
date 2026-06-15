use std::io::Cursor;
use std::path::Path;

use macroquad::audio::{load_sound, load_sound_from_bytes, play_sound, PlaySoundParams, Sound};

const MUSIC_DIR: &str = "music";
const MUSIC_EXTENSIONS: &[&str] = &["ogg", "wav", "mp3"];

pub struct MusicManager {
    track: Option<Sound>,
    playing: bool,
}

impl MusicManager {
    pub async fn new() -> Self {
        let path = find_music_file(MUSIC_DIR);
        let track = match path {
            Some(path) => match load_music(&path).await {
                Ok(sound) => {
                    println!("Loaded music: {path}");
                    Some(sound)
                }
                Err(err) => {
                    eprintln!("Failed to load music '{path}': {err}");
                    None
                }
            },
            None => {
                eprintln!("No music file found in '{MUSIC_DIR}/'");
                None
            }
        };

        Self {
            track,
            playing: false,
        }
    }

    pub fn play_looping(&mut self) {
        if self.playing {
            return;
        }

        if let Some(track) = &self.track {
            play_sound(
                track,
                PlaySoundParams {
                    looped: true,
                    volume: 0.55,
                },
            );
            self.playing = true;
        }
    }
}

async fn load_music(path: &str) -> Result<Sound, String> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "mp3" => {
            let wav_bytes = mp3_to_wav_bytes(path).map_err(|e| e.to_string())?;
            load_sound_from_bytes(&wav_bytes)
                .await
                .map_err(|e| e.to_string())
        }
        "ogg" | "wav" => load_sound(path).await.map_err(|e| e.to_string()),
        _ => Err(format!("unsupported music format: {ext}")),
    }
}

fn mp3_to_wav_bytes(path: &str) -> std::io::Result<Vec<u8>> {
    let mp3_data = std::fs::read(path)?;
    let mut decoder = minimp3::Decoder::new(Cursor::new(mp3_data));

    let mut samples = Vec::new();
    let mut sample_rate = 44_100u32;
    let mut channels = 1u16;

    loop {
        match decoder.next_frame() {
            Ok(frame) => {
                sample_rate = frame.sample_rate as u32;
                channels = frame.channels as u16;
                samples.extend_from_slice(&frame.data);
            }
            Err(minimp3::Error::Eof) => break,
            Err(err) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("{err}"),
                ));
            }
        }
    }

    if samples.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "mp3 file contains no audio samples",
        ));
    }

    let mut wav_data = Vec::new();
    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::new(Cursor::new(&mut wav_data), spec)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    for sample in samples {
        writer
            .write_sample(sample)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    }
    writer
        .finalize()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(wav_data)
}

fn find_music_file(dir: &str) -> Option<String> {
    let entries = std::fs::read_dir(dir).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path.extension()?.to_str()?.to_lowercase();
        if MUSIC_EXTENSIONS.contains(&ext.as_str()) {
            return path_to_str(path);
        }
    }

    None
}

fn path_to_str(path: std::path::PathBuf) -> Option<String> {
    path.to_str().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_project_mp3_to_wav_bytes() {
        let path = "music/geireczkaop.mp3";
        if !Path::new(path).exists() {
            return;
        }

        let wav = mp3_to_wav_bytes(path).expect("mp3 should convert to wav");
        assert!(wav.len() > 44);
        assert_eq!(&wav[0..4], b"RIFF");
    }
}
