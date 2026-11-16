/// Base path aplikasi.
///
/// Di GitHub Pages (project site) aplikasi dilayani di `/<nama-repo>/`, bukan di root.
/// Workflow deploy mengisi `VELO_BASE_PATH=/VeloMotion` saat build.
/// Saat development lokal (`trunk serve`) nilainya kosong, jadi app tetap jalan di `/`.
pub const BASE_PATH: &str = match option_env!("VELO_BASE_PATH") {
    Some(path) => path,
    None => "",
};
