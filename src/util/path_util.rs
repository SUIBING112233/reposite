use std::{
    fs::FileType,
    path::{Path, PathBuf},
};

use rocket::serde::Serialize;
use walkdir::WalkDir;

use crate::constants::ENV_REPOSITE_DIR;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FileItem {
    full_path: String,
    item_type: ItemType,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum ItemType {
    File,
    Directory,
}

/// Path warden can be used to check if the request path illegals.
///
/// If the request path is legal, it will return the real path in server.
///
/// example, request argument is `test_pool/tech/icedlab/advagri/advagri.jar`, the working dir is `/home/iced/reposite`
///
/// Then it will return `/home/iced/reposite/pool/test_pool/tech/icedlab/advagri/advagri.jar`
pub fn path_warden(pool_name: String, path: PathBuf) -> Option<PathBuf> {
    // Don't allow absolute path query
    if path.is_absolute() {
        return None;
    }

    let x = String::from(
        PathBuf::from(get_working_path())
            .join(pool_name)
            .join(path)
            .to_str()
            .unwrap(),
    );

    if x.contains("../") || x.contains("%2e%2e/") || x.contains("%2e%2e%2f") {
        return None;
    }

    Some(PathBuf::from(x))
}

pub fn walk_path(path: PathBuf) -> Vec<FileItem> {
    let mut l = vec![];
    let x = WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .follow_links(false)
        .sort_by_file_name();

    for entry in x {
        match entry {
            Ok(d) => l.push(FileItem {
                full_path: d.path().to_str().unwrap().to_string(),
                item_type: {
                    if d.path().is_dir() {
                        ItemType::Directory
                    } else {
                        ItemType::File
                    }
                },
            }),
            Err(_) => {}
        }
    }

    l
}

/// Return the working directory
///
/// If you set env var `REPOSITE_DIR` is `/home/iced/reposite`, it will return `/home/iced/reposite`
///
/// If you don't set env var `REPOSITE_DIR`, and current working path is `/home/iced`, it will return `/home/iced/reposite`
///
/// You can set env var `REPOSITE_DIR` to change the working path. The default working path is current directory.
pub fn get_working_path() -> PathBuf {
    let x = PathBuf::new();
    match std::env::var_os(ENV_REPOSITE_DIR) {
        Some(p) => {
            // is absolute
            if PathBuf::from(p.clone()).is_absolute() {
                return PathBuf::from(p.clone());
            }
            // If it isn't absolute,
            // convert it into absolute
            x.join(std::env::current_dir().unwrap().join(p))
        }
        None => x.join(std::env::current_dir().unwrap().join("reposite")),
    }
}
