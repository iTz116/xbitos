use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct ThemeManager {
    home_dir: PathBuf,
}

impl ThemeManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        
        Ok(Self {
            home_dir: home,
        })
    }

    pub fn setup_themes(&self) -> Result<()> {
        info!("Setting up system themes...");
        
        // تثبيت السمات الأساسية
        let themes = vec![
            "papirus-icon-theme",
            "breeze",
            "qt5ct",
            "kvantum",
        ];

        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&themes)?;

        // إعداد المتغيرات البيئية
        self.setup_environment_vars()?;
        
        // إعداد GTK theme
        self.setup_gtk_theme()?;

        Ok(())
    }

    fn setup_environment_vars(&self) -> Result<()> {
        let env_content = r#"
export QT_QPA_PLATFORMTHEME=qt5ct
export GTK_THEME=Breeze
export ICON_THEME=Papirus
export XCURSOR_THEME=breeze_cursors
"#;
        
        let env_file = PathBuf::from("/etc/environment.d/99-theming.conf");
        fs::write(env_file, env_content)?;

        Ok(())
    }

    fn setup_gtk_theme(&self) -> Result<()> {
        let gtk_settings = self.home_dir.join(".config/gtk-3.0/settings.ini");
        fs::create_dir_all(gtk_settings.parent().unwrap())?;
        
        let gtk_content = r#"[Settings]
gtk-theme-name=Breeze
gtk-icon-theme-name=Papirus
gtk-font-name=Noto Sans 10
gtk-cursor-theme-name=breeze_cursors
gtk-cursor-theme-size=24
"#;
        
        fs::write(gtk_settings, gtk_content)?;

        Ok(())
    }
} 