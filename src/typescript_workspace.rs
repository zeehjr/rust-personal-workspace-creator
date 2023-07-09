use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    process::Command,
};

use serde_json::Value;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum NodePackageManager {
    NPM,
    PNPM,
    Yarn,
}

pub struct TypescriptWorkspace {
    path: PathBuf,
    package_manager_bin: String,
}

impl TypescriptWorkspace {
    pub fn new(path: impl Into<PathBuf>, package_manager: NodePackageManager) -> Self {
        let package_manager_bin = match package_manager {
            NodePackageManager::NPM => "npm",
            NodePackageManager::PNPM => "pnpm",
            NodePackageManager::Yarn => "yarn",
        }
        .to_string();

        Self {
            path: path.into(),
            package_manager_bin,
        }
    }

    pub fn init_workspace(&self) -> io::Result<&Self> {
        println!(
            "Initializing {} workspace at {}...",
            self.package_manager_bin,
            self.path.display()
        );

        let mut args = vec!["init"];

        if self.package_manager_bin == "npm" || self.package_manager_bin == "yarn" {
            args.push("--yes");
        }

        Command::new(&self.package_manager_bin)
            .args(&args)
            .current_dir(&self.path)
            .output()?;

        println!(
            "Successfully initialized {} workspace!",
            self.package_manager_bin
        );

        Ok(self)
    }

    pub fn install_dev_dependencies(&self) -> io::Result<&Self> {
        println!(
            "Installing dev dependencies with {}...",
            self.package_manager_bin
        );

        Command::new(&self.package_manager_bin)
            .args(&[
                "add",
                "-D",
                "typescript",
                "@types/node",
                "jest",
                "ts-jest",
                "@types/jest",
                "tsx",
            ])
            .current_dir(&self.path)
            .output()?;

        println!(
            "Successfully installed dependencies with {}!",
            self.package_manager_bin
        );

        Ok(self)
    }

    pub fn jest_init(&self) -> io::Result<&Self> {
        println!("Initializing jest config...");

        Command::new(&self.package_manager_bin)
            .args(&["ts-jest", "config:init"])
            .current_dir(&self.path)
            .output()?;

        println!("Successfully initialized jest config!");

        Ok(self)
    }

    pub fn tsc_init(&self) -> io::Result<&Self> {
        println!("Initializing tsc config...");

        Command::new(&self.package_manager_bin)
            .args(&["tsc", "--init"])
            .current_dir(&self.path)
            .output()?;

        println!("Successfully initialized tsc config!");

        Ok(self)
    }

    pub fn add_scripts(&self) -> io::Result<&Self> {
        println!("Adding scripts to package.json...");

        let package_json_path = self.path.join("package.json");
        let mut package_json = File::open(&package_json_path)?;

        let mut contents = String::new();
        package_json.read_to_string(&mut contents)?;

        let mut data: Value = serde_json::from_str(&contents)?;

        if let Some(scripts) = data["scripts"].as_object_mut() {
            scripts.remove("test");

            scripts.insert(
                "dev".to_string(),
                Value::String("tsx watch src/index.ts".to_string()),
            );

            scripts.insert("test".to_string(), Value::String("jest".to_string()));
        }

        let mut file = File::create(&package_json_path)?;
        file.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;

        println!("Successfully added scripts to package.json!");

        Ok(self)
    }

    pub fn create_index_file(&self) -> io::Result<&Self> {
        println!("Creating index file...");

        let index_file_path = self.path.join("src/index.ts");

        fs::create_dir_all(index_file_path.parent().unwrap())?;

        let mut file = File::create(&index_file_path)?;
        file.write_all(b"console.log('Hello world!');\n")?;

        println!("Successfully created index file!");

        Ok(self)
    }
}
