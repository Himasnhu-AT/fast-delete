#!/bin/bash

# Function to run a command and measure its time
measure_time() {
    local cmd=$1
    local output_file=$2

    { time $cmd; } 2> $output_file
}

# Function to extract time from the output file
extract_time() {
    local output_file=$1
    grep "real" $output_file | awk '{print $2}'
}

# Ensure the frm binary is built
cargo build --release

# Generate test data
bash test_data_gen.sh

# Measure time for rm -rf
measure_time "rm -rf ./test_data" "rm_time.txt"
rm_time=$(extract_time "rm_time.txt")

# Regenerate test data
bash test_data_gen.sh

# Measure time for frm
measure_time "./target/release/frm ./test_data" "frm_time.txt"
frm_time=$(extract_time "frm_time.txt")

# Create performance.md file
cat <<EOL > performance.md
# Performance Benchmark

## Results

| Command | Time (s) |
|---------|----------|
| rm -rf  | $rm_time |
| frm     | $frm_time |

## Conclusion

The table above shows the time taken by each command to delete the test data directory. The \`frm\` command is compared against the traditional \`rm -rf\` command.
EOL

# Clean up
rm rm_time.txt frm_time.txt


clear
echo "| Command | Time (s) |"
echo "|---------|----------|"
echo "| rm -rf  | $rm_time |"
echo "| frm     | $frm_time |"
echo ""
echo ""
echo ""

echo "Benchmarking complete. Results stored in performance.md."
