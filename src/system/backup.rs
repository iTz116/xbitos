use anyhow::Result;
use chrono::Local;
use log::{info, error};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct BackupManager {
    backup_dir: PathBuf,
    config_path: PathBuf,
}

impl BackupManager {
    pub fn new() -> Self {
        Self {
            backup_dir: PathBuf::from("/var/lib/xbitos/backups"),
            config_path: PathBuf::from("/etc/xbitos/backup.conf"),
        }
    }

    pub fn setup(&self) -> Result<()> {
        // تثبيت أدوات النسخ الاحتياطي
        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&["rsync", "borg", "snapper"])?;

        // إنشاء المجلدات المطلوبة
        fs::create_dir_all(&self.backup_dir)?;

        // إعداد تكوين النسخ الاحتياطي
        self.setup_config()?;

        // إعداد المهام المجدولة
        self.setup_scheduled_backups()?;

        Ok(())
    }

    pub fn create_backup(&self) -> Result<()> {
        let date = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let backup_name = format!("backup_{}", date);

        // إنشاء نسخة احتياطية للنظام
        Command::new("borg")
            .args([
                "create",
                &format!("{}::{}", self.backup_dir.display(), backup_name),
                "/",
                "--exclude", "/proc",
                "--exclude", "/sys",
                "--exclude", "/tmp",
                "--exclude", "/run",
                "--exclude", "/mnt",
                "--exclude", "/media",
                "--exclude", "/lost+found",
            ])
            .status()?;

        Ok(())
    }

    pub fn restore_backup(&self, backup_name: &str) -> Result<()> {
        Command::new("borg")
            .args([
                "extract",
                &format!("{}::{}", self.backup_dir.display(), backup_name),
            ])
            .status()?;

        Ok(())
    }

    fn setup_config(&self) -> Result<()> {
        let config = r#"
# xBitOS Backup Configuration

# Backup Settings
BACKUP_RETENTION_DAYS=30
BACKUP_COMPRESSION=zstd
BACKUP_ENCRYPTION=repokey

# Schedule
BACKUP_SCHEDULE="daily"
BACKUP_TIME="03:00"

# Locations
BACKUP_PATHS="/etc /home /root /var/lib/xbitos"
BACKUP_EXCLUDE="/home/*/.cache/* /home/*/.local/share/Trash/*"

# Notifications
NOTIFY_ON_SUCCESS=true
NOTIFY_ON_FAILURE=true
"#;

        fs::write(&self.config_path, config)?;
        Ok(())
    }

    fn setup_scheduled_backups(&self) -> Result<()> {
        let service = r#"
[Unit]
Description=xBitOS Backup Service
After=network.target

[Service]
Type=oneshot
ExecStart=/usr/bin/xbitos-backup

[Install]
WantedBy=multi-user.target
"#;

        let timer = r#"
[Unit]
Description=Daily xBitOS Backup

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
"#;

        fs::write("/etc/systemd/system/xbitos-backup.service", service)?;
        fs::write("/etc/systemd/system/xbitos-backup.timer", timer)?;

        let service_manager = crate::system::services::ServiceManager::new();
        service_manager.enable_service("xbitos-backup.timer")?;

        Ok(())
    }
} 