mod system;

use anyhow::Result;
use log::{info, error};
use crate::system::{
    package_manager::PackageManager,
    display::DisplayManager,
    services::ServiceManager,
    login::LoginManager,
    theming::ThemeManager,
    audio::AudioManager,
    power::PowerManager,
    network::NetworkManager,
    kernel::KernelManager,
    bootloader::BootManager,
    storage::StorageManager,
    updates::UpdateManager,
    distro::DistroManager,
};

async fn setup_system() -> Result<()> {
    // إعداد التوزيعة أولاً
    info!("Setting up xBitOS distribution...");
    let distro_manager = DistroManager::new();
    distro_manager.setup_distro()?;

    let pkg_manager = PackageManager::new();
    let display_manager = DisplayManager::new();
    let service_manager = ServiceManager::new();
    let login_manager = LoginManager::new();
    let theme_manager = ThemeManager::new()?;

    // تحديث النظام
    info!("Updating system...");
    pkg_manager.update_system()?;

    // تثبيت الحزم الأساسية
    info!("Installing base packages...");
    pkg_manager.install_packages(&[
        "base-devel",
        "hyprland",
        "waybar",
        "alacritty",
        "wofi",
        "light",
        "networkmanager",
        "pipewire",
        "xdg-desktop-portal-hyprland",
        "polkit",
        "gtk3",
        "gtk4",
        "qt5-wayland",
        "qt6-wayland",
        "sddm",
        "noto-fonts",
        "noto-fonts-emoji",
    ])?;

    // إعداد الخدمات
    info!("Setting up system services...");
    service_manager.setup_essential_services()?;

    // إعداد مدير تسجيل الدخول
    info!("Setting up login manager...");
    login_manager.setup_sddm()?;

    // إعداد Hyprland
    info!("Setting up display environment...");
    display_manager.setup_hyprland()?;

    // إعداد السمات
    info!("Setting up system themes...");
    theme_manager.setup_themes()?;

    // إعداد الصوت
    info!("Setting up audio system...");
    let audio_manager = AudioManager::new();
    audio_manager.setup_audio()?;

    // إعداد إدارة الطاقة
    info!("Setting up power management...");
    let power_manager = PowerManager::new();
    power_manager.setup_power_management()?;

    // إعداد الشبكة
    info!("Setting up networking...");
    let network_manager = NetworkManager::new();
    network_manager.setup_networking()?;

    // إعداد النواة
    info!("Setting up kernel...");
    let kernel_manager = KernelManager::new();
    kernel_manager.setup_kernel()?;

    // إعداد برنامج الإقلاع
    info!("Setting up bootloader...");
    let boot_manager = BootManager::new();
    boot_manager.setup_bootloader()?;

    // تحديث تكوين الإقلاع في النهاية
    boot_manager.update_boot_configuration()?;

    // إعداد التخزين
    info!("Setting up storage system...");
    let storage_manager = StorageManager::new("/dev/sda");
    storage_manager.setup_storage()?;

    // إعداد التحديثات التلقائية
    info!("Setting up automatic updates...");
    let update_manager = UpdateManager::new();
    update_manager.setup_auto_updates()?;

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    info!("Starting xBitOS setup...");
    
    if let Err(e) = setup_system().await {
        error!("Failed to setup system: {}", e);
        std::process::exit(1);
    }
    
    info!("System setup completed successfully!");
}
