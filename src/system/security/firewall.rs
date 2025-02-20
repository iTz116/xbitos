use anyhow::Result;
use std::process::Command;

pub struct FirewallManager;

impl FirewallManager {
    pub fn new() -> Self {
        Self
    }

    pub fn setup(&self) -> Result<()> {
        // تثبيت وإعداد firewalld
        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&["firewalld"])?;

        // تكوين القواعد الأساسية
        Command::new("firewall-cmd")
            .args(["--permanent", "--add-service=ssh"])
            .status()?;

        Command::new("firewall-cmd")
            .args(["--permanent", "--add-service=dhcpv6-client"])
            .status()?;

        // تمكين وتشغيل الخدمة
        let service_manager = crate::system::services::ServiceManager::new();
        service_manager.enable_service("firewalld")?;
        service_manager.start_service("firewalld")?;

        Ok(())
    }
} 