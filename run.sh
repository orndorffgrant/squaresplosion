trap 'kill $(jobs -p)' EXIT
npx serve --ssl-cert devca/localhost.crt --ssl-key devca/localhost.key frontend &
cd backend
cargo run localhost:9999 --cert ../devca/localhost.crt --key ../devca/localhost.key
