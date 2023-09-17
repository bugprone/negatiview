#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cd client; trunk serve' &
 bash -c 'cd server; cargo watch -- cargo run --bin server -- --port 8081')
