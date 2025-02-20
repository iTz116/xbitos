use anyhow::Result;
use log::{info, error};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

pub struct BootManager {
    esp_path: PathBuf,
}

impl BootManager {
    pub fn new() -> Self {
        Self {
            esp_path: PathBuf::from("/boot/efi"),
        }
    }

    pub fn setup_bootloader(&self) -> Result<()> {
        info!("Setting up bootloader...");

        // تثبيت برنامج الإقلاع والأدوات المطلوبة
        let bootloader_packages = vec![
            "systemd-boot",
            "efibootmgr",
            "efivar",
            "efitools",
        ];

        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&bootloader_packages)?;

        // تثبيت systemd-boot
        self.install_systemd_boot()?;

        // إعداد إدخالات الإقلاع
        self.configure_boot_entries()?;

        Ok(())
    }

    fn install_systemd_boot(&self) -> Result<()> {
        info!("Installing systemd-boot...");

        let status = Command::new("bootctl")
            .args(["install", "--path", self.esp_path.to_str().unwrap()])
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to install systemd-boot"));
        }

        Ok(())
    }

    fn configure_boot_entries(&self) -> Result<()> {
        let loader_conf = r#"
default  xbitos.conf
timeout  4
console-mode max
editor   no
"#;
        fs::write(self.esp_path.join("loader/loader.conf"), loader_conf)?;

        // الحصول على معلمات النواة
        let kernel_manager = crate::system::kernel::KernelManager::new();
        let kernel_params = kernel_manager.get_kernel_parameters();

        let entry_content = format!(r#"
title   xBitOS
linux   /vmlinuz-linux-zen
initrd  /amd-ucode.img
initrd  /intel-ucode.img
initrd  /initramfs-linux-zen.img
options {}
"#, kernel_params);

        fs::write(
            self.esp_path.join("loader/entries/xbitos.conf"),
            entry_content
        )?;

        Ok(())
    }

    pub fn update_boot_configuration(&self) -> Result<()> {
        info!("Updating boot configuration...");

        // تحديث تكوين برنامج الإقلاع
        let status = Command::new("bootctl")
            .arg("update")
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to update bootloader"));
        }

        Ok(())
    }
} 