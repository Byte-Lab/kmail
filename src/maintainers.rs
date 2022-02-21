use email_address::EmailAddress;
use log::warn;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get the get_maintainer.pl script from the repo, which can be invoked to get the list of
/// maintainers for the specified patchset.
fn get_maintainers_script(repo_root: &Path) -> PathBuf {
    let scripts_path = repo_root.join("scripts").join("get_maintainer.pl");
    let path_display = scripts_path.as_path().display();
    match scripts_path.as_path().exists() {
        true => log::info!("Using script {}", path_display),
        false => panic!("Maintainers script {} not found", path_display)
    };

    scripts_path
}

/// Get the MAINTAINERS file from the repo.
fn get_maintainers_file(repo_root: &Path) -> PathBuf {
    let maintainers_path = repo_root.join("MAINTAINERS");
    let maintainers_display = maintainers_path.as_path().display();
    match maintainers_path.as_path().exists() {
        true => log::info!("Using MAINTAINERS file {}", maintainers_display),
        false => panic!("Maintainers file {} not found", maintainers_display)
    };

    maintainers_path
}

fn get_patchset_files(patch_path: &Path) -> Vec<PathBuf> {
    let metadata = fs::metadata(patch_path).unwrap();
    let mut files = Vec::new();
    if metadata.is_dir() {
        for entry in fs::read_dir(patch_path).unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                warn!("Detected nested directory {}, skipping", entry.path().as_path().display());
                continue;
            }

            files.push(entry.path());
        }
    } else {
        files.push(patch_path.to_path_buf());
    }

    // If we weren't able to find any patch files, let's just fail hard.
    assert!(!files.is_empty());

    files
}

fn get_maintainers_for_file(maintainers_file: &Path, script: &Path, patch: &Path) -> Vec<String> {
    // Invoke get_maintainer.pl with the flags required to only return email addresses.
    let output = Command::new(script.to_str().unwrap())
                         .args(
                             ["--non",
                              "--noroles",
                              "--no-rolestats",
                              "--no-tree",
                              "--mpath",
                              maintainers_file.to_str().unwrap(),
                              patch.to_str().unwrap()])
                         .output()
                         .expect("Failed to execute maintainers script");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("Failed to parse stderr");
        panic!("({} | {}): Failed to invoke {} on {}", output.status, stderr, script.display(), patch.display());
    }

    let script_output = String::from_utf8(output.stdout).expect("Failed to read stdout");
    script_output
        .trim()
        .split("\n")
        .map(|address| {
            if !EmailAddress::is_valid(&address) {
                panic!("Email address {} returned from script was invalid", address);
            }
            address.to_string()})
        .collect()
}

/// Get the list of maintainers (and lists) by invoking scripts/get_maintainters.pl on a
/// repository.
pub fn get_maintainers(patch_path: &Path, repo_root: &Path) -> Vec<String> {
    let script = get_maintainers_script(repo_root);
    let maintainers_file = get_maintainers_file(repo_root);
    let patchsets = get_patchset_files(patch_path);

    let mut maintainers = Vec::new();
    for patch in patchsets {
        maintainers.extend(get_maintainers_for_file(&maintainers_file, &script, &patch));
    }

    maintainers.sort();
    maintainers.dedup();

    maintainers
}
