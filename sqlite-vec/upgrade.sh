#!/bin/bash
# Download latest sqlite-vec amalgamation

VERSION="v0.1.6"  # Update as needed
BASE_URL="https://github.com/asg017/sqlite-vec/releases/download"

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd "$SCRIPT_DIR"

echo "Downloading sqlite-vec ${VERSION}..."
curl -L -o sqlite-vec-amalgamation.zip "${BASE_URL}/${VERSION}/sqlite-vec-0.1.6-amalgamation.zip"

echo "Extracting files..."
unzip -o sqlite-vec-amalgamation.zip

echo "Cleaning up..."
rm sqlite-vec-amalgamation.zip

echo "Done! sqlite-vec files updated to ${VERSION}"
ls -lh sqlite-vec.c sqlite-vec.h
