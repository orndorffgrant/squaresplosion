set -x
set -e

mkdir -p "/Library/Application Support/Mozilla/Certificates"
cp devcacert.pem "/Library/Application Support/Mozilla/Certificates"
mkdir -p "/Applications/Firefox Developer Edition.app/Contents/Resources/distribution"
cp policies.json "/Applications/Firefox Developer Edition.app/Contents/Resources/distribution"
