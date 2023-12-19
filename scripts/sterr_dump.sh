# make logs dir exists
mkdir -p logs/

# 2>&1 means print both stderr and std out. redirect to file
cargo run -- dump >logs/stderr_dump.txt 2>&1 

# Get the absolute path of the script file
SCRIPT_PATH=$(readlink -f "$0")

# Extract the test name from the script path
TEST_NAME=$(basename "$SCRIPT_PATH")

# Print the final full path of the new output file
echo "Output file path: $SCRIPT_PATH/logs/stderr_dump.txt"
echo "Test name: $TEST_NAME"

