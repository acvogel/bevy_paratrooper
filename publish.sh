#!/usr/bin/bash
set -x

VERSION='0.3.0'
DISTDIR='dist/'

# linux
OUTDIR="$DISTDIR/linux/"
cargo build --release
mkdir -p $OUTDIR
cp -r assets $OUTDIR
cp target/release/bevy_paratrooper $OUTDIR
pushd $OUTDIR
zip -r -9 paratrooper.zip assets/ bevy_paratrooper
butler push paratrooper.zip mbusux/paratrooper:linux --userversion $VERSION
popd

# windows
OUTDIR="$DISTDIR/windows/"
cargo build --target=x86_64-pc-windows-gnu --release
mkdir -p $OUTDIR
cp -r assets $OUTDIR
cp target/x86_64-pc-windows-gnu/release/bevy_paratrooper.exe $OUTDIR
pushd $OUTDIR
zip -r -9 paratrooper.zip assets/ bevy_paratrooper.exe
butler push paratrooper.zip mbusux/paratrooper:windows --userversion $VERSION
popd
