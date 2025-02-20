use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;

pub struct PowerManager {
    config_path: PathBuf,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            config_path: PathBuf::from("/etc/tlp.d"),
        }
    }

    pub fn setup_power_management(&self) -> Result<()> {
        info!("Setting up power management...");

        // تثبيت أدوات إدارة الطاقة
        let power_packages = vec![
            "tlp",
            "tlp-rdw",
            "powertop",
            "acpi",
            "acpi_call",
            "thermald",
        ];

        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&power_packages)?;

        // إعداد TLP
        self.setup_tlp_config()?;

        // تمكين وتشغيل خدمات إدارة الطاقة
        let service_manager = crate::system::services::ServiceManager::new();
        service_manager.enable_service("tlp")?;
        service_manager.enable_service("thermald")?;

        Ok(())
    }

    fn setup_tlp_config(&self) -> Result<()> {
        fs::create_dir_all(&self.config_path)?;

        let config_content = r#"
# CPU frequency scaling
CPU_SCALING_GOVERNOR_ON_AC=performance
CPU_SCALING_GOVERNOR_ON_BAT=powersave

# CPU energy performance preferences
CPU_ENERGY_PERF_POLICY_ON_AC=performance
CPU_ENERGY_PERF_POLICY_ON_BAT=power

# Disk devices
DISK_DEVICES="nvme0n1 sda"
DISK_IOSCHED="mq-deadline"

# Battery care
START_CHARGE_THRESH_BAT0=75
STOP_CHARGE_THRESH_BAT0=80

# Platform specific settings
PLATFORM_PROFILE_ON_AC=performance
PLATFORM_PROFILE_ON_BAT=low-power
"#;

        let config_file = self.config_path.join("01-custom.conf");
        fs::write(config_file, config_content)?;

        Ok(())
    }
} 