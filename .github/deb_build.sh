#!/bin/bash

# Define package information
PACKAGE_NAME="rusty-belt"
MAINTAINER="Aleksandr Mikhailov <iam@fidonode.me>"
DESCRIPTION="Rust replacement for tmux powerline"

AARCH="$1"
BINARY_PATH="$2"
VERSION="$3"

# Create package directory
PACKAGE_DIR="${PACKAGE_NAME}-${AARCH}-v${VERSION}"
mkdir -p "$PACKAGE_DIR/DEBIAN"

# Create control file
cat > "$PACKAGE_DIR/DEBIAN/control" <<EOF
Package: $PACKAGE_NAME
Architecture: $AARCH
Maintainer: $MAINTAINER
Description: $DESCRIPTION
Version: $VERSION

EOF

# Create install file
cat > "$PACKAGE_DIR/DEBIAN/install" <<EOF
tmux_client usr/bin
rusty_belt_server usr/bin
EOF

cp $BINARY_PATH/tmux_client $PACKAGE_DIR
cp $BINARY_PATH/rusty_belt_server $PACKAGE_DIR

# Create package
dpkg-deb --build "$PACKAGE_DIR"

# Clean up
rm -r "$PACKAGE_DIR"
