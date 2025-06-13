#!/bin/bash

# Check if a version parameter is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <new_version>"
  exit 1
fi

new_version="$1"

# Function to update the version in a Cargo.toml file
# [WARN] This relies on the version being on the 3rd line
update_cargo_toml() {
  file="$1"
  sed -i '3s/version = ".*"/version = "'"$new_version"'"/' "$file"
}

# Update Cargo.toml files
update_cargo_toml "Cargo.toml"
update_cargo_toml "src-tauri/Cargo.toml"

# Update tauri.conf.json in src-tauri directory
sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$new_version\"/" src-tauri/tauri.conf.json

echo "Version updated to $new_version in:"
echo "- Cargo.toml"
echo "- src-tauri/Cargo.toml"
echo "- src-tauri/tauri.conf.json"