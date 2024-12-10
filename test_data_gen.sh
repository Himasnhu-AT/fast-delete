#!/bin/bash

# Function to generate a random string with a fixed seed
generate_random_string() {
    local length=$1
    local seed=$2
    echo "$seed" | md5 | head -c $length
}

# Function to create random files and directories with a fixed seed
create_random_files_and_dirs() {
    local base_dir=$1
    local num_dirs=$2
    local num_files=$3
    local depth=$4
    local seed=$5

    if [ $depth -le 0 ]; then
        return
    fi

    for ((i = 0; i < num_dirs; i++)); do
        local dir_name=$(generate_random_string 8 "$seed-$i")
        local dir_path="$base_dir/$dir_name"
        mkdir -p "$dir_path"
        echo "Created directory: $dir_path"

        # Recursively create files and directories in the new directory
        create_random_files_and_dirs "$dir_path" $((num_dirs / 2)) $((num_files / 2)) $((depth - 1)) "$seed-$i"
    done

    for ((i = 0; i < num_files; i++)); do
        local file_name=$(generate_random_string 8 "$seed-$i")
        local file_path="$base_dir/$file_name.txt"
        head -c 1048576 </dev/urandom >"$file_path"  # Increase file size to 1MB
        echo "Created file: $file_path"
    done
}

# Main script
BASE_DIR="./test_data"
NUM_DIRS=40
NUM_FILES=900
DEPTH=4
SEED="fixed_seed"

# Create base directory if it doesn't exist
mkdir -p "$BASE_DIR"

# Generate random files and directories with a fixed seed
create_random_files_and_dirs "$BASE_DIR" $NUM_DIRS $NUM_FILES $DEPTH "$SEED"

echo "Random file and directory generation complete."
