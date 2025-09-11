PATH_DEST="/Users/stephane/Code/macos/ref/Numerologie.Du.Tarot.De.Marseille.Bressani.Dev/Sources/Numerologie.Du.Tarot.De.Marseille.Bressani.Dev/theme_numerologie_docx_rust"
./build_ios_xcframework.sh
cargo build --release --target aarch64-apple-darwin
cp /Users/stephane/Code/rust/ref/theme_numerologie_docx/ios_build/theme_numerologie_docx.xcframework/ios-arm64/libtheme_numerologie_docx.a $PATH_DEST/iosarm64
cp /Users/stephane/Code/rust/ref/theme_numerologie_docx/ios_build/theme_numerologie_docx.xcframework/ios-arm64-simulator/libtheme_numerologie_docx.a $PATH_DEST/iosarm64simulator
cp /Users/stephane/Code/rust/ref/theme_numerologie_docx/target/aarch64-apple-darwin/release/libtheme_numerologie_docx.a $PATH_DEST/macos