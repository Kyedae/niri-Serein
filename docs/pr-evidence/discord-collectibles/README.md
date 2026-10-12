# Discord profile cosmetics

Baseline `178bde35`; the changed build is this PR's branch. Both are release builds of
`profile_preview --features demo`, run with the same `collectibles` fixture page. The baseline
worktree received only that page and the fixture-only `preview_profile_popout` helper. Its users
have no cosmetics because the field does not exist there.

```sh
cargo build --release --locked -p serein --features demo --example profile_preview
gamescope --backend headless -w 1440 -h 900 -W 1440 -H 900 -- env -u WAYLAND_DISPLAY \
  target/release/examples/profile_preview --demo --width=1440 --height=900 \
  --page=collectibles --output=after-popout.png   # also: --full, --light, --page=dm-tags
```

These images are eframe framebuffer exports, not OS window captures. All decoration, nameplate,
effect and frame artwork is drawn locally and is synthetic. None of it is Discord artwork, and no
network request is made. Pointer hover (animated nameplates and decorations) and screen-reader
behavior were not exercised natively.
