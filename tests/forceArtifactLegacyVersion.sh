#!/bin/bash

for file in ./artifacts/*; do sed -i 's/"version": 5/"version": 4/g' "$file"; done