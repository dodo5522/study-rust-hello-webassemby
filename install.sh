#!/usr/bin/env bash
set -eu

echo "
#!/bin/bash

cd \$(git rev-parse --show-toplevel) || exit 1

for FILE in \$(git diff --staged --name-only | grep -E '.*\.rs$'); do
  cargo fmt -- \${FILE}
  git add \${FILE}
done
" > pre-commit

chmod +x pre-commit
mv pre-commit .git/hooks/pre-commit
