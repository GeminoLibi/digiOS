use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, error, warn};
use std::io::Write;
use reqwest;

#[derive(Clone)]
pub struct ModelDownloader;

impl ModelDownloader {
    pub fn new() -> Self {
        Self
    }

    pub async fn download(&self, url: &str, dest_dir: &PathBuf) -> Result<()> {
        info!("Downloading model from: {}", url);
        
        // Ensure destination directory exists
        std::fs::create_dir_all(dest_dir)?;
        
        // Check if this is a placeholder URL
        if url.contains("example.com") {
            info!("Placeholder URL detected - creating placeholder model file");
            
            // Extract model name from URL or use default
            let model_name = if url.contains("small") {
                "digios-default-small"
            } else if url.contains("large") {
                "digios-default-large"
            } else {
                "digios-default"
            };
            
            let model_path = dest_dir.join(model_name);
            
            // Create a placeholder file with metadata
            let placeholder_data = format!(
                "digiOS Model Placeholder\n\
                URL: {}\n\
                Created: {}\n\
                Status: Placeholder - Actual model download not yet implemented\n\
                \n\
                To use a real model:\n\
                1. Download a compatible model file\n\
                2. Place it at: {:?}\n\
                3. Update model.json with the correct path\n",
                url,
                // Timestamp placeholder - chrono not needed for placeholder
                "2025-11-09T00:00:00Z",
                model_path
            );
            
            std::fs::write(&model_path, placeholder_data)?;
            info!("Placeholder model file created at: {:?}", model_path);
            
            return Ok(());
        }
        
        // Try to download from real URL
        info!("Attempting to download from: {}", url);
        
        // Check if it's a Hugging Face URL
        if url.contains("huggingface.co") {
            return self.download_from_huggingface(url, dest_dir).await;
        }
        
        // Generic download
        self.download_generic(url, dest_dir).await
    }
    
    async fn download_from_huggingface(&self, url: &str, dest_dir: &PathBuf) -> Result<()> {
        info!("Downloading from Hugging Face...");
        
        // Hugging Face URLs are typically like:
        // https://huggingface.co/user/model-name
        // We need to find the actual file URL
        
        // For now, try to construct a direct download URL
        // This is a simplified approach - in production, you'd use the HF API
        warn!("Hugging Face download requires API integration");
        warn!("For now, please download manually from: {}", url);
        warn!("Or use Hugging Face CLI: huggingface-cli download <model>");
        
        Err(anyhow::anyhow!(
            "Hugging Face download requires manual setup.\n\
            Please either:\n\
            1. Install huggingface-cli and download manually\n\
            2. Download from the web interface: {}\n\
            3. Use a detected model from your system",
            url
        ))
    }
    
    async fn download_generic(&self, url: &str, dest_dir: &PathBuf) -> Result<()> {
        info!("Downloading from URL: {}", url);
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()?;
        
        let response = client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Download failed: HTTP {}", response.status()));
        }
        
        // Get filename from URL or Content-Disposition header
        let filename = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("model.bin")
            .to_string();
        
        let file_path = dest_dir.join(&filename);
        
        info!("Downloading to: {:?}", file_path);
        
        // Get content length for progress
        let total_size = response.content_length();
        
        let mut file = std::fs::File::create(&file_path)?;
        let mut stream = response.bytes_stream();
        let mut downloaded: u64 = 0;
        
        use futures::StreamExt;
        use std::io::Write as IoWrite;
        
        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;
            
            // Show progress
            if let Some(total) = total_size {
                let percent = (downloaded * 100) / total;
                print!("\rProgress: {}% ({}/{})", 
                    percent,
                    Self::format_bytes(downloaded),
                    Self::format_bytes(total)
                );
                std::io::stdout().flush()?;
            } else {
                print!("\rDownloaded: {}", Self::format_bytes(downloaded));
                std::io::stdout().flush()?;
            }
        }
        
        println!("\nDownload complete: {:?}", file_path);
        info!("Model downloaded successfully");
        
        Ok(())
    }
    
    fn format_bytes(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

