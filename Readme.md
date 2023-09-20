# Uniffi Async Example

Showcase uniffi using async and no `udl` file.

Requires [cargo xcframework](https://crates.io/crates/xcframework).

1. Install [cargo xcframework](https://crates.io/crates/xcframework)
2. run `./make.sh`
3. Open `Andaluh` folder in xcode and run

## Notes

- This doesn't work with `cargo swift` yet, hence we need the xcframework and the generated `include/swiftandaluh.swift` file in our Xcode project
- Because it doesn't use `cargo swift` we're also not generating a Swift `Package`. I think this would be possible, but it would require more time
- In general the better way would be to use the `bazel` build system as this would also support debugging etc. This is something I want to look into (supporting uniffi, bazel & async in one go).