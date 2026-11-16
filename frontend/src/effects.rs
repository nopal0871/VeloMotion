use std::collections::HashMap;
use web_sys::CanvasRenderingContext2d;

/// Efek modular yang bisa dipasang ke clip dan dirender di canvas.
pub trait Effect {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn apply(
        &self,
        ctx: &CanvasRenderingContext2d,
        width: f64,
        height: f64,
        time: f64,
        params: &HashMap<String, f64>,
    );
}

fn intensity(params: &HashMap<String, f64>) -> f64 {
    params.get("intensity").copied().unwrap_or(1.0).clamp(0.0, 1.0)
}

pub struct FlashInvert;
impl Effect for FlashInvert {
    fn id(&self) -> &'static str {
        "flash_invert_v1"
    }
    fn name(&self) -> &'static str {
        "Flash Invert"
    }
    fn apply(&self, ctx: &CanvasRenderingContext2d, _w: f64, _h: f64, time: f64, params: &HashMap<String, f64>) {
        // Berkedip: invert aktif pada separuh siklus 0.5 detik
        if (time * 2.0).fract() < 0.5 {
            ctx.set_filter(&format!("invert({}%)", (intensity(params) * 100.0).round()));
        }
    }
}

pub struct Grayscale;
impl Effect for Grayscale {
    fn id(&self) -> &'static str {
        "grayscale_v1"
    }
    fn name(&self) -> &'static str {
        "Grayscale"
    }
    fn apply(&self, ctx: &CanvasRenderingContext2d, _w: f64, _h: f64, _t: f64, params: &HashMap<String, f64>) {
        ctx.set_filter(&format!("grayscale({}%)", (intensity(params) * 100.0).round()));
    }
}

pub struct Sepia;
impl Effect for Sepia {
    fn id(&self) -> &'static str {
        "sepia_v1"
    }
    fn name(&self) -> &'static str {
        "Sepia"
    }
    fn apply(&self, ctx: &CanvasRenderingContext2d, _w: f64, _h: f64, _t: f64, params: &HashMap<String, f64>) {
        ctx.set_filter(&format!("sepia({}%)", (intensity(params) * 100.0).round()));
    }
}

pub struct Blur;
impl Effect for Blur {
    fn id(&self) -> &'static str {
        "blur_v1"
    }
    fn name(&self) -> &'static str {
        "Blur"
    }
    fn apply(&self, ctx: &CanvasRenderingContext2d, _w: f64, _h: f64, _t: f64, params: &HashMap<String, f64>) {
        ctx.set_filter(&format!("blur({}px)", (intensity(params) * 8.0).round()));
    }
}

pub fn get_built_in_effects() -> Vec<Box<dyn Effect>> {
    vec![
        Box::new(FlashInvert),
        Box::new(Grayscale),
        Box::new(Sepia),
        Box::new(Blur),
    ]
}

/// Menjalankan efek bawaan berdasarkan id-nya. Id yang tidak dikenal diabaikan.
pub fn apply_effect_by_id(
    effect_id: &str,
    ctx: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    time: f64,
    params: &HashMap<String, f64>,
) {
    if let Some(effect) = get_built_in_effects().into_iter().find(|e| e.id() == effect_id) {
        effect.apply(ctx, width, height, time, params);
    }
}
