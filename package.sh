#!/usr/bin/env bash
# Portable Windows bundle for tsukimi, adapted from the official
# .github/workflows/build_windows.yml that upstream deleted in 432db6d.
# Run from the repo root inside MSYS2 UCRT64 after
# `cargo build --release`.
set -euo pipefail

MSYS_PREFIX=/ucrt64
WORKSPACE="$(pwd)"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"
ARTIFACT="${1:-$WORKSPACE/../artifact}"
BUNDLE="$ARTIFACT/tsukimi-x86_64-windows-gnu"

rm -rf "$BUNDLE"
mkdir -p "$BUNDLE/bin" "$BUNDLE/share" "$BUNDLE/lib"
cd "$BUNDLE"

cp -v "$WORKSPACE/target/release/tsukimi.exe" bin/
cp -v $MSYS_PREFIX/bin/gdbus.exe bin/

cp -r $MSYS_PREFIX/lib/gdk-pixbuf-2.0 lib/
find lib/gdk-pixbuf-2.0/2.10.0/loaders -type f ! -name "*.dll" -exec rm -f "{}" \;

cp -r $MSYS_PREFIX/lib/gio lib/

cp -r $MSYS_PREFIX/lib/gstreamer-1.0 lib/
find lib/gstreamer-1.0 -type f ! -name "*.dll" -exec rm -f "{}" \;

# Locale: compile .mo from po/ (upstream CI got these from a meson step;
# here msgfmt is driven directly). LINGUAS has CRLF line endings.
while IFS= read -r lang; do
    lang="${lang%$'\r'}"
    [ -z "$lang" ] && continue
    mkdir -p "share/locale/$lang/LC_MESSAGES"
    msgfmt "$WORKSPACE/po/$lang.po" -o "share/locale/$lang/LC_MESSAGES/tsukimi.mo"
done < "$WORKSPACE/po/LINGUAS"

cp -r $MSYS_PREFIX/share/glib-2.0 share/
find share/glib-2.0/* -maxdepth 0 -type d ! -name "*schemas*" -exec rm -rf "{}" \;
cp -v "$WORKSPACE/resources/moe.tsuna.tsukimi.gschema.xml" share/glib-2.0/schemas/
glib-compile-schemas share/glib-2.0/schemas/
find share/glib-2.0/ -type f ! -name "*.compiled" -exec rm -f "{}" \;

cp -r $MSYS_PREFIX/share/icons share/
cp -v "$WORKSPACE/resources/icons/moe.tsuna.tsukimi.svg" share/icons/
rm -rf share/icons/hicolor share/icons/AdwaitaLegacy share/icons/Adwaita/scalable \
    share/icons/Adwaita/cursors share/icons/Adwaita/16x16 share/icons/Adwaita/symbolic-up-to-32

# The app loads share/tsukimi/tsukimi.gresource relative to the exe prefix.
mkdir -p share/tsukimi
glib-compile-resources --sourcedir="$WORKSPACE/resources" \
    --target=share/tsukimi/tsukimi.gresource "$WORKSPACE/resources/resources.gresource.xml"

find . -type d -empty -delete

# Encoders and exotic plugins the app never uses (same list as upstream CI).
(cd lib/gstreamer-1.0 && rm -f \
    libgstadpcmenc.dll libgstamfcodec.dll libgstdvbsubenc.dll libgstencoding.dll \
    libgstfrei0r.dll libgstinter.dll libgstlame.dll libgstldac.dll libgstmpeg2enc.dll \
    libgstmpegpsmux.dll libgstmpegtsmux.dll libgstmplex.dll libgstrealmedia.dll \
    libgstsubenc.dll libgstsvtav1.dll libgstsvthevcenc.dll libgsttwolame.dll \
    libgstvoamrwbenc.dll libgstwavenc.dll libgstx264.dll libgstx265.dll \
    libgstxingmux.dll libgsty4menc.dll libgstzbar.dll)

# Harvest DLL dependencies. Note: modern cp -n exits 1 when it skips.
ldd bin/tsukimi.exe | grep -o "$MSYS_PREFIX.*\.dll" | sort -u | while IFS= read -r dll; do
    cp -n "$dll" bin/ || true
done
find lib/ -type f -name "*.dll" -exec ldd "{}" \; | grep -o "$MSYS_PREFIX.*\.dll" | sort -u |
    while IFS= read -r dll; do
        cp -n "$dll" bin/ || true
    done

cd "$BUNDLE"
rm -f "$ARTIFACT/tsukimi-v$VERSION-x86_64-windows-gnu.7z"
7z a "$ARTIFACT/tsukimi-v$VERSION-x86_64-windows-gnu.7z" ./*

echo "PACKAGE_OK: $ARTIFACT/tsukimi-v$VERSION-x86_64-windows-gnu.7z"
