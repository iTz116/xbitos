use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct NetworkManager {
    config_path: PathBuf,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            config_path: PathBuf::from("/etc/NetworkManager/conf.d"),
        }
    }

    pub fn setup_networking(&self) -> Result<()> {
        info!("Setting up network management...");

        // تثبيت حزم الشبكة
        let network_packages = vec![
            "networkmanager",
            "network-manager-applet",
            "iwd",
            "dhcpcd",
            "openssh",
            "firewalld",
        ];

        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&network_packages)?;

        // إعداد NetworkManager
        self.setup_networkmanager_config()?;

        // تمكين وتشغيل خدمات الشبكة
        let service_manager = crate::system::services::ServiceManager::new();
        service_manager.enable_service("NetworkManager")?;
        service_manager.enable_service("iwd")?;
        service_manager.enable_service("firewalld")?;

        // إعداد جدار الحماية
        self.setup_firewall()?;

        Ok(())
    }

    fn setup_networkmanager_config(&self) -> Result<()> {
        fs::create_dir_all(&self.config_path)?;

        let config_content = r#"
[device]
wifi.backend=iwd

[connection]
wifi.powersave=2

[connectivity]
uri=http://networkcheck.gnome.org/
interval=300
"#;

        let config_file = self.config_path.join("00-custom.conf");
        fs::write(config_file, config_content)?;

        Ok(())
    }

    fn setup_firewall(&self) -> Result<()> {
        // تكوين قواعد جدار الحماية الأساسية
        Command::new("firewall-cmd")
            .args(["--permanent", "--add-service=ssh"])
            .status()?;

        Command::new("firewall-cmd")
            .args(["--permanent", "--add-service=dhcpv6-client"])
            .status()?;

        Command::new("firewall-cmd")
            .arg("--reload")
            .status()?;

        Ok(())
    }
} 