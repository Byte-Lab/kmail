use std::path::Path;
use std::process::{Command, Stdio};

/// Send the specified patch to the given list of recipients.
pub fn send_patch(recipients: &[String], patch_path: &Path) {
    let mut command_args = vec!["send-email"];
    for recipient in recipients {
        log::debug!("to: {}", recipient);
        command_args.extend(["--to", recipient]);
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
        let stderr = String::from_utf8(output.stderr).expect("Failed to parse stderr");
        panic!("({} | {}): Failed to invoke git send-email {}", output.status, stderr, command_args.join(" "));
    }
}
