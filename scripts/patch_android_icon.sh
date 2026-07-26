#!/usr/bin/env bash
# dx (0.7.9) doesn't apply Dioxus.toml's [bundle] icon to the generated
# Android project, so it always ships the default robot placeholder. This
# script patches our real icon into whatever `dx build`/`dx bundle` last
# generated under target/dx/*/debug/android/app. Run it after any Android
# build/bundle, before installing the APK.
set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOURCE_ICON="$PROJECT_ROOT/icons/icon.png"
RES_DIR=$(find "$PROJECT_ROOT/target/dx" -type d -path "*android/app/app/src/main/res" | head -1)

if [ -z "$RES_DIR" ]; then
    echo "error: no generated Android res/ directory found. Run 'dx build --platform android' first." >&2
    exit 1
fi

echo "Patching Android icon into: $RES_DIR"

declare -A SIZES=( [mdpi]=48 [hdpi]=72 [xhdpi]=96 [xxhdpi]=144 [xxxhdpi]=192 )

for density in "${!SIZES[@]}"; do
    size="${SIZES[$density]}"
    dir="$RES_DIR/mipmap-$density"
    mkdir -p "$dir"

    # Legacy square launcher icon (used pre-Android-8 and as a fallback).
    convert "$SOURCE_ICON" -resize "${size}x${size}" "$dir/ic_launcher.webp"

    # Adaptive icon foreground: same artwork, reused as-is (it already has
    # generous internal padding so it survives the OS's circular/squircle mask).
    convert "$SOURCE_ICON" -resize "${size}x${size}" "$dir/ic_launcher_foreground.webp"

    # Adaptive icon background: flat fill sampled from the icon's own sky color.
    convert -size "${size}x${size}" xc:'#e6f5ec' "$dir/ic_launcher_background.webp"
done

# Point the adaptive-icon descriptor at our raster mipmaps instead of the
# default vector drawables.
cat > "$RES_DIR/mipmap-anydpi-v26/ic_launcher.xml" <<'EOF'
<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@mipmap/ic_launcher_background" />
    <foreground android:drawable="@mipmap/ic_launcher_foreground" />
</adaptive-icon>
EOF

rm -f "$RES_DIR/drawable/ic_launcher_background.xml" "$RES_DIR/drawable-v24/ic_launcher_foreground.xml"

echo "Done. Re-run gradlew assembleDebug (or dx bundle again without regenerating) to package."
