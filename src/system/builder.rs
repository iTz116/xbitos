use anyhow::Result;
use log::info;
use std::process::Command;
use std::path::PathBuf;
use std::fs;

pub struct SystemBuilder {
    build_path: PathBuf,
    packages_path: PathBuf,
}

impl SystemBuilder {
    pub fn new() -> Self {
        Self {
            build_path: PathBuf::from("/var/lib/xbitos/build"),
            packages_path: PathBuf::from("/var/lib/xbitos/packages"),
        }
    }

    pub fn build_base_system(&self) -> Result<()> {
        info!("Building base system...");

        // إنشاء مجلدات البناء
        fs::create_dir_all(&self.build_path)?;
        fs::create_dir_all(&self.packages_path)?;

        // تثبيت الأدوات الأساسية للبناء
        self.install_build_tools()?;

        // بناء النظام الأساسي
        self.build_core_packages()?;

        // إنشاء صورة النظام
        self.create_system_image()?;

        Ok(())
    }

    fn install_build_tools(&self) -> Result<()> {
        let build_tools = vec![
            "base-devel",
            "git",
            "archiso",
            "mkinitcpio-archiso",
            "squashfs-tools",
            "pacman-contrib",
        ];

        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&build_tools)?;

        Ok(())
    }

    fn build_core_packages(&self) -> Result<()> {
        // قائمة الحزم الأساسية للنظام
        let core_packages = r#"
# النظام الأساسي
base
base-devel
linux-zen
linux-zen-headers
linux-firmware

# أدوات النظام
sudo
systemd
systemd-sysvcompat
e2fsprogs
btrfs-progs
xfsprogs
dosfstools
networkmanager
wpa_supplicant

# الأدوات الأساسية
vim
nano
git
wget
curl
tar
gzip
unzip

# واجهة المستخدم
hyprland
waybar
alacritty
wofi
light
xdg-desktop-portal-hyprland
polkit
gtk3
gtk4
qt5-wayland
qt6-wayland
sddm

# الصوت
pipewire
pipewire-pulse
pipewire-alsa
pipewire-jack
wireplumber

# الشبكة
networkmanager
network-manager-applet
iwd
dhcpcd
openssh
firewalld

# الأدوات المساعدة
gparted
htop
neofetch
"#;

        fs::write(self.build_path.join("packages.txt"), core_packages)?;

        // إنشاء مستودع الحزم المحلي
        Command::new("repo-add")
            .args([
                self.packages_path.join("xbitos.db.tar.gz").to_str().unwrap(),
                self.packages_path.join("*.pkg.tar.zst").to_str().unwrap(),
            ])
            .status()?;

        Ok(())
    }

    fn create_system_image(&self) -> Result<()> {
        info!("Creating system image...");

        // إنشاء ملف التكوين للصورة
        let profile_content = r#"
#!/usr/bin/env bash

# إعدادات الصورة
iso_name="xbitos"
iso_label="XBITOS_$(date +%Y%m)"
iso_publisher="xBitOS <https://xbitos.org>"
iso_application="xBitOS Live/Rescue CD"
iso_version="$(date +%Y.%m.%d)"
install_dir="xbitos"
buildmodes=('iso')
bootmodes=('bios.syslinux.mbr' 'bios.syslinux.eltorito' 'uefi-x64.systemd-boot.esp' 'uefi-x64.systemd-boot.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')

# حزم إضافية للصورة الحية
packages=(
    'memtest86+'
    'mkinitcpio-nfs-utils'
    'nbd'
    'edk2-shell'
    'grub'
)

# نسخ الملفات الإضافية
file_permissions=(
    ["/etc/shadow"]="0:0:400"
    ["/etc/gshadow"]="0:0:400"
    ["/root"]="0:0:750"
    ["/root/.automated_script.sh"]="0:0:755"
    ["/usr/local/bin/choose-mirror"]="0:0:755"
    ["/usr/local/bin/Installation_guide"]="0:0:755"
)
"#;

        fs::write(self.build_path.join("profiledef.sh"), profile_content)?;

        // بناء الصورة
        Command::new("mkarchiso")
            .args(["-v", "-w", "/tmp/archiso-tmp", &self.build_path.to_string_lossy()])
            .status()?;

        Ok(())
    }
} 