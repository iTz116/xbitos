use anyhow::{Result, Context};
use log::{info, error};
use std::process::Command;

pub struct PackageManager {
    backend: PackageBackend,
}

enum PackageBackend {
    Pacman,
    // يمكن إضافة المزيد من مديري الحزم في المستقبل
}

impl PackageManager {
    pub fn new() -> Self {
        // حالياً نستخدم Pacman فقط
        Self {
            backend: PackageBackend::Pacman,
        }
    }

    pub fn install_packages(&self, packages: &[&str]) -> Result<()> {
        match self.backend {
            PackageBackend::Pacman => {
                for package in packages {
                    info!("Installing package: {}", package);
                    let status = Command::new("pacman")
                        .args(["-S", "--noconfirm", package])
                        .status()
                        .with_context(|| format!("Failed to install package: {}", package))?;

                    if !status.success() {
                        error!("Failed to install package: {}", package);
                        return Err(anyhow::anyhow!("Package installation failed: {}", package));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn update_system(&self) -> Result<()> {
        match self.backend {
            PackageBackend::Pacman => {
                info!("Updating system packages...");
                let status = Command::new("pacman")
                    .args(["-Syu", "--noconfirm"])
                    .status()
                    .context("Failed to update system")?;

                if !status.success() {
                    return Err(anyhow::anyhow!("System update failed"));
                }
            }
        }
        Ok(())
    }
} 