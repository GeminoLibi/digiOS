use anyhow::Result;
use std::process::Command;
use tracing::{info, warn, error};

/// Python Ollama client - Uses Python ollama package as fallback
pub struct PythonOllamaClient {
    model_name: String,
}

impl PythonOllamaClient {
    pub fn new(model_name: String) -> Self {
        // Extract model name from "ollama:modelname" format
        let name = if model_name.starts_with("ollama:") {
            model_name.strip_prefix("ollama:").unwrap().to_string()
        } else {
            model_name
        };
        
        Self {
            model_name: name,
        }
    }

    /// Check if Python ollama is available
    pub async fn check_available(&self) -> Result<bool> {
        let python_check = Command::new("python")
            .arg("-c")
            .arg("import ollama; print('ok')")
            .output();
        
        match python_check {
            Ok(output) if output.status.success() => {
                info!("Python ollama client is available");
                Ok(true)
            }
            _ => {
                // Try python3
                let python3_check = Command::new("python3")
                    .arg("-c")
                    .arg("import ollama; print('ok')")
                    .output();
                
                match python3_check {
                    Ok(output) if output.status.success() => {
                        info!("Python3 ollama client is available");
                        Ok(true)
                    }
                    _ => {
                        warn!("Python ollama client not available");
                        Ok(false)
                    }
                }
            }
        }
    }

    /// Generate text using Python ollama client
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        info!("Calling Python ollama client for model '{}'", self.model_name);
        
        // Create Python script
        let script = format!(
            r#"
import ollama
import sys

try:
    response = ollama.generate(
        model='{}',
        prompt='''{}'''
    )
    print(response['response'])
except Exception as e:
    print(f"ERROR: {{e}}", file=sys.stderr)
    sys.exit(1)
"#,
            self.model_name.replace("'", "\\'"),
            prompt.replace("'", "\\'").replace("\n", "\\n")
        );
        
        // Try python first, then python3
        let mut cmd = Command::new("python");
        cmd.arg("-c").arg(&script);
        
        let output = match cmd.output() {
            Ok(o) => o,
            Err(_) => {
                // Try python3
                let mut cmd3 = Command::new("python3");
                cmd3.arg("-c").arg(&script);
                cmd3.output()?
            }
        };
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Python ollama error: {}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        info!("Python ollama response received ({} chars)", stdout.len());
        
        Ok(stdout.trim().to_string())
    }
}

