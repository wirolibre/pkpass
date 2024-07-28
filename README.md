# Woborder

> W/o border, Wob order

## Generate a PKCS#12 archive to sign passes

```sh
# Generate a RSA private key
pkpass crypto key --output pkpass.key
# Create a certificate signing request with this key or your own
pkpass crypto request --private-key pkpass.key --output pkpass.csr
# and submit your certificate signing request to Apple.

# Create an archive containing all you need to sign
pkpass crypto bundle --private-key pkpass.key --certificate path/to/pass.cer --output pkpass.p12
```
