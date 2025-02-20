use anyhow::Result;
use log::info;
use std::path::PathBuf;
use std::fs;

pub struct DistroManager {
    config_path: PathBuf,
    repo_path: PathBuf,
}

impl DistroManager {
    pub fn new() -> Self {
        Self {
            config_path: PathBuf::from("/etc/xbitos"),
            repo_path: PathBuf::from("/var/lib/xbitos/repo"),
        }
    }

    pub fn setup_distro(&self) -> Result<()> {
        info!("Setting up xBitOS distribution...");

        // إنشاء المجلدات الأساسية
        self.create_directories()?;

        // إعداد مستودع الحزم
        self.setup_repository()?;

        // إعداد ملفات التكوين
        self.setup_configuration()?;

        // إعداد نظام البناء
        let builder = crate::system::builder::SystemBuilder::new();
        builder.build_base_system()?;

        Ok(())
    }

    fn create_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.config_path)?;
        fs::create_dir_all(&self.repo_path)?;
        fs::create_dir_all("/etc/xbitos/hooks")?;
        fs::create_dir_all("/var/lib/xbitos/cache")?;
        fs::create_dir_all("/var/log/xbitos")?;

        Ok(())
    }

    fn setup_repository(&self) -> Result<()> {
        // إعداد ملف تكوين pacman للمستودع المحلي
        let repo_conf = r#"
[options]
HoldPkg     = pacman glibc
Architecture = auto

CheckSpace
Color
ILoveCandy
ParallelDownloads = 5

SigLevel    = Required DatabaseOptional
LocalFileSigLevel = Optional

[xbitos]
SigLevel = Optional TrustAll
Server = file:///var/lib/xbitos/repo

[core]
Include = /etc/pacman.d/mirrorlist

[extra]
Include = /etc/pacman.d/mirrorlist

[community]
Include = /etc/pacman.d/mirrorlist
"#;

        fs::write("/etc/pacman.conf", repo_conf)?;

        Ok(())
    }

    fn setup_configuration(&self) -> Result<()> {
        // إعداد ملف تكوين التوزيعة
        let distro_conf = r#"
# xBitOS Configuration

# System
DISTRO_NAME="xBitOS"
DISTRO_VERSION="1.0.0"
DISTRO_CODENAME="Genesis"
DISTRO_DESCRIPTION="A modern Arch-based Linux distribution with Hyprland"

# Repository
REPO_NAME="xbitos"
REPO_URL="https://repo.xbitos.org"

# Updates
UPDATE_INTERVAL="daily"
BACKUP_ENABLED="yes"
BACKUP_KEEP_DAYS="30"

# Desktop
DEFAULT_SESSION="hyprland"
DEFAULT_THEME="breeze"
DEFAULT_ICON_THEME="papirus"
"#;

        fs::write(self.config_path.join("xbitos.conf"), distro_conf)?;

        Ok(())
    }
} 