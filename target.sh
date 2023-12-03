#!/bin/bash

# Find and delete all "target" directories
find . -type d -name 'target' -exec rm -r {} +

echo "Deleted all 'target' directories."