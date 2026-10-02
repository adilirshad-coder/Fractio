#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
expected=$(sed -n 's/^declare_id!("\([^"]*\)");$/\1/p' programs/fractio/src/lib.rs)
test -n "$expected"
test "$(grep -c '^declare_id!' programs/fractio/src/lib.rs)" -eq 1
mapfile -t anchor_ids < <(sed -n 's/^fractio = "\([^"]*\)"$/\1/p' Anchor.toml)
ids=("${anchor_ids[@]}" "$(sed -n 's/^FRACTIO_PROGRAM_ID=//p' .env.example)" "$(sed -n 's/.*FRACTIO_PROGRAM_ID: &str = "\([^"]*\)";.*/\1/p' backend/src/config.rs)")
for id in "${ids[@]}"; do
  test "$id" = "$expected" || { echo "program ID mismatch: expected $expected, got $id" >&2; exit 1; }
done
test "${#anchor_ids[@]}" -eq 2
test "${#ids[@]}" -eq 4
echo "Program ID locations are synchronized: $expected"
