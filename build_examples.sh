#!/bin/bash

if [ -n "$1" ]; then
  echo "Building examples with tests"
  BUILD_TESTS=true
else
  echo "Building examples without tests"
  BUILD_TESTS=false
fi

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

    if [ "$BUILD_TESTS" = true ]; then
      echo "Running e2e-tests in $dir"
      cargo test --features e2e-tests --release || exit
    fi

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
