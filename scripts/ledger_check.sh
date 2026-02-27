#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "ERROR: usage: $0 <ledger.md>" >&2
  exit 2
fi

LEDGER_PATH="$1"
if [[ ! -f "$LEDGER_PATH" ]]; then
  echo "ERROR: ledger file not found: $LEDGER_PATH" >&2
  exit 1
fi

expected_header='| path | layer | component | status | proof | notes |'
header_line="$(awk '/^\|[[:space:]]*path[[:space:]]*\|/ { print; exit }' "$LEDGER_PATH")"
if [[ -z "$header_line" ]]; then
  echo "FAIL: missing ledger table header" >&2
  exit 1
fi
if [[ "$header_line" != "$expected_header" ]]; then
  echo "FAIL: header mismatch" >&2
  echo "expected: $expected_header" >&2
  echo "actual:   $header_line" >&2
  exit 1
fi

row_count="$(
awk '
BEGIN {
  valid["todo"]=1;
  valid["stubbed"]=1;
  valid["implemented"]=1;
  valid["verified"]=1;
  valid["skipped"]=1;
  rows=0;
  in_table=0;
}

/^\| path \| layer \| component \| status \| proof \| notes \|$/ {
  in_table=1;
  next;
}

in_table && /^\|[[:space:]]*---[[:space:]]*\|/ { next; }

in_table && /^\|/ {
  line=$0;
  trimmed=line;
  gsub(/^\|[[:space:]]*/, "", trimmed);
  gsub(/[[:space:]]*\|$/, "", trimmed);
  n=split(trimmed, cells, /\|/);
  if (n != 6) {
    print "FAIL: wrong column count: " line > "/dev/stderr";
    exit 1;
  }

  for (i=1; i<=n; i++) {
    gsub(/^[[:space:]]+|[[:space:]]+$/, "", cells[i]);
  }

  status=cells[4];
  if (!(status in valid)) {
    print "FAIL: invalid or missing status '"'"'" status "'"'"' in row: " line > "/dev/stderr";
    exit 1;
  }

  rows++;
  next;
}

in_table && !/^\|/ {
  in_table=0;
}

END {
  if (rows == 0) {
    print "FAIL: no ledger rows found" > "/dev/stderr";
    exit 1;
  }
  print rows;
}
' "$LEDGER_PATH"
)"

echo "PASS: ledger status validation passed (${row_count} rows)"
