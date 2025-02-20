use anyhow::Result;
use std::process::Command;

pub struct SoftwareCenter {
    package_manager: crate::system::package_manager::PackageManager,
}

impl SoftwareCenter {
    pub fn new() -> Self {
        Self {
            package_manager: crate::system::package_manager::PackageManager::new(),
        }
    }

    pub fn show(&self) -> Result<()> {
        println!("xBitOS Software Center (CLI Version)");
        println!("1. Search packages");
        println!("2. Install package");
        println!("3. Remove package");
        println!("4. Update system");
        println!("5. Exit");

        Ok(())
    }
} 