use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum AspectRatioPreset {
    Landscape16_9,
    Portrait9_16,
    Square1_1,
    Feed4_5,
    Custom(u32, u32),
}

impl fmt::Display for AspectRatioPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AspectRatioPreset::Landscape16_9 => write!(f, "16:9"),
            AspectRatioPreset::Portrait9_16 => write!(f, "9:16"),
            AspectRatioPreset::Square1_1 => write!(f, "1:1"),
            AspectRatioPreset::Feed4_5 => write!(f, "4:5"),
            AspectRatioPreset::Custom(w, h) => write!(f, "{}:{}", w, h),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Resolution {
    P720,
    P1080,
    P1440,
    K4,
}

impl Resolution {
    pub fn label(&self) -> &'static str {
        match self {
            Resolution::P720 => "720p",
            Resolution::P1080 => "1080p",
            Resolution::P1440 => "1440p",
            Resolution::K4 => "4K",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CanvasBackground {
    Black,
    White,
    Green,
    Blue,
}

impl CanvasBackground {
    pub fn hex(&self) -> &'static str {
        match self {
            CanvasBackground::Black => "#000000",
            CanvasBackground::White => "#FFFFFF",
            CanvasBackground::Green => "#00FF00",
            CanvasBackground::Blue => "#0000FF",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TrackType {
    Video,
    Audio,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClipType {
    Rectangle,
    Circle,
    Star,
    Text { content: String },
    SolidColor { color: String },
    Video { src: String },
    Photo { src: String },
    Audio { src: String, waveform: Vec<f32> },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppliedEffect {
    pub effect_id: String,
    pub enabled: bool,
    pub params: HashMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    pub name: String,
    pub start_time: f64,
    pub duration: f64,
    pub in_point: f64,
    pub out_point: f64,
    pub clip_type: ClipType,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub rotation: f64,
    pub opacity: f64,
    pub effects: Vec<AppliedEffect>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub track_type: TrackType,
    pub clips: Vec<Clip>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeatMarker {
    pub time: f64,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub aspect_ratio: AspectRatioPreset,
    pub resolution: Resolution,
    pub fps: u32,
    pub bg_color: CanvasBackground,
    pub created_at: String,
    pub last_opened: String,
    pub duration_used_seconds: u32,
    pub tracks: Vec<Track>,
    pub beat_markers: Vec<BeatMarker>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomEffectPlugin {
    pub id: String,
    pub name: String,
    pub code: String,
}
