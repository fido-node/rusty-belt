#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <folder_path> <archive_base_name>"
    exit 1
fi

folder_path="$1"
archive_base_name="$2"
output_dir="$(pwd)"

# Check if the folder path contains "apple"
if [[ "$folder_path" == *"apple"* ]]; then
    # Use zip if "apple" is in the folder path
    archive_name="$archive_base_name.zip"
    zip -j "$output_dir/$archive_name" "$folder_path/rusty_belt_server" "$folder_path/tmux_client"
else
    # Use tar.gz for other cases
    archive_name="$archive_base_name.tar.gz"
    tar -czf "$output_dir/$archive_name" -C "$folder_path" rusty_belt_server tmux_client
fi

echo "Archive created: $output_dir/$archive_name"
