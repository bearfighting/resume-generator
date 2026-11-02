#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture="${project_root}/fixtures/pdf-spike.html"
output_dir="${1:-$(mktemp -d)}"
chrome_bin="${CHROME_BIN:-google-chrome}"
mkdir -p "${output_dir}"

for paper in A4 Letter; do
  pdf="${output_dir}/resume-${paper,,}.pdf"
  text_file="${output_dir}/resume-${paper,,}.txt"
  html="${output_dir}/resume-${paper,,}.html"
  sed "s/__PAGE_SIZE__/${paper}/" "${fixture}" > "${html}"
  "${chrome_bin}" --headless --no-sandbox --disable-gpu --disable-dev-shm-usage \
    --disable-crash-reporter --disable-breakpad --noerrdialogs \
    --no-pdf-header-footer --print-to-pdf="${pdf}" \
    --user-data-dir="${output_dir}/chrome-profile-${paper,,}" \
    "file://${html}"
  pdfinfo "${pdf}" > "${output_dir}/resume-${paper,,}-info.txt"
  pdftotext -layout "${pdf}" "${text_file}"
  grep -Fq 'Alex Morgan' "${text_file}"
  grep -Fq 'Northstar Systems' "${text_file}"
  grep -Fq '中文' "${text_file}"
  grep -Fq 'without-causing-horizontal-overflow-or-clipping' "${text_file}"
  if ! grep -Eq 'Pages:[[:space:]]+([3-9]|[1-9][0-9]+)' "${output_dir}/resume-${paper,,}-info.txt"; then
    echo "Expected a 3+ page PDF for ${paper}; see ${output_dir}/resume-${paper,,}-info.txt" >&2
    exit 1
  fi
  if ! python3 "${project_root}/scripts/check-pdf-pages.py" "${text_file}"; then
    echo "Expected at least three pages with 500+ text characters each for ${paper}; see ${text_file}" >&2
    exit 1
  fi
  echo "${paper}: ${pdf}"
  cat "${output_dir}/resume-${paper,,}-info.txt"
done

echo "Extracted text and PDF metadata: ${output_dir}"
