use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub platform: Vec<String>, // "windows", "linux", "macos"
    pub install_method: InstallMethod,
    pub check_command: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallMethod {
    Download { url: String, extract: bool },
    PackageManager { command: String },
    BuildFromSource { repo: String },
    SystemDefault, // Use system default if available
}

/// Tool Manager - Automatically detects, downloads, and installs needed tools
pub struct ToolManager {
    tools: HashMap<String, Tool>,
    installed: HashMap<String, PathBuf>,
}

impl ToolManager {
    pub fn new() -> Self {
        let mut manager = Self {
            tools: HashMap::new(),
            installed: HashMap::new(),
        };
        
        // Register common tools
        manager.register_default_tools();
        manager
    }

    fn register_default_tools(&mut self) {
        // Python (for compatibility layer)
        self.tools.insert("python".to_string(), Tool {
            name: "python".to_string(),
            description: "Python interpreter for running Python scripts".to_string(),
            platform: vec!["windows".to_string(), "linux".to_string(), "macos".to_string()],
            install_method: InstallMethod::SystemDefault,
            check_command: Some("python --version".to_string()),
            required: false,
        });

        // Node.js (for web tools)
        self.tools.insert("node".to_string(), Tool {
            name: "node".to_string(),
            description: "Node.js runtime".to_string(),
            platform: vec!["windows".to_string(), "linux".to_string(), "macos".to_string()],
            install_method: InstallMethod::SystemDefault,
            check_command: Some("node --version".to_string()),
            required: false,
        });

        // Git (for cloning repos)
        self.tools.insert("git".to_string(), Tool {
            name: "git".to_string(),
            description: "Git version control".to_string(),
            platform: vec!["windows".to_string(), "linux".to_string(), "macos".to_string()],
            install_method: InstallMethod::SystemDefault,
            check_command: Some("git --version".to_string()),
            required: false,
        });

        // Cargo (for Rust tools)
        self.tools.insert("cargo".to_string(), Tool {
            name: "cargo".to_string(),
            description: "Rust package manager".to_string(),
            platform: vec!["windows".to_string(), "linux".to_string(), "macos".to_string()],
            install_method: InstallMethod::SystemDefault,
            check_command: Some("cargo --version".to_string()),
            required: true, // We need this for self-improvement
        });
    }

    /// Check if a tool is available
    pub async fn check_tool(&self, name: &str) -> bool {
        if let Some(tool) = self.tools.get(name) {
            if let Some(ref cmd) = tool.check_command {
                let parts: Vec<&str> = cmd.split_whitespace().collect();
                if parts.is_empty() {
                    return false;
                }
                
                let output = Command::new(parts[0])
                    .args(&parts[1..])
                    .output();
                
                return output.is_ok();
            }
        }
        false
    }

    /// Automatically install a tool if needed
    pub async fn ensure_tool(&mut self, name: &str) -> Result<bool> {
        if self.check_tool(name).await {
            info!("Tool '{}' is already available", name);
            return Ok(true);
        }

        info!("Tool '{}' not found, attempting to install...", name);
        
        if let Some(tool) = self.tools.get(name) {
            match self.install_tool(tool).await {
                Ok(true) => {
                    info!("Successfully installed tool: {}", name);
                    Ok(true)
                }
                Ok(false) => {
                    warn!("Tool '{}' installation failed or not needed", name);
                    Ok(false)
                }
                Err(e) => {
                    error!("Failed to install tool '{}': {}", name, e);
                    Err(e)
                }
            }
        } else {
            warn!("Unknown tool: {}", name);
            Ok(false)
        }
    }

    async fn install_tool(&mut self, tool: &Tool) -> Result<bool> {
        let platform = std::env::consts::OS;
        
        if !tool.platform.iter().any(|p| p == platform) {
            warn!("Tool '{}' not available for platform: {}", tool.name, platform);
            return Ok(false);
        }

        match &tool.install_method {
            InstallMethod::SystemDefault => {
                // Try to find in system PATH
                info!("Checking system for: {}", tool.name);
                if self.check_tool(&tool.name).await {
                    return Ok(true);
                }
                
                // Try platform-specific package managers
                self.try_package_manager(&tool.name).await
            }
            InstallMethod::Download { url, extract } => {
                info!("Downloading tool from: {}", url);
                self.download_tool(url, extract).await
            }
            InstallMethod::PackageManager { command } => {
                info!("Installing via package manager: {}", command);
                self.run_package_command(command).await
            }
            InstallMethod::BuildFromSource { repo } => {
                info!("Building from source: {}", repo);
                self.build_from_source(repo).await
            }
        }
    }

    async fn try_package_manager(&self, tool: &str) -> Result<bool> {
        let platform = std::env::consts::OS;
        
        let commands = match platform {
            "windows" => vec![
                ("choco", format!("choco install {} -y", tool)),
                ("winget", format!("winget install {}", tool)),
                ("scoop", format!("scoop install {}", tool)),
            ],
            "linux" => vec![
                ("apt", format!("sudo apt-get install -y {}", tool)),
                ("yum", format!("sudo yum install -y {}", tool)),
                ("pacman", format!("sudo pacman -S --noconfirm {}", tool)),
            ],
            "macos" => vec![
                ("brew", format!("brew install {}", tool)),
            ],
            _ => vec![],
        };

        for (manager, cmd) in commands {
            if self.check_tool(manager).await {
                info!("Trying {} to install {}", manager, tool);
                if self.run_command(&cmd).await.is_ok() {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    async fn download_tool(&mut self, url: &str, extract: &bool) -> Result<bool> {
        // TODO: Implement actual download
        info!("Would download from: {}", url);
        Ok(false)
    }

    async fn run_package_command(&self, command: &str) -> Result<bool> {
        self.run_command(command).await
    }

    async fn build_from_source(&self, repo: &str) -> Result<bool> {
        // TODO: Clone, build, install
        info!("Would build from: {}", repo);
        Ok(false)
    }

    async fn run_command(&self, command: &str) -> Result<bool> {
        let platform = std::env::consts::OS;
        let shell = if platform == "windows" { "cmd" } else { "sh" };
        let flag = if platform == "windows" { "/c" } else { "-c" };
        
        let output = Command::new(shell)
            .arg(flag)
            .arg(command)
            .output()?;
        
        Ok(output.status.success())
    }

    /// Check and install all required tools
    pub async fn ensure_all_required(&mut self) -> Result<()> {
        info!("Checking and installing required tools...");
        
        for (name, tool) in &self.tools.clone() {
            if tool.required {
                self.ensure_tool(&name).await?;
            }
        }
        
        Ok(())
    }
}

