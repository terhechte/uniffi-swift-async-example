cargo build --target aarch64-apple-ios
# Create Headers
cargo run --bin uniffi-bindgen generate --library target/aarch64-apple-ios/debug/libswiftandaluh.dylib --language swift --out-dir include
# Rename
mv include/swiftandaluhFFI.modulemap include/module.modulemap
# delete old
rm -rf target/swiftandaluhFFI.xcframework
# Generate
xcframework -r
