#!/bin/bash

# 设置变量
APP_NAME="Tsukimi"
EXECUTABLE_NAME="tsukimi"
CARGO_TARGET_DIR="target/release"
APP_BUNDLE_PATH="$APP_NAME.app"
FRAMEWORKS_PATH="$APP_BUNDLE_PATH/Contents/Frameworks"
RESOURCES_PATH="$APP_BUNDLE_PATH/Contents/Resources"
MACOS_PATH="$APP_BUNDLE_PATH/Contents/MacOS"

# 创建应用程序包结构
mkdir -p "$FRAMEWORKS_PATH" "$RESOURCES_PATH" "$MACOS_PATH"

# 复制可执行文件
cp "$CARGO_TARGET_DIR/$EXECUTABLE_NAME" "$MACOS_PATH/"

# 复制动态库
copy_and_fix_dylib() {
    local lib=$1
    local dest="$FRAMEWORKS_PATH/$(basename $lib)"
    cp "$lib" "$dest"
    chmod 644 "$dest"
    install_name_tool -id "@executable_path/../Frameworks/$(basename $lib)" "$dest"
}

# 复制并修复主要依赖库
copy_and_fix_dylib "/path/to/libgtk-4.dylib"
copy_and_fix_dylib "/path/to/libgstreamer-1.0.dylib"
copy_and_fix_dylib "/path/to/libmpv.dylib"

# 复制其他必要的库（根据需要添加更多）
cp -R /usr/local/lib/gdk-pixbuf-2.0 "$FRAMEWORKS_PATH/"
cp -R /usr/local/lib/gio "$FRAMEWORKS_PATH/"

# 复制 GLib schemas
mkdir -p "$RESOURCES_PATH/glib-2.0"
cp /usr/local/share/glib-2.0/schemas/gschemas.compiled "$RESOURCES_PATH/glib-2.0/"

# 复制图标和其他资源
cp /path/to/your/icons/*.svg "$RESOURCES_PATH/"

# 创建 Info.plist 文件
cat > "$APP_BUNDLE_PATH/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleExecutable</key>
    <string>${EXECUTABLE_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.${APP_NAME}</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# 修复所有库的依赖关系
find "$APP_BUNDLE_PATH" -name "*.dylib" -o -name "*.so" | while read lib; do
    otool -L "$lib" | grep -v ":" | awk '{print $1}' | while read dep; do
        base_dep=$(basename "$dep")
        if [ -f "$FRAMEWORKS_PATH/$base_dep" ]; then
            install_name_tool -change "$dep" "@executable_path/../Frameworks/$base_dep" "$lib"
        fi
    done
done

# 修复可执行文件的依赖关系
otool -L "$MACOS_PATH/$EXECUTABLE_NAME" | grep -v ":" | awk '{print $1}' | while read dep; do
    base_dep=$(basename "$dep")
    if [ -f "$FRAMEWORKS_PATH/$base_dep" ]; then
        install_name_tool -change "$dep" "@executable_path/../Frameworks/$base_dep" "$MACOS_PATH/$EXECUTABLE_NAME"
    fi
done

echo "打包完成：$APP_BUNDLE_PATH"