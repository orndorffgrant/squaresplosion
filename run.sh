trap 'kill $(jobs -p)' EXIT
npx serve --ssl-cert minica/localhost.crt --ssl-key minica/localhost.key frontend &
cd backend
cargo run localhost:9999 --cert ../minica/localhost.crt --key ../minica/localhost.key
