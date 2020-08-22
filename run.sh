#!/usr/bin/env bash
trap 'kill $(jobs -p)' EXIT
npx serve --ssl-cert ~/.local/share/devca/certs/localhost/cert.pem --ssl-key ~/.local/share/devca/certs/localhost/key.pem frontend &
cd backend
cargo run localhost:9999 --cert ~/.local/share/devca/certs/localhost/cert.pem --key ~/.local/share/devca/certs/localhost/key.pem
