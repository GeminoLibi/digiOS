use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, warn, error};
use std::process::Command;

/// Hugging Face model client - Uses transformers library via Python
pub struct HuggingFaceClient {
    model_path: PathBuf,
    model_name: String,
}

impl HuggingFaceClient {
    pub fn new(model_path: PathBuf) -> Self {
        let model_name = model_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("model")
            .to_string();
        
        Self {
            model_path,
            model_name,
        }
    }

    /// Check if transformers library is available
    pub async fn check_available(&self) -> Result<bool> {
        let python_check = Command::new("python")
            .arg("-c")
            .arg("import transformers; print('ok')")
            .output();
        
        match python_check {
            Ok(output) if output.status.success() => {
                info!("Python transformers library is available");
                Ok(true)
            }
            _ => {
                // Try python3
                let python3_check = Command::new("python3")
                    .arg("-c")
                    .arg("import transformers; print('ok')")
                    .output();
                
                match python3_check {
                    Ok(output) if output.status.success() => {
                        info!("Python3 transformers library is available");
                        Ok(true)
                    }
                    _ => {
                        warn!("Python transformers library not available");
                        Ok(false)
                    }
                }
            }
        }
    }

    /// Generate text using Hugging Face transformers
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        info!("Calling Hugging Face model: {:?}", self.model_path);
        
        // Create Python script to use transformers
        let script = format!(
            r#"
import sys
import os
from pathlib import Path

try:
    from transformers import AutoTokenizer, AutoModelForCausalLM
    import torch
    
    model_path = r"{}"
    
    # Try to load the model
    # For now, use a simple approach - in production, would cache loaded model
    print("Loading model from: " + model_path, file=sys.stderr)
    
    # This is a simplified version - full implementation would:
    # 1. Cache loaded models
    # 2. Use appropriate device (CPU/GPU)
    # 3. Handle different model types
    
    # For now, return a message that this needs proper setup
    print("Hugging Face model inference requires proper model loading setup.")
    print("Please use Ollama or LM Studio for now, or install transformers properly.")
    print("ERROR: Hugging Face inference not fully implemented", file=sys.stderr)
    sys.exit(1)
    
except ImportError as e:
    print(f"ERROR: transformers not installed: {{e}}", file=sys.stderr)
    sys.exit(1)
except Exception as e:
    print(f"ERROR: {{e}}", file=sys.stderr)
    sys.exit(1)
"#,
            self.model_path.to_string_lossy().replace("\\", "\\\\")
        );
        
        // Try python first, then python3
        let mut cmd = Command::new("python");
        cmd.arg("-c").arg(&script);
        
        let output = match cmd.output() {
            Ok(o) => o,
            Err(_) => {
                let mut cmd3 = Command::new("python3");
                cmd3.arg("-c").arg(&script);
                cmd3.output()?
            }
        };
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Hugging Face inference error: {}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().to_string())
    }
}

