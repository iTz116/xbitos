pub mod system_tests;
pub mod package_tests;
pub mod security_tests;

use anyhow::Result;
use log::{info, error};
use std::process::Command;

pub struct TestRunner {
    test_dir: std::path::PathBuf,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            test_dir: std::path::PathBuf::from("/var/lib/xbitos/tests"),
        }
    }

    pub fn run_all_tests(&self) -> Result<()> {
        info!("Running system tests...");
        self.run_system_tests()?;

        info!("Running package tests...");
        self.run_package_tests()?;

        info!("Running security tests...");
        self.run_security_tests()?;

        Ok(())
    }

    fn run_system_tests(&self) -> Result<()> {
        system_tests::run_tests()?;
        Ok(())
    }

    fn run_package_tests(&self) -> Result<()> {
        package_tests::run_tests()?;
        Ok(())
    }

    fn run_security_tests(&self) -> Result<()> {
        security_tests::run_tests()?;
        Ok(())
    }
} 