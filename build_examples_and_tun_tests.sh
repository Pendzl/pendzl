#!/bin/bash

IGNORED_DIRS=(
  "./examples/test_helpers"
  "./examples/test_helpers/"
)

ignore_dir() {
  local element
  for element in "${@:2}"; do
    [[ "$element" == "$1" ]] && return 0
  done
  return 1
}

process_directory() {
  local dir=$1

  if ignore_dir "$dir" "${IGNORED_DIRS[@]}"; then
    echo "Ignoring $dir" 
    return
  fi

  if [ -f "${dir}/Cargo.toml" ]; then
    cd "$dir" || exit

    echo "Building contract in $dir"
    cargo contract build  --release || exit

    echo "Running e2e-tests in $dir"
    cargo test --features e2e-tests --release || exit

    cd - || exit
  else
    for inner in "$dir"/*/; do
      if [[ -d $inner ]]; then
        process_directory "$inner"
      fi
  done
  fi
}

for pattern in "./examples"; do
  for dir in $pattern; do
    if [[ -d $dir ]]; then
      process_directory "$dir"
    fi
  done
done
