# MSI Builder (temp name)

This project is to create a command line tool that can be used on both Windows
and Linux to create an MSI deliverable. The best alternative that I can find is
the [msitools](https://gitlab.gnome.org/GNOME/msitools) project but the
documentation for that tool is nearly non-existent and many features are lacking
such as CustomActions.

## Development

- [MSI Reference
  Material](https://learn.microsoft.com/en-us/windows/win32/msi/specifying-directory-structure)

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
