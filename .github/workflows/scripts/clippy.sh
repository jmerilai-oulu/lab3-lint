#!/bin/bash
set -xeu

clippy_exit_code=$?

if [ $clippy_exit_code -eq 0 ]; then
    echo "No Clippy errors or warnings found."
else
    echo "Clippy found errors or warnings."
    exit 1
fi