#!/usr/bin/env bash
set -o errexit

TARGET=armv7-unknown-linux-gnueabihf

cross build --release --target "$TARGET"
ssh rm2-tablet rm /tmp/vnc-server || true
scp target/"$TARGET"/release/main rm2-tablet:/tmp/vnc-server
ssh rm2-tablet /tmp/vnc-server
