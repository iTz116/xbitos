use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;

pub struct AudioManager {
    config_path: PathBuf,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            config_path: PathBuf::from("/etc/pipewire"),
        }
    }

    pub fn setup_audio(&self) -> Result<()> {
        info!("Setting up audio system...");

        // تثبيت حزم الصوت الإضافية
        let audio_packages = vec![
            "pipewire-audio",
            "pipewire-alsa",
            "pipewire-jack",
            "wireplumber",
            "pavucontrol",
            "easyeffects",
        ];

        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&audio_packages)?;

        // إعداد تكوين PipeWire
        self.setup_pipewire_config()?;

        // تمكين وتشغيل خدمات الصوت
        let service_manager = crate::system::services::ServiceManager::new();
        service_manager.enable_service("pipewire")?;
        service_manager.enable_service("pipewire-pulse")?;
        service_manager.enable_service("wireplumber")?;

        Ok(())
    }

    fn setup_pipewire_config(&self) -> Result<()> {
        fs::create_dir_all(&self.config_path)?;

        let config_content = r#"
context.properties = {
    default.clock.rate = 48000
    default.clock.quantum = 1024
    default.clock.min-quantum = 32
    default.clock.max-quantum = 8192
}

context.modules = [
    { name = libpipewire-module-protocol-native }
    { name = libpipewire-module-profiler }
    { name = libpipewire-module-metadata }
    { name = libpipewire-module-spa-device-factory }
    { name = libpipewire-module-spa-node-factory }
    { name = libpipewire-module-client-node }
    { name = libpipewire-module-client-device }
    { name = libpipewire-module-portal }
    { name = libpipewire-module-access }
    { name = libpipewire-module-adapter }
    { name = libpipewire-module-link-factory }
    { name = libpipewire-module-session-manager }
]
"#;

        let config_file = self.config_path.join("pipewire.conf");
        fs::write(config_file, config_content)?;

        Ok(())
    }
} 