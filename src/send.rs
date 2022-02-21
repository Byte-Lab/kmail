use std::path::Path;
use std::process::{Command, Stdio};

/// Send the specified patch to the given list of recipients.
pub fn send_patch(recipients: &[String], patch_path: &Path, extra_arguments: &[String]) {
    let mut command_args = vec!["send-email"];
    for recipient in recipients {
        command_args.extend(["--to", recipient]);
    }

    // Pass any extra arguments specified by the caller.
    for arg in extra_arguments {
        command_args.push(arg);
    }

    command_args.push(patch_path.to_str().unwrap());

    // Invoke git send-email.
    let output = Command::new("git")
                         .args(&command_args)
                         .stderr(Stdio::inherit())
                         .stdout(Stdio::inherit())
                         .stdin(Stdio::inherit())
                         .output()
                         .expect("Failed to execute git send-email");
    if !output.status.success() {
        panic!("{}: Failed to invoke git send-email", output.status);
    }
}
