#!/bin/sh

export MESON_BUILD_ROOT="$1"
CARGO_HOME="~/.cargo"
export MESON_SOURCE_ROOT="$2"
export CARGO_TARGET_DIR="$MESON_BUILD_ROOT"/target
if test -r $CARGO_HOME -a -w $CARGO_HOME || test -w "~"
then
echo "It seem we can use the host's Cargo home."
else
echo "It seems we can't use the host's Cargo home due to the permissions policy. This usually happens under sandboxes like Flatpak or Firejail. Setting the Cargo home to $MESON_BUILD_ROOT/cargo-home."
export CARGO_HOME="$MESON_BUILD_ROOT"/cargo-home
fi
export OUTPUT="$3"
export BUILDTYPE="$4"
export APP_BIN="$5"


if [ $BUILDTYPE = "release" ]
then
    echo "RELEASE MODE"
    cargo build --manifest-path \
        "$MESON_SOURCE_ROOT"/editor/Cargo.toml --release && \
        cp "$CARGO_TARGET_DIR"/release/"$APP_BIN" "$OUTPUT"
else
    echo "DEBUG MODE"
    cargo build --manifest-path \
        "$MESON_SOURCE_ROOT"/editor/Cargo.toml && \
        cp "$CARGO_TARGET_DIR"/debug/"$APP_BIN" "$OUTPUT"
fi

