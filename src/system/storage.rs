use anyhow::Result;
use log::{info, error};
use std::process::Command;
use std::path::PathBuf;
use uuid::Uuid;

pub struct StorageManager {
    root_device: String,
    esp_path: PathBuf,
}

impl StorageManager {
    pub fn new(device: &str) -> Self {
        Self {
            root_device: device.to_string(),
            esp_path: PathBuf::from("/boot/efi"),
        }
    }

    pub fn setup_storage(&self) -> Result<()> {
        info!("Setting up storage system...");

        // تثبيت الأدوات المطلوبة
        let storage_packages = vec![
            "btrfs-progs",
            "cryptsetup",
            "lvm2",
            "dosfstools",
            "e2fsprogs",
            "snapper",
        ];

        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&storage_packages)?;

        // إعداد الأقسام
        self.create_partitions()?;
        
        // إعداد التشفير
        let encrypted_device = self.setup_encryption()?;
        
        // إعداد نظام الملفات
        self.setup_filesystems(&encrypted_device)?;

        // إعداد Snapper للنسخ الاحتياطية
        self.setup_snapper()?;

        Ok(())
    }

    fn create_partitions(&self) -> Result<()> {
        info!("Creating partitions...");

        // إنشاء جدول أقسام GPT
        Command::new("parted")
            .arg(&self.root_device)
            .args(["mklabel", "gpt"])
            .status()?;

        // إنشاء قسم EFI
        Command::new("parted")
            .arg(&self.root_device)
            .args(["mkpart", "ESP", "fat32", "1MiB", "513MiB"])
            .status()?;

        // إنشاء قسم التمهيد
        Command::new("parted")
            .arg(&self.root_device)
            .args(["mkpart", "boot", "513MiB", "1025MiB"])
            .status()?;

        // إنشاء قسم النظام
        Command::new("parted")
            .arg(&self.root_device)
            .args(["mkpart", "root", "1025MiB", "100%"])
            .status()?;

        Ok(())
    }

    fn setup_encryption(&self) -> Result<String> {
        info!("Setting up disk encryption...");

        let root_partition = format!("{}3", self.root_device);
        let encrypted_name = "cryptroot";

        // تهيئة القسم المشفر
        Command::new("cryptsetup")
            .args([
                "luksFormat",
                "--type", "luks2",
                "--cipher", "aes-xts-plain64",
                "--key-size", "512",
                "--hash", "sha512",
                "--iter-time", "5000",
                &root_partition,
            ])
            .status()?;

        // فتح القسم المشفر
        Command::new("cryptsetup")
            .args(["open", &root_partition, encrypted_name])
            .status()?;

        Ok(format!("/dev/mapper/{}", encrypted_name))
    }

    fn setup_filesystems(&self, encrypted_device: &str) -> Result<()> {
        info!("Setting up filesystems...");

        // تهيئة قسم EFI
        Command::new("mkfs.fat")
            .args(["-F32", &format!("{}1", self.root_device)])
            .status()?;

        // تهيئة قسم التمهيد
        Command::new("mkfs.ext4")
            .arg(&format!("{}2", self.root_device))
            .status()?;

        // تهيئة نظام ملفات BTRFS للنظام
        Command::new("mkfs.btrfs")
            .arg(encrypted_device)
            .status()?;

        // إنشاء أقسام فرعية BTRFS
        let mount_point = "/mnt";
        Command::new("mount")
            .args([encrypted_device, mount_point])
            .status()?;

        // إنشاء أقسام فرعية
        for subvol in &["@", "@home", "@snapshots", "@var", "@tmp"] {
            Command::new("btrfs")
                .args(["subvolume", "create", &format!("{}/{}", mount_point, subvol)])
                .status()?;
        }

        Ok(())
    }

    fn setup_snapper(&self) -> Result<()> {
        info!("Setting up Snapper backup system...");

        // تكوين Snapper للنظام الأساسي
        Command::new("snapper")
            .args(["create-config", "/"])
            .status()?;

        // تعديل تكوين النسخ الاحتياطية التلقائية
        let snapper_config = r#"
# تكوين Snapper
TIMELINE_MIN_AGE="1800"
TIMELINE_LIMIT_HOURLY="5"
TIMELINE_LIMIT_DAILY="7"
TIMELINE_LIMIT_WEEKLY="2"
TIMELINE_LIMIT_MONTHLY="2"
TIMELINE_LIMIT_YEARLY="0"

# النسخ الاحتياطية قبل التحديثات
ALLOW_USERS="root"
SYNC_ACL="yes"
"#;

        std::fs::write("/etc/snapper/configs/root", snapper_config)?;

        Ok(())
    }
} 