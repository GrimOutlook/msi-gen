# msipmbuild

This project is to create a command line tool that can be used on both Windows
and Linux to create an MSI deliverable. The best alternative that I can find is
the [msitools](https://gitlab.gnome.org/GNOME/msitools) project but the
documentation for that tool is nearly non-existent and many features are lacking
such as CustomActions which is what this package aims to fix.

If you end up using this package I'd love to know as it helps me stay motivated
to continue working on projects like these! So drop a star or shoot me a message
if you get any use out of it!

## Todo

- [ ] Convert creation method to rpmbuild-esque
- [ ] Convert MsiError to anyhow crate.

## Development

- [MSI Reference
  Material](https://learn.microsoft.com/en-us/windows/win32/msi/specifying-directory-structure)
- `example_2.msi` is an msi for DualMonitorTools v2.3. Chosen at random from the
  smaller installers that were on my system.

## CI/CD Desires

- [Fuzzing?](https://github.com/rust-fuzz/afl.rs)
- [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat)
- [cargo-audit](https://rustsec.org/)
- [cargo-auditable](https://github.com/rust-secure-code/cargo-auditable)
- [cargo-deny](https://embarkstudios.github.io/cargo-deny/)
- [cargo-udeps](https://github.com/est31/cargo-udeps)
- [cargo-semver-checks](https://crates.io/crates/cargo-semver-checks)
- [cargo-spellcheck](https://github.com/drahnr/cargo-spellcheck)
- [cargo-unused-features](https://github.com/TimonPost/cargo-unused-features)
- [kani](https://github.com/model-checking/kani)
- [lockbud](https://github.com/BurtonQin/lockbud)
- [mirai](https://github.com/endorlabs/MIRAI)
