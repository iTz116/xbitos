use anyhow::Result;
use log::{info, error};
use std::process::Command;

pub struct ServiceManager {
    init_system: InitSystem,
}

enum InitSystem {
    Systemd,
    // يمكن إضافة أنظمة init أخرى في المستقبل
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            init_system: InitSystem::Systemd,
        }
    }

    pub fn enable_service(&self, service: &str) -> Result<()> {
        match self.init_system {
            InitSystem::Systemd => {
                info!("Enabling service: {}", service);
                let status = Command::new("systemctl")
                    .args(["enable", service])
                    .status()?;

                if !status.success() {
                    return Err(anyhow::anyhow!("Failed to enable service: {}", service));
                }
            }
        }
        Ok(())
    }

    pub fn start_service(&self, service: &str) -> Result<()> {
        match self.init_system {
            InitSystem::Systemd => {
                info!("Starting service: {}", service);
                let status = Command::new("systemctl")
                    .args(["start", service])
                    .status()?;

                if !status.success() {
                    return Err(anyhow::anyhow!("Failed to start service: {}", service));
                }
            }
        }
        Ok(())
    }

    pub fn setup_essential_services(&self) -> Result<()> {
        let essential_services = vec![
            "NetworkManager",
            "bluetooth",
            "pipewire",
            "pipewire-pulse",
            "sddm",
        ];

        for service in essential_services {
            self.enable_service(service)?;
            self.start_service(service)?;
        }

        Ok(())
    }
} 