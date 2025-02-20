use anyhow::Result;
use log::{info, error};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PackageConfig {
    name: String,
    version: String,
    release: String,
    description: String,
    dependencies: Vec<String>,
    build_dependencies: Vec<String>,
    source: Vec<String>,
}

pub struct PackageBuilder {
    build_root: PathBuf,
    repo_path: PathBuf,
    aur_cache: PathBuf,
}

impl PackageBuilder {
    pub fn new() -> Self {
        Self {
            build_root: PathBuf::from("/var/lib/xbitos/build"),
            repo_path: PathBuf::from("/var/lib/xbitos/repo"),
            aur_cache: PathBuf::from("/var/cache/xbitos/aur"),
        }
    }

    pub fn build_package(&self, config: &PackageConfig) -> Result<()> {
        info!("Building package: {}", config.name);

        // إنشاء مجلد البناء
        let build_dir = self.build_root.join(&config.name);
        fs::create_dir_all(&build_dir)?;

        // إنشاء ملف PKGBUILD
        self.create_pkgbuild(&build_dir, config)?;

        // تثبيت اعتماديات البناء
        self.install_build_deps(config)?;

        // بناء الحزمة
        Command::new("makepkg")
            .args(["-sf", "--noconfirm"])
            .current_dir(&build_dir)
            .status()?;

        // إضافة الحزمة إلى المستودع
        self.add_to_repo(&build_dir)?;

        Ok(())
    }

    pub fn build_aur_package(&self, package_name: &str) -> Result<()> {
        info!("Building AUR package: {}", package_name);

        // استنساخ الحزمة من AUR
        let aur_dir = self.aur_cache.join(package_name);
        Command::new("git")
            .args([
                "clone",
                &format!("https://aur.archlinux.org/{}.git", package_name),
                aur_dir.to_str().unwrap(),
            ])
            .status()?;

        // بناء الحزمة
        Command::new("makepkg")
            .args(["-si", "--noconfirm"])
            .current_dir(&aur_dir)
            .status()?;

        Ok(())
    }

    fn create_pkgbuild(&self, build_dir: &PathBuf, config: &PackageConfig) -> Result<()> {
        let pkgbuild = format!(r#"
# Maintainer: xBitOS Team <team@xbitos.org>
pkgname={}
pkgver={}
pkgrel={}
pkgdesc="{}"
arch=('x86_64')
depends=({})
makedepends=({})
source=({})
sha256sums=('SKIP')

build() {{
    cd "$srcdir/$pkgname-$pkgver"
    ./configure --prefix=/usr
    make
}}

package() {{
    cd "$srcdir/$pkgname-$pkgver"
    make DESTDIR="$pkgdir/" install
}}
"#,
            config.name,
            config.version,
            config.release,
            config.description,
            config.dependencies.join(" "),
            config.build_dependencies.join(" "),
            config.source.join(" "),
        );

        fs::write(build_dir.join("PKGBUILD"), pkgbuild)?;
        Ok(())
    }

    fn install_build_deps(&self, config: &PackageConfig) -> Result<()> {
        let pkg_manager = crate::system::package_manager::PackageManager::new();
        pkg_manager.install_packages(&config.build_dependencies)?;
        Ok(())
    }

    fn add_to_repo(&self, build_dir: &PathBuf) -> Result<()> {
        // نقل الحزمة المبنية إلى المستودع
        let packages = fs::read_dir(build_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path()
                    .extension()
                    .map_or(false, |ext| ext == "pkg.tar.zst")
            });

        for package in packages {
            fs::copy(
                package.path(),
                self.repo_path.join(package.file_name()),
            )?;
        }

        // تحديث قاعدة بيانات المستودع
        Command::new("repo-add")
            .args([
                self.repo_path.join("xbitos.db.tar.gz").to_str().unwrap(),
                "*.pkg.tar.zst",
            ])
            .current_dir(&self.repo_path)
            .status()?;

        Ok(())
    }
} 