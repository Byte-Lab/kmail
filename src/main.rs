mod maintainers;
mod send;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author = "David Vernet <void@manifault.com>",
        version = "1.0",
        about = "A utility for automatically sending emails to the correct maintainers \
                 for a kernel patchset.

Kmail invokes scripts/get_maintainer.pl on a patchset, parses the email
addresses (both the maintainers themselves and the relevant lists) to
email, and sends them the patchset using git send-email.")]
struct Cli {
    /// The path to the patch (or a directory containing patches) to be mailed to the kernel
    /// upstream community using git send-email.
    patch_path: PathBuf,

    /// The path to a linux source tree containing a MAINTAINERS file, and a
    /// scripts/get_maintainer.pl script. If no path is specified, the current directory is assumed
    /// to be a linux kernel tree.
    #[clap(short, long = "tree")]
    tree_path: Option<PathBuf>,

    /// Arguments that will be passed directly to git send-email. For example, you may specify --
    /// --dry-run --cc personal@my_domain.com to have the invocation be a dry-run, and to cc your
    /// own personal email address.
    #[clap(last = true)]
    extra_arguments: Vec<String>,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let tree_root = cli.tree_path.unwrap_or(PathBuf::from("."));
    let maintainers = maintainers::get_maintainers(&cli.patch_path, &tree_root);

    send::send_patch(&maintainers, &cli.patch_path, &cli.extra_arguments);
}
