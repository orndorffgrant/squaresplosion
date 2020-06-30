#!/usr/bin/env bash
set -x
set -e
if [ ! -f index.txt ]; then
  touch index.txt
fi
if [ ! -f serial.txt ]; then
  echo '01' > serial.txt
fi
# From https://stackoverflow.com/questions/21297139/how-do-you-sign-a-certificate-signing-request-with-your-certification-authority/21340898#21340898
# Create ca cert
openssl req -x509 -config openssl-ca.cnf -newkey rsa:4096 -sha256 -nodes -out devcacert.pem -keyout devcakey.pem -outform PEM
# Create localhost key and cert signing request
openssl req -config openssl-localhost.cnf -newkey rsa:2048 -sha256 -nodes -out localhostcert.csr -keyout localhostkey.pem -outform PEM
# Sign localhost key to create cert
openssl ca -config openssl-ca-signing.cnf -policy signing_policy -extensions signing_req -out localhostcert.pem -infiles localhostcert.csr
