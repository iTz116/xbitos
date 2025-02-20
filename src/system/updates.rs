use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;

pub struct UpdateManager {
    config_path: PathBuf,
}

impl UpdateManager {
    pub fn new() -> Self {
        Self {
            config_path: PathBuf::from("/etc/systemd/system"),
        }
    }

    pub fn setup_auto_updates(&self) -> Result<()> {
        info!("Setting up automatic updates...");

        // إنشاء خدمة التحديث التلقائي
        self.create_update_service()?;
        
        // إنشاء مؤقت التحديث
        self.create_update_timer()?;

        // تمكين التحديث التلقائي
        let service_manager = crate::system::services::ServiceManager::new();
        service_manager.enable_service("xbitos-update.timer")?;

        Ok(())
    }

    fn create_update_service(&self) -> Result<()> {
        let service_content = r#"
[Unit]
Description=xBitOS System Update Service
After=network-online.target
Wants=network-online.target

[Service]
Type=oneshot
ExecStart=/usr/bin/pacman -Syu --noconfirm
ExecStartPost=/usr/bin/snapper create -c timeline -d "Auto Update"

[Install]
WantedBy=multi-user.target
"#;

        fs::write(
            self.config_path.join("xbitos-update.service"),
            service_content
        )?;

        Ok(())
    }

    fn create_update_timer(&self) -> Result<()> {
        let timer_content = r#"
[Unit]
Description=xBitOS Daily System Update Timer

[Timer]
OnCalendar=daily
RandomizedDelaySec=1h
Persistent=true

[Install]
WantedBy=timers.target
"#;

        fs::write(
            self.config_path.join("xbitos-update.timer"),
            timer_content
        )?;

        Ok(())
    }
} 