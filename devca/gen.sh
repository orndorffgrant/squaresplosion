set -x
set -e
touch index.txt
echo '01' > serial.txt
# From https://stackoverflow.com/questions/21297139/how-do-you-sign-a-certificate-signing-request-with-your-certification-authority/21340898#21340898
# Create ca cert
openssl req -x509 -config openssl-ca.cnf -newkey rsa:4096 -sha256 -nodes -out devcacert.pem -keyout devcakey.pem -outform PEM
# Create localhost key and cert signing request
openssl req -config openssl-localhost.cnf -newkey rsa:2048 -sha256 -nodes -out localhostcert.csr -keyout localhostkey.pem -outform PEM
# Sign localhost key to create cert
openssl ca -config openssl-ca-signing.cnf -policy signing_policy -extensions signing_req -out localhostcert.pem -infiles localhostcert.csr

mkdir -p ~/.mozilla/certificates
cp devcacert.pem ~/.mozilla/certificates
cp policies.json /usr/lib/firefox/distribution/