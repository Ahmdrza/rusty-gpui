#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This launcher is for macOS. On Linux, use cargo run --locked -p rusty-gpui-gallery."
  exit 1
fi
if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust is missing. Install it from https://rustup.rs, reopen Terminal, then run this script again."
  exit 1
fi
if ! xcrun --find metal >/dev/null 2>&1; then
  echo "GPUI requires Xcode with the Metal compiler."
  echo "Install/open Xcode, accept its license, and select it in Xcode > Settings > Locations > Command Line Tools."
  echo "On Xcode versions that download Metal separately, run: xcodebuild -downloadComponent MetalToolchain"
  exit 1
fi
echo "Building and opening Rusty GPUI's native GPUI gallery. The first build downloads dependencies and can take several minutes."
exec cargo run --locked -p rusty-gpui-gallery
