use anyhow::Result;
use log::{info, error};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct SoftwarePackage {
    name: String,
    description: String,
    version: String,
    category: String,
    dependencies: Vec<String>,
    optional_deps: Vec<String>,
    size: u64,
    installed: bool,
}

pub struct SoftwareCenter {
    db_path: PathBuf,
    cache_path: PathBuf,
    packages: HashMap<String, SoftwarePackage>,
}

impl SoftwareCenter {
    pub fn new() -> Result<Self> {
        let instance = Self {
            db_path: PathBuf::from("/var/lib/xbitos/software"),
            cache_path: PathBuf::from("/var/cache/xbitos/packages"),
            packages: HashMap::new(),
        };

        instance.initialize()?;
        Ok(instance)
    }

    fn initialize(&self) -> Result<()> {
        fs::create_dir_all(&self.db_path)?;
        fs::create_dir_all(&self.cache_path)?;
        
        // تحديث قاعدة البيانات
        self.update_database()?;

        Ok(())
    }

    pub fn update_database(&self) -> Result<()> {
        info!("Updating software database...");

        // تحديث مستودعات pacman
        Command::new("pacman")
            .args(["-Sy"])
            .status()?;

        // تحديث قاعدة بيانات البرامج المحلية
        self.sync_local_database()?;

        Ok(())
    }

    pub fn install_package(&self, package_name: &str) -> Result<()> {
        info!("Installing package: {}", package_name);

        // التحقق من وجود الحزمة
        if !self.packages.contains_key(package_name) {
            return Err(anyhow::anyhow!("Package not found"));
        }

        // تثبيت الحزمة
        Command::new("pacman")
            .args(["-S", "--noconfirm", package_name])
            .status()?;

        // تحديث قاعدة البيانات المحلية
        self.sync_local_database()?;

        Ok(())
    }

    pub fn remove_package(&self, package_name: &str) -> Result<()> {
        info!("Removing package: {}", package_name);

        Command::new("pacman")
            .args(["-R", "--noconfirm", package_name])
            .status()?;

        self.sync_local_database()?;

        Ok(())
    }

    pub fn search_packages(&self, query: &str) -> Vec<&SoftwarePackage> {
        self.packages
            .values()
            .filter(|pkg| {
                pkg.name.contains(query) || 
                pkg.description.contains(query)
            })
            .collect()
    }

    pub fn get_package_info(&self, package_name: &str) -> Option<&SoftwarePackage> {
        self.packages.get(package_name)
    }

    fn sync_local_database(&self) -> Result<()> {
        // تحديث قائمة الحزم المثبتة
        let installed = Command::new("pacman")
            .args(["-Q"])
            .output()?;

        let installed_packages = String::from_utf8(installed.stdout)?;

        // تحديث معلومات الحزم
        for line in installed_packages.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0];
                let version = parts[1];

                if let Some(pkg) = self.packages.get_mut(name) {
                    pkg.installed = true;
                    pkg.version = version.to_string();
                }
            }
        }

        Ok(())
    }
} 