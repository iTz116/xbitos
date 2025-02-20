use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct InstallConfig {
    hostname: String,
    username: String,
    password: String,
    timezone: String,
    locale: String,
    keyboard: String,
    disk: String,
    use_encryption: bool,
    desktop_environment: String,
}

pub struct SystemInstaller {
    config: InstallConfig,
    mount_point: PathBuf,
}

impl SystemInstaller {
    pub fn new(config: InstallConfig) -> Self {
        Self {
            config,
            mount_point: PathBuf::from("/mnt"),
        }
    }

    pub fn install_system(&self) -> Result<()> {
        info!("Starting system installation...");

        // إعداد التخزين
        self.prepare_storage()?;

        // تثبيت النظام الأساسي
        self.install_base_system()?;

        // تكوين النظام
        self.configure_system()?;

        // تثبيت واجهة المستخدم
        self.install_desktop()?;

        // إعداد المستخدم
        self.setup_user()?;

        // إعداد برنامج الإقلاع
        self.setup_bootloader()?;

        Ok(())
    }

    fn prepare_storage(&self) -> Result<()> {
        let storage_manager = crate::system::storage::StorageManager::new(&self.config.disk);
        
        if self.config.use_encryption {
            storage_manager.setup_encrypted_storage()?;
        } else {
            storage_manager.setup_storage()?;
        }

        Ok(())
    }

    fn install_base_system(&self) -> Result<()> {
        info!("Installing base system...");

        // تثبيت النظام الأساسي باستخدام pacstrap
        Command::new("pacstrap")
            .args([
                &self.mount_point.to_string_lossy(),
                "base",
                "base-devel",
                "linux-zen",
                "linux-zen-headers",
                "linux-firmware",
            ])
            .status()?;

        // إنشاء fstab
        Command::new("genfstab")
            .args(["-U", &self.mount_point.to_string_lossy()])
            .output()?;

        Ok(())
    }

    fn configure_system(&self) -> Result<()> {
        info!("Configuring system...");

        // إعداد المنطقة الزمنية
        self.setup_timezone()?;

        // إعداد اللغة
        let locale_gen = format!("{} UTF-8", self.config.locale);
        fs::write(
            self.mount_point.join("etc/locale.gen"),
            locale_gen,
        )?;

        // إعداد اسم الجهاز
        fs::write(
            self.mount_point.join("etc/hostname"),
            &self.config.hostname,
        )?;

        // تنفيذ الأوامر داخل chroot
        self.chroot_execute(&[
            "locale-gen",
            "hwclock --systohc",
            &format!("echo LANG={}.UTF-8 > /etc/locale.conf", self.config.locale),
            &format!("echo KEYMAP={} > /etc/vconsole.conf", self.config.keyboard),
        ])?;

        Ok(())
    }

    fn install_desktop(&self) -> Result<()> {
        info!("Installing desktop environment...");

        match self.config.desktop_environment.as_str() {
            "hyprland" => {
                let display_manager = crate::system::display::DisplayManager::new();
                display_manager.setup_hyprland()?;
            },
            // يمكن إضافة دعم لبيئات سطح مكتب أخرى
            _ => return Err(anyhow::anyhow!("Unsupported desktop environment")),
        }

        Ok(())
    }

    fn setup_user(&self) -> Result<()> {
        info!("Setting up user account...");

        // إنشاء المستخدم
        self.chroot_execute(&[
            &format!("useradd -m -G wheel -s /bin/bash {}", self.config.username),
            &format!("echo '{}:{}' | chpasswd", self.config.username, self.config.password),
            "echo '%wheel ALL=(ALL) ALL' > /etc/sudoers.d/wheel",
        ])?;

        Ok(())
    }

    fn setup_bootloader(&self) -> Result<()> {
        info!("Setting up bootloader...");

        let boot_manager = crate::system::bootloader::BootManager::new();
        boot_manager.setup_bootloader()?;

        Ok(())
    }

    fn chroot_execute(&self, commands: &[&str]) -> Result<()> {
        for cmd in commands {
            Command::new("arch-chroot")
                .args([&self.mount_point.to_string_lossy(), "sh", "-c", cmd])
                .status()?;
        }
        Ok(())
    }

    #[cfg(unix)]
    fn setup_timezone(&self) -> Result<()> {
        std::os::unix::fs::symlink(
            format!("/usr/share/zoneinfo/{}", self.config.timezone),
            self.mount_point.join("etc/localtime"),
        )?;
        Ok(())
    }

    #[cfg(not(unix))]
    fn setup_timezone(&self) -> Result<()> {
        // على Windows نستخدم نسخ الملف بدلاً من الرابط الرمزي
        std::fs::copy(
            format!("/usr/share/zoneinfo/{}", self.config.timezone),
            self.mount_point.join("etc/localtime"),
        )?;
        Ok(())
    }
} 