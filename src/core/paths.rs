/// Cross-platform path utilities for digiOS
use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    if cfg!(windows) {
        // On Windows, use E:\digiOS\etc\digios or current directory
        if std::path::Path::new("E:\\digiOS").exists() {
            PathBuf::from("E:\\digiOS\\etc\\digios")
        } else {
            PathBuf::from("etc\\digios")
        }
    } else {
        PathBuf::from("/etc/digios")
    }
}

pub fn get_data_dir() -> PathBuf {
    if cfg!(windows) {
        if std::path::Path::new("E:\\digiOS").exists() {
            PathBuf::from("E:\\digiOS\\var\\lib\\digios")
        } else {
            PathBuf::from("var\\lib\\digios")
        }
    } else {
        PathBuf::from("/var/lib/digios")
    }
}

pub fn get_setup_complete_path() -> PathBuf {
    get_config_dir().join("setup_complete")
}

pub fn get_model_config_path() -> PathBuf {
    get_config_dir().join("model.json")
}

pub fn get_models_dir() -> PathBuf {
    get_data_dir().join("models")
}

pub fn get_memory_dir() -> PathBuf {
    get_data_dir().join("memory")
}

