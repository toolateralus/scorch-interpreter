# make logs dir exists
mkdir -p logs/

# 2>&1 means print both stderr and std out. redirect to file
cargo run -- dump >logs/stderr_dump.txt 2>&1 

echo "Output file path: logs/stderr_dump.txt"