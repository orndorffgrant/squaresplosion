set -x
set -e
# might need -keyout devcakey.pem
openssl req -x509 -config openssl-ca.cnf -newkey rsa:4096 -sha256 -nodes -out devcacert.pem -outform PEM


# openssl genrsa -out devca.key 2048
# openssl req -x509 -new -nodes -key devca.key -sha256 -days 1825 -out devca.pem -subj '/CN=devca'
# openssl genrsa -out localhost.key 2048
# openssl req -new -key localhost.key -out localhost.csr -subj '/CN=localhost'
# openssl x509 -req -in localhost.csr -CA devca.pem -CAkey devca.key -CAcreateserial \
#     -out localhost.crt -days 365 -sha256 -extfile <( \
#     printf "[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")


# openssl req -x509 -out localhost.crt -keyout localhost.key \
#   -newkey rsa:2048 -nodes -sha256 \
#   -subj '/CN=localhost' -extensions EXT -config <( \
#    printf "[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")

mkdir -p ~/.mozilla/certificates
cp devca.pem ~/.mozilla/certificates
cp policies.json /usr/lib/firefox/distribution/