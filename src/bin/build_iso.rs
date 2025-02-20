use anyhow::{Result, Context};
use xbitos::system::iso_builder::IsoBuilder;
use env_logger;
use std::{env, process::Command};

fn main() -> Result<()> {
    // تهيئة نظام التسجيل
    env_logger::init();

    // التحقق من المتطلبات
    match check_requirements() {
        Ok(_) => {
            // إنشاء منشئ ISO
            let iso_builder = IsoBuilder::new("1.0.0");
            
            // بناء ISO
            iso_builder.build_iso().context("Failed to build ISO")?;
            
            println!("ISO file created successfully!");
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("\nWould you like to install the required packages? (y/n)");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if input.trim().to_lowercase() == "y" {
                install_requirements()?;
                println!("\nPlease run the command again to build the ISO.");
            }
        }
    }

    Ok(())
}

fn check_requirements() -> Result<()> {
    let required_tools = vec![
        "mkarchiso",
        "pacstrap",
        "arch-chroot",
    ];

    for tool in required_tools {
        if which::which(tool).is_err() {
            return Err(anyhow::anyhow!("Required tool not found: {}. Please install archiso package.", tool));
        }
    }

    Ok(())
}

fn install_requirements() -> Result<()> {
    println!("Installing required packages...");
    
    // تحديث قواعد البيانات
    Command::new("sudo")
        .args(["pacman", "-Sy"])
        .status()
        .context("Failed to update package database")?;

    // تثبيت الحزم المطلوبة
    Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm", "archiso", "arch-install-scripts"])
        .status()
        .context("Failed to install required packages")?;

    println!("Required packages installed successfully!");
    Ok(())
} 