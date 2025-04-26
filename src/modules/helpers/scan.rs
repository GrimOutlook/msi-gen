#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use flexstr::{local_str, LocalStr};
use itertools::Itertools;
use log::{debug, error};

use crate::modules::{
    component::{directory::Directory, file::File},
    helpers::sequencer::Sequencer,
};

const DOT: LocalStr = local_str!(".");
const SOURCEDIR: LocalStr = local_str!("SourceDir");
const TARGETDIR: LocalStr = local_str!("TARGETDIR");
const PROGRAMFILESFOLDER: LocalStr = local_str!("ProgramFilesFolder");
const PROGRAMFILES64FOLDER: LocalStr = local_str!("ProgramFiles64Folder");

fn scan_path(
    scan_target: &Utf8PathBuf,
    sequencer: &mut Sequencer,
    parent_directory_id: &str,
) -> Result<(Vec<Directory>, Vec<File>)> {
    debug!("Scanning directory path [{}]", scan_target);
    // Get the entries present in the `scan_target` directory.
    let directory_entries = scan_target.read_dir_utf8().with_context(|| format!("Failed to read directory {scan_target}"))?;

    // Get all the entries that did not return an `Err` when scanned.
    let (ok_entries, errs): (Vec<_>, Vec<_>) =
        directory_entries.partition_result();
    // If any of them returned an error, short circuit and return that error.
    // May change this behavior based on config if desired in the future.
    let _ = errs.first().with_context(|| format!("Failed to read file inside {scan_target}"))?;
    
    // Get all the entries that have a valid filetype. We need to check if
    // these are directories so if we can't read that from somewhere we need to
    // exit.
    //
    // Also I shamelessly stole the [implementation] for
    // [`partition_result`](https://docs.rs/itertools/0.14.0/src/itertools/lib.rs.html#3669-3679)
    // in itertools to make this because I needed the entries back out, not the
    // filetypes that have to be checked.
    let mut ok_type_entries = Vec::new();
    for entry in ok_entries {
        let filetype = entry.file_type()?;
        ok_type_entries.push((entry, filetype));
    }

    // Convert all the entries that are directories into PathBufs for later use.
    let (mut found_dir_paths, mut found_file_paths) = (Vec::new(), Vec::new());
    for (entry, filetype) in ok_type_entries {
        // Only keep the entries that are either directories or files. We don't
        // care about symlinks or other filetypes.
        if filetype.is_dir() {
            found_dir_paths.push(entry.path().to_path_buf())
        } else if filetype.is_file() {
            found_file_paths.push(entry.path().to_path_buf())
        }
    }

    // Convert all the found directories found in the scan_path directory to
    // Directory objects. We need to generate a UUID for them and have those
    // available to pass into the recursive `scan_path` call so they know what
    // parent directory they are related to
    //
    // We only have to do this because we return Directory objects instead of
    // just PathBuf objects.
    //
    // TODO: Look into only returning PathBuf objects and
    // converting them outside of this function. I'm hesitant this will be much
    // cleaner because I feel like I'll just have to remake the structure
    // already present here but required more thought.
    let found_directories = found_dir_paths
        .iter()
        .map(|source| Directory::from_path(source, parent_directory_id))
        .collect_vec();

    // Recursively scan all the directories that were found in the
    // `scan_target` directory and return all the files and directories that
    // were found.
    //
    // There has to be a better way than making `errs` mutable and popping it
    // out but if I try to do `errs.first()`but that returns a reference and I
    // couldn't figure out how to get around the `cannot move out of `*err`
    // which is behind a shared reference`.
    let (mut all_dirs, mut all_files): (Vec<Directory>, Vec<File>) =
        (Default::default(), Default::default());
    let path_scan_results = found_directories
        .iter()
        .map(|dir| {
            all_dirs.push(dir.clone());
            scan_path(dir.source().as_ref().unwrap(), sequencer, dir.id())
        })
        .collect_vec();
    for paths in path_scan_results {
        match paths {
            Ok(paths) => {
                let (mut found_dirs, mut found_files) = paths;
                all_dirs.append(&mut found_dirs);
                all_files.append(&mut found_files);
            }
            // Short circuit if any errors were hit during the recursive scan.
            Err(err) => return Err(err),
        }
    }

    for file_path in found_file_paths {
        let size = match file_path.metadata() {
            Ok(metadata) => {
                #[cfg(target_os = "linux")]
                {
                    metadata.size()
                }
                #[cfg(target_os = "windows")]
                {
                    metadata.file_size()
                }
            }
            Err(err) => {
                error!("Couldn't get metadata from file [{}]", file_path);
                return Err(err.into());
            }
        };
        let file = File::new(&file_path, sequencer.get(), size);
        all_files.push(file);
    }

    Ok((all_dirs, all_files))
}
