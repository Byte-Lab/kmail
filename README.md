# kmail

`kmail` is a small helper utility tool for automatically collecting the email
addresses that should be included when emailing a patch, and invoking git
send-email with those addresses and the patch set. This could easily have been a
bash script, but I like rust.

# Installation

Firstly, make sure you have `cargo` installed on your system. Something like
this should do the trick depending on what platform you're on:

```
$ sudo dnf install cargo
```

or

```
sudo apt install cargo
```

Once you have `cargo` installed, you can install `kmail` with:

```
$ cargo install kmail
```

Make sure that you add the `$HOME/.cargo/bin` to your `$PATH` so that any
binaries installed by `cargo` can be automatically discovered:

```
echo "export PATH=$PATH:$HOME/.cargo.bin" >> ~/.bashrc
```

# Usage

Once you have `kmail` installed, you can simply invoke `kmail` on the command
line to use it. Let's take a look at the usage message:

```
$ kmail --help
kmail 1.0
David Vernet <void@manifault.com>
A utility for automatically sending emails to the correct maintainers for a kernel patchset.

Kmail invokes scripts/get_maintainer.pl on a patchset, parses the email
addresses (both the maintainers themselves and the relevant lists) to
email, and sends them the patchset using git send-email.

USAGE:
    kmail [OPTIONS] <PATCH_PATH> [-- <EXTRA_ARGUMENTS>...]

ARGS:
    <PATCH_PATH>            The path to the patch (or a directory containing patches) to be
                            mailed to the kernel upstream community using git send-email
    <EXTRA_ARGUMENTS>...    Arguments that will be passed directly to git send-email. For
                            example, you may specify -- --dry-run --cc personal@my_domain.com to
                            have the invocation be a dry-run, and to cc your own personal email
                            address

OPTIONS:
    -h, --help                Print help information
    -t, --tree <TREE_PATH>    The path to a linux source tree containing a MAINTAINERS file, and a
                              scripts/get_maintainer.pl script. If no path is specified, the current
                              directory is assumed to be a linux kernel tree
    -V, --version             Print version information
```

As you can see, there aren't many options to specify, and there is only 1
required argument (though it depends on how the script is invoked).

## Emailing Patch

Say that you had a patch `/tmp/0001-TESTING-MY-DUMB-SCRIPT.patch` that you
wanted to send out for review, and which had the following entry in
`MAINTAINERS` for the subsystem that was updated in the patch:

```
LIVE PATCHING
M:      David Vernet <void@manifault.com>
L:      fake-list@manifault.com
S:      Maintained
```

You could use `kmail` to automatically invoke `git send-email` like this:

```
$ pwd
/home/Decave/linux

$ kmail /tmp/0001-TESTING-MY-DUMB-SCRIPT.patch

/tmp/0001-TESTING-MY-DUMB-SCRIPT.patch
From: David Vernet <void@manifault.com>
To: fake-list@manifault.com,
        void@manifault.com
Subject: [PATCH] TESTING MY DUMB SCRIPT
Date: Tue, 22 Feb 2022 06:36:07 -0800
Message-Id: <20220222143607.1582198-1-void@manifault.com>
X-Mailer: git-send-email 2.30.2
MIME-Version: 1.0
Content-Transfer-Encoding: 7bit

Send this email? ([y]es|[n]o|[e]dit|[q]uit|[a]ll):
```

## Specific tree

`kmail` accepts a `-t` / `--tree` option that allows you to point to a specific
linux kernel tree (if you're a linux kernel contributor then you probably have
many trees checked out locally depending on what subsystem you're contributing
to for a patch). If the option is not speciifed (as it was not in the example
above), `kmail` checks if your local tree is a linux kernel tree. Otherwise, you
may specify `-t` as follows:

```
$ pwd
/home/Decave/linux

$ kmail --tree ../linus /tmp/0001-TESTING-MY-DUMB-SCRIPT.patch

/tmp/0001-TESTING-MY-DUMB-SCRIPT.patch
From: David Vernet <void@manifault.com>
To: fake-list@manifault.com,
        void@manifault.com
Subject: [PATCH] TESTING MY DUMB SCRIPT
Date: Tue, 22 Feb 2022 06:56:29 -0800
Message-Id: <20220222145629.1659614-1-void@manifault.com>
X-Mailer: git-send-email 2.30.2
MIME-Version: 1.0
Content-Transfer-Encoding: 7bit

Send this email? ([y]es|[n]o|[e]dit|[q]uit|[a]ll):
```

In this example, `kmail` uses the `MAINTAINERS` and `scripts/get_maintainer.pl`
files from Linus' source tree rather than from the "linux" tree in the current
working directory.

## Passing arguments to git send-email

You may pass additional arguments to `git send-email` by specifying them after a
`--`, which itself follows all options and parameters that are specific to
`kmail`:

```
$ kmail --tree ../linus /tmp/0001-TESTING-MY-DUMB-SCRIPT.patch -- --dry-run

/tmp/0001-TESTING-MY-DUMB-SCRIPT.patch
Dry-OK. Log says:
Sendmail: /bin/msmtp -f void@manifault.com -i fake-list@manifault.com nothing@manifault.com void@manifault.com
From: David Vernet <void@manifault.com>
To: fake-list@manifault.com,
        void@manifault.com
Subject: [PATCH] TESTING MY DUMB SCRIPT
Date: Tue, 22 Feb 2022 06:59:33 -0800
Message-Id: <20220222145933.1671200-1-void@manifault.com>
X-Mailer: git-send-email 2.30.2
MIME-Version: 1.0
Content-Transfer-Encoding: 7bit

Result: OK
```

In this case, we passed `--dry-run`. We could also, for example, have passed
`--cc another-email@domain.com` if we wanted to cc another recipient on the
patch:

```
$ kmail --tree ../linus /tmp/0001-TESTING-MY-DUMB-SCRIPT.patch -- --dry-run --cc another-email@domain.com

/tmp/0001-TESTING-MY-DUMB-SCRIPT.patch
Dry-OK. Log says:
Sendmail: /bin/msmtp -f void@manifault.com -i fake-list@manifault.com nothing@manifault.com void@manifault.com another-email@domain.com
From: David Vernet <void@manifault.com>
To: fake-list@manifault.com,
        void@manifault.com
Cc: another-email@domain.com
Subject: [PATCH] TESTING MY DUMB SCRIPT
Date: Tue, 22 Feb 2022 07:02:25 -0800
Message-Id: <20220222150225.1681218-1-void@manifault.com>
X-Mailer: git-send-email 2.30.2
MIME-Version: 1.0
Content-Transfer-Encoding: 7bit

Result: OK
```

# Contributing

I don't think there's much (if anything) else to do here, but if you think of
some feature you'd like, please feel free to contribute.
