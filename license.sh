#!/bin/bash

LICENSE="// SPDX-License-Identifier: MIT"

for file in $(find . -name '*.rs'); do
  if ! grep -q "$LICENSE" "$file"; then
    echo -e "$LICENSE\n$(cat $file)" > "$file"
    echo "Added license to $file"
  fi
done
