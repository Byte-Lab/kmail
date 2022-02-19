use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author = "David Vernet <void@manifault.com>",
        version = "1.0",
        about = "A utility for automatically sending emails to the correct maintainers \
                 for a kernel patchset.

Kmail invokes scripts/get_maintainer.pl on a patchset, parses the email
addresses (both the maintainers themselves and the relevant lists) to,
email, and sends them the patchset using git send-email.")]
struct Cli {
    /// The path to the patch (or a directory containing patches) to be mailed to the kernel
    /// upstream community using git send-email.
    patch_path: PathBuf,

    /// The path to a linux source repository containing a MAINTAINERS file. If no path is
    /// specified, the current directory is checked for a MAINTAINERS file.
    #[clap(short, long = "repo")]
    repo_path: Option<PathBuf>,
}

fn main() {
    let _ = Cli::parse();
}
