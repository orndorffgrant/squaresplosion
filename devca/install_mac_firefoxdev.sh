set -x
set -e

mkdir -p ~/Library/Application Support/Mozilla/Certificates
cp devcacert.pem ~/Library/Application Support/Mozilla/Certificates
cp policies.json "Firefox Developer Edition.app/Contents/Resources/distribution"
