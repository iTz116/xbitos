use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;

pub struct LoginManager {
    config_path: PathBuf,
}

impl LoginManager {
    pub fn new() -> Self {
        Self {
            config_path: PathBuf::from("/etc/sddm.conf.d"),
        }
    }

    pub fn setup_sddm(&self) -> Result<()> {
        info!("Setting up SDDM display manager...");
        
        fs::create_dir_all(&self.config_path)?;
        
        let config_content = r#"[Wayland]
SessionDir=/usr/share/wayland-sessions

[Theme]
Current=breeze
CursorTheme=breeze_cursors

[Users]
MaximumUid=60000
MinimumUid=1000

[General]
InputMethod=
Numlock=on
"#;
        
        let config_file = self.config_path.join("10-wayland.conf");
        fs::write(config_file, config_content)?;

        // إنشاء ملف جلسة Hyprland
        self.create_wayland_session()?;

        Ok(())
    }

    fn create_wayland_session(&self) -> Result<()> {
        let session_content = r#"[Desktop Entry]
Name=Hyprland
Comment=Highly customizable dynamic tiling Wayland compositor
Exec=/usr/local/bin/start-hyprland
Type=Application
"#;
        
        let session_dir = PathBuf::from("/usr/share/wayland-sessions");
        fs::create_dir_all(&session_dir)?;
        
        let session_file = session_dir.join("hyprland.desktop");
        fs::write(session_file, session_content)?;

        Ok(())
    }
} 