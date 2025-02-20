use anyhow::Result;
use log::{info, error};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct IsoBuilder {
    work_dir: PathBuf,
    output_dir: PathBuf,
    version: String,
}

impl IsoBuilder {
    pub fn new(version: &str) -> Self {
        Self {
            work_dir: PathBuf::from("/var/lib/xbitos/iso"),
            output_dir: PathBuf::from("/var/lib/xbitos/releases"),
            version: version.to_string(),
        }
    }

    pub fn build_iso(&self) -> Result<()> {
        info!("Building xBitOS ISO...");

        // إنشاء مجلدات العمل
        self.setup_directories()?;

        // إعداد ملفات التكوين
        self.setup_config_files()?;

        // نسخ الملفات الأساسية
        self.copy_base_files()?;

        // إعداد برنامج التثبيت
        self.setup_installer()?;

        // بناء الصورة
        self.create_iso()?;

        Ok(())
    }

    fn setup_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.work_dir)?;
        fs::create_dir_all(&self.output_dir)?;
        fs::create_dir_all(self.work_dir.join("airootfs"))?;
        fs::create_dir_all(self.work_dir.join("boot"))?;
        fs::create_dir_all(self.work_dir.join("efiboot"))?;
        Ok(())
    }

    fn setup_config_files(&self) -> Result<()> {
        let profiledef = format!(r#"
#!/usr/bin/env bash
iso_name="xbitos"
iso_label="XBITOS_{}"
iso_publisher="xBitOS <https://xbitos.org>"
iso_application="xBitOS Live/Rescue CD"
iso_version="{}"
install_dir="xbitos"
buildmodes=('iso')
bootmodes=('bios.syslinux.mbr' 'bios.syslinux.eltorito' 'uefi-x64.systemd-boot.esp' 'uefi-x64.systemd-boot.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
"#, self.version.replace(".", ""), self.version);

        fs::write(self.work_dir.join("profiledef.sh"), profiledef)?;
        
        // إضافة المزيد من ملفات التكوين...
        Ok(())
    }

    fn copy_base_files(&self) -> Result<()> {
        // نسخ النواة والملفات الأساسية
        Command::new("pacstrap")
            .args([
                self.work_dir.join("airootfs").to_str().unwrap(),
                "base",
                "linux-zen",
                "linux-firmware",
                "hyprland",
                "networkmanager",
                "xbitos-desktop",  // حزمة سطح المكتب الخاصة بنا
            ])
            .status()?;

        Ok(())
    }

    fn setup_installer(&self) -> Result<()> {
        // نسخ برنامج التثبيت
        fs::copy(
            "/usr/bin/xbitos-installer",
            self.work_dir.join("airootfs/usr/bin/xbitos-installer"),
        )?;

        // إعداد الإطلاق التلقائي
        let autostart = r#"
[Desktop Entry]
Type=Application
Name=Install xBitOS
Exec=xbitos-installer
Icon=system-software-install
"#;

        fs::write(
            self.work_dir.join("airootfs/etc/xdg/autostart/installer.desktop"),
            autostart,
        )?;

        Ok(())
    }

    fn create_iso(&self) -> Result<()> {
        let iso_name = format!("xbitos-{}-x86_64.iso", self.version);
        
        Command::new("mkarchiso")
            .args([
                "-v",
                "-w", "/tmp/archiso-tmp",
                "-o", self.output_dir.to_str().unwrap(),
                self.work_dir.to_str().unwrap(),
            ])
            .status()?;

        info!("ISO created successfully: {}", iso_name);
        Ok(())
    }

    fn setup_packages(&self) -> Result<()> {
        let essential_packages = vec![
            // حزم النظام الأساسية
            "base",
            "base-devel",
            "linux-zen",
            "linux-firmware",
            
            // بيئة سطح المكتب
            "hyprland",
            "waybar",
            "alacritty",
            "wofi",
            "dunst",
            "polkit-kde-agent",
            
            // الشبكة
            "networkmanager",
            "network-manager-applet",
            
            // الصوت
            "pipewire",
            "pipewire-pulse",
            
            // الأدوات الأساسية
            "firefox",
            "git",
            "nano",
            "vim",
            
            // السمات والخطوط
            "noto-fonts",
            "ttf-jetbrains-mono-nerd",
            "papirus-icon-theme",
            
            // أدوات النظام
            "gparted",
            "htop",
            "neofetch",
        ];

        Command::new("pacstrap")
            .args([
                self.work_dir.join("airootfs").to_str().unwrap(),
                &essential_packages.join(" "),
            ])
            .status()?;

        Ok(())
    }
} 