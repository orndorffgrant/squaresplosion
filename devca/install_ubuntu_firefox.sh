#!/usr/bin/env bash
set -x
set -e

mkdir -p ~/.mozilla/certificates
cp devcacert.pem ~/.mozilla/certificates
cp policies.json /usr/lib/firefox/distribution/
