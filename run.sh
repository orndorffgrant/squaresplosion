trap 'kill $(jobs -p)' EXIT
npx serve --ssl-cert devca/localhostcert.pem --ssl-key devca/localhostkey.pem frontend &
cd backend
cargo run localhost:9999 --cert ../devca/localhostcert.pem --key ../devca/localhostkey.pem
