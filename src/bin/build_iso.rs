use anyhow::{Result, Context};
use xbitos::system::iso_builder::IsoBuilder;
use env_logger;
use std::env;

fn main() -> Result<()> {
    // تهيئة نظام التسجيل
    env_logger::init();

    // التحقق من نظام التشغيل
    if cfg!(windows) {
        println!("Building ISO on Windows requires WSL2 with Arch Linux installed.");
        println!("Please run this command from within WSL2.");
        
        // التحقق من وجود WSL
        if let Ok(output) = std::process::Command::new("wsl")
            .args(["--list"])
            .output() 
        {
            if String::from_utf8_lossy(&output.stdout).contains("Arch") {
                println!("Arch Linux found in WSL. Run 'wsl -d Arch' first.");
            } else {
                println!("Please install Arch Linux in WSL2 first.");
            }
        } else {
            println!("WSL2 not found. Please install WSL2 first.");
        }
        
        return Ok(());
    }

    // التحقق من المتطلبات على Linux
    check_requirements().context("Failed to check requirements")?;

    // إنشاء منشئ ISO
    let iso_builder = IsoBuilder::new("1.0.0");
    
    // بناء ISO
    iso_builder.build_iso().context("Failed to build ISO")?;
    
    println!("ISO file created successfully!");
    Ok(())
}

fn check_requirements() -> Result<()> {
    if cfg!(unix) {
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
    }

    Ok(())
} 