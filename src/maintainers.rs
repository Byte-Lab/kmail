use log::warn;
use std::fs;
use std::path::Path;

/// Get the get_maintainer.pl script from the repo, which can be invoked to get the list of
/// maintainers for the specified patchset.
fn get_maintainers_script(repo_root: &Path) -> fs::File {
    let scripts_path = repo_root.join("scripts").join("get_maintainer.pl");
    match fs::File::open(scripts_path.as_path()) {
        Ok(file) => file,
        Err(e) => panic!("{}: Maintainers path not found at {}", e, scripts_path.as_path().display())
    }
}

fn get_patchset_files(patch_path: &Path) -> Vec<fs::File> {
    let metadata = fs::metadata(patch_path).unwrap();
    let mut files = Vec::new();
    if metadata.is_dir() {
        for entry in fs::read_dir(patch_path).unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                warn!("Detected nested directory {}, skipping", entry.path().as_path().display());
                continue;
            }

            files.push(fs::File::open(entry.path()).unwrap());
        }
    } else {
        files.push(fs::File::open(patch_path).unwrap());
    }

    // If we weren't able to find any patch files, let's just fail hard.
    assert!(!files.is_empty());

    files
}

/// Get the list of maintainers (and lists) by invoking scripts/get_maintainters.pl on a
/// repository.
pub fn get_maintainers(patch_path: &Path, repo_root: &Path) -> Vec<String> {
    let _ = get_maintainers_script(repo_root);
    let patchsets = get_patchset_files(patch_path);

    let mut maintainers = Vec::new();
    for _ in patchsets {
       maintainers.push(String::from("Fake file"));
    }

    maintainers
}
