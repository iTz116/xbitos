pub mod firewall;
pub mod apparmor;
pub mod package_verifier;

use anyhow::Result;
use log::{info, error};

pub struct SecurityManager {
    firewall: firewall::FirewallManager,
    apparmor: apparmor::AppArmorManager,
    package_verifier: package_verifier::PackageVerifier,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            firewall: firewall::FirewallManager::new(),
            apparmor: apparmor::AppArmorManager::new(),
            package_verifier: package_verifier::PackageVerifier::new(),
        }
    }

    pub fn setup_security(&self) -> Result<()> {
        info!("Setting up system security...");

        // إعداد جدار الحماية
        self.firewall.setup()?;

        // إعداد AppArmor
        self.apparmor.setup()?;

        // إعداد التحقق من الحزم
        self.package_verifier.setup()?;

        Ok(())
    }
} 