# Serein with PRs #674 and #698

This private Arch package builds upstream `main` and merges Kyedae's
`fix/linux-global-mouse-keybinds` and `fix/screenshare-audio` branches during
each build. It uses the repository's pinned Rust toolchain and lockfile, with
voice enabled. A merge conflict stops the build and leaves the installed
package alone. Future upstream changes can still require manual PR repairs.

Build with `makepkg -s`, then install the resulting `serein-custom-git` package
with `sudo pacman -U <package>`. It replaces the ordinary `serein` package.
Its source revisions are recorded in
`/usr/share/serein-custom-build/source-revisions`.

`serein-custom-update` checks upstream and both PR heads, then rebuilds and
installs only when one changes. `--check` checks without building; `--force`
rebuilds after a system library/toolchain change even if source is unchanged.
Builds run the standard package command and `cargo xtask check`.

The optional per-user wrapper is installed by creating an executable copy of
`/usr/share/serein-custom-build/yay` at `~/.local/bin/yay`, with that directory
before `/usr/bin` in PATH. It forwards all arguments to `/usr/bin/yay`.
After a successful `yay -Syu --devel`, it calls `serein-custom-update`.
The custom package is not published in the AUR; system yay may list it as
missing from the AUR. The wrapper handles its source updates separately.
`/usr/bin/yay` bypasses the wrapper. Plain upgrades without `--devel` leave
the custom package installed without rebuilding it.

To undo this setup, remove only the wrapper you installed at
`~/.local/bin/yay`, then install `serein` from the AUR and accept replacing
`serein-custom-git`.

## Umbriel

Use the Umbriel ScreenCast portal already installed for your session.
PR #674's evdev mouse bindings are independent of compositor, but require
read access to the relevant `/dev/input/event*` devices. No groups or input
permissions are changed by this package. Missing access leaves focused input
available. Queue overflow releases held actions and reopens the device during
a later scan; the next fresh press activates the binding again.

PR #698's Linux application-audio and VA-API border fixes are included.
Its window-specific audio filtering uses niri's `Casts`/`Windows` IPC and
does not apply on Umbriel. On Umbriel, enabling share audio captures all
eligible applications, including for window video shares. `UMBRIEL_SOCKET`
must not be passed as `NIRI_SOCKET`: their protocols differ. This package
does not claim live Discord, physical input, microphone or capture validation.
