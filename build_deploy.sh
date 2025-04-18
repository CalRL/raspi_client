#!/bin/bash

set -e

APP_NAME="raspi_client"
TARGET="aarch64-unknown-linux-gnu"
REMOTE_USER="cal"
REMOTE_HOST="192.168.0.102"
REMOTE_PATH="/home/$REMOTE_USER/Documents/$APP_NAME/$APP_NAME"

echo "🛠 Building Rust app in release mode..."
cargo build --release --target aarch64-unknown-linux-gnu

echo "🚀 Copying binary to $REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH..."
scp "target/$TARGET/release/$APP_NAME" "$REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH"

echo "✅ Done! Binary deployed to $REMOTE_HOST"