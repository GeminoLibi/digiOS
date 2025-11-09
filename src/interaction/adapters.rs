use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use tracing::{info, warn};

/// Compatibility Adapters - Allow digiOS to use Windows/Mac programs and files

pub struct CompatibilityAdapter {
    platform: String,
}

impl CompatibilityAdapter {
    pub fn new() -> Self {
        Self {
            platform: std::env::consts::OS.to_string(),
        }
    }

    /// Check if a program/file can be adapted for use
    pub async fn can_adapt(&self, path: &str) -> bool {
        let path_buf = PathBuf::from(path);
        
        // Check if it's an executable
        if path_buf.is_file() {
            // Check file extension
            if let Some(ext) = path_buf.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                match ext_str.as_str() {
                    "exe" | "msi" | "app" | "dmg" | "deb" | "rpm" => return true,
                    "py" | "js" | "sh" | "bat" | "ps1" => return true,
                    _ => {}
                }
            }
        }
        
        false
    }

    /// Adapt a Windows/Mac program for use
    pub async fn adapt_program(&self, path: &str) -> Result<AdaptedProgram> {
        info!("Adapting program: {}", path);
        
        let path_buf = PathBuf::from(path);
        let program_name = path_buf.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Determine how to run it
        let run_method = self.determine_run_method(&path_buf)?;
        
        Ok(AdaptedProgram {
            original_path: path.to_string(),
            adapted_name: program_name,
            run_method,
            platform: self.platform.clone(),
        })
    }

    fn determine_run_method(&self, path: &PathBuf) -> Result<RunMethod> {
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            
            match ext_str.as_str() {
                "exe" => Ok(RunMethod::DirectExecute),
                "msi" => Ok(RunMethod::InstallThenRun),
                "app" => Ok(RunMethod::MacAppBundle),
                "dmg" => Ok(RunMethod::MountThenRun),
                "py" => Ok(RunMethod::Interpreter {
                    command: "python".to_string(),
                }),
                "js" => Ok(RunMethod::Interpreter {
                    command: "node".to_string(),
                }),
                "sh" => Ok(RunMethod::Interpreter {
                    command: "sh".to_string(),
                }),
                "bat" | "cmd" => Ok(RunMethod::Interpreter {
                    command: "cmd".to_string(),
                }),
                "ps1" => Ok(RunMethod::Interpreter {
                    command: "powershell".to_string(),
                }),
                _ => Ok(RunMethod::TryExecute),
            }
        } else {
            Ok(RunMethod::TryExecute)
        }
    }

    /// Run an adapted program
    pub async fn run_adapted(&self, program: &AdaptedProgram, args: &[String]) -> Result<String> {
        info!("Running adapted program: {}", program.adapted_name);
        
        match &program.run_method {
            RunMethod::DirectExecute => {
                self.run_executable(&program.original_path, args).await
            }
            RunMethod::Interpreter { command } => {
                self.run_with_interpreter(command, &program.original_path, args).await
            }
            RunMethod::InstallThenRun => {
                // Install first, then find and run
                self.install_then_run(&program.original_path).await
            }
            RunMethod::MacAppBundle => {
                self.run_mac_app(&program.original_path).await
            }
            RunMethod::MountThenRun => {
                self.mount_and_run(&program.original_path).await
            }
            RunMethod::TryExecute => {
                self.run_executable(&program.original_path, args).await
            }
        }
    }

    async fn run_executable(&self, path: &str, args: &[String]) -> Result<String> {
        let output = Command::new(path)
            .args(args)
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn run_with_interpreter(&self, interpreter: &str, script: &str, args: &[String]) -> Result<String> {
        let mut cmd = Command::new(interpreter);
        cmd.arg(script);
        cmd.args(args);
        
        let output = cmd.output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn install_then_run(&self, installer: &str) -> Result<String> {
        // TODO: Implement installation logic
        warn!("Install-then-run not fully implemented for: {}", installer);
        Ok("Installation initiated".to_string())
    }

    async fn run_mac_app(&self, app_path: &str) -> Result<String> {
        // macOS .app bundles
        let executable = format!("{}/Contents/MacOS/{}", app_path, 
            PathBuf::from(app_path).file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("app"));
        
        self.run_executable(&executable, &[]).await
    }

    async fn mount_and_run(&self, dmg: &str) -> Result<String> {
        // TODO: Mount DMG, find app, run
        warn!("DMG mounting not implemented for: {}", dmg);
        Ok("DMG mounting not yet implemented".to_string())
    }

    /// Convert file formats for digiOS use
    pub async fn convert_file(&self, input: &str, output_format: &str) -> Result<String> {
        info!("Converting file: {} to format: {}", input, output_format);
        
        // TODO: Implement file conversion
        // - Convert documents
        // - Convert media files
        // - Extract data from proprietary formats
        
        Ok(format!("Converted {} to {}", input, output_format))
    }
}

#[derive(Debug, Clone)]
pub struct AdaptedProgram {
    pub original_path: String,
    pub adapted_name: String,
    pub run_method: RunMethod,
    pub platform: String,
}

#[derive(Debug, Clone)]
pub enum RunMethod {
    DirectExecute,
    Interpreter { command: String },
    InstallThenRun,
    MacAppBundle,
    MountThenRun,
    TryExecute,
}

