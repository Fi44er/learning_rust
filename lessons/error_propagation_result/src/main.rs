#![warn(clippy::all, clippy::pedantic)]

use std::{
    env::current_dir,
    fmt::format,
    fs::create_dir_all,
    io::{Error as IOError, ErrorKind},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //std::io::Result<()>
    // let current_path = current_dir();
    // let mut target_path = match current_path {
    //     Ok(path) => path,
    //     Err(e) => panic!("Failed to get current directory: {e:?}"),
    // };

    let mut target_path = get_current_path()?;

    target_path.push("?.sqws");

    create_dir_all(&target_path).expect("Failed to create target directory");

    // create_dir_all(&target_path)
    //     .unwrap_or_else(|e| panic!("Failed to create target directory: {e:?}"));

    // match create_dir_all(&target_path) {
    //     Ok(()) => print!("Created target directory: {target_path:?}"),
    //     Err(e) => match e.kind() {
    //         ErrorKind => {}
    //         unknown_e => panic!("Failed to create target directory: {unknown_e:?}"),
    //     },
    // }

    match create_dir_in(&target_path) {
        Ok(()) => print!("Created target directory"),
        Err(e) => panic!("Failed to create target directory: {e:?}"),
    }

    Ok(())
}

fn get_current_path() -> Result<PathBuf, IOError> {
    let path = current_dir()?;
    Ok(path)
}

// fn create_dir_in(target: &PathBuf) -> Result<String, IOError> {
//     match create_dir_all(target) {
//         Ok(_) => Ok(format!("{}", target.to_string_lossy())),
//         Err(e) => Err(e),
//     }
// }

fn create_dir_in(target: &PathBuf) -> Result<(), IOError> {
    create_dir_all(target)?;
    Ok(())
}
