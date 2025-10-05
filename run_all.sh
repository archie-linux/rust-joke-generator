#!/bin/bash
# filepath: /Users/anish/anpa6841/github-projects/random-joke-generator-api/run_all.sh

# Start Rust backend
nohup cargo run > backend.log 2>&1 &

# Start Flask GUI & Notifier
cd gui
nohup python3 app.py > gui.log 2>&1 &
nohup python3 notifier.py > notifier.log 2>&1 &
