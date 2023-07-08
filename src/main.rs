mod typescript_workspace;

use std::{fs, io, path::Path};

use typescript_workspace::TypescriptWorkspace;

fn get_workspace_name() -> String {
    std::env::args()
        .nth(1)
        .expect("Workspace name argument is missing")
}

fn get_workdir() -> String {
    std::env::current_dir()
        .expect("Failed to retrieve current working directory")
        .to_str()
        .expect("Failed to convert current working directory path to string")
        .to_string()
}

fn is_absolute_path(path: &str) -> bool {
    Path::new(path).is_absolute()
}

fn main() -> io::Result<()> {
    let workspace_name = get_workspace_name();
    let workdir = get_workdir();

    let path = if is_absolute_path(&workspace_name) {
        workspace_name
    } else {
        let base_path = Path::new(&workdir);
        base_path
            .join(&workspace_name)
            .to_string_lossy()
            .to_string()
    };

    if Path::new(&path).is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Workspace already exists",
        ));
    }

    fs::create_dir_all(&path)?;

    TypescriptWorkspace::new(path, typescript_workspace::NodePackageManager::NPM)
        .init_workspace()?
        .install_dev_dependencies()?
        .jest_init()?
        .tsc_init()?
        .add_scripts()?
        .create_index_file()?;

    Ok(())
}
