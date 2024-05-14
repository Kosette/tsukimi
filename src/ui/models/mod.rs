use once_cell::sync::Lazy;
pub mod settings;
pub use self::settings::Settings;
pub static SETTINGS: Lazy<Settings> = Lazy::new(Settings::default);

pub static CACHE_PATH: Lazy<std::path::PathBuf> = Lazy::new(|| {
    let path = gtk::glib::user_cache_dir().join("tsukimi");
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create directory");
    }
    path
});

pub fn emby_cache_path() -> std::path::PathBuf {
    let path = CACHE_PATH.join(std::env::var("EMBY_NAME").unwrap());
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create directory");
    }
    path
}
