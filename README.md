# pkpass

<p align="center"><strong>
A toolchain for creating `pkpass` files and interacting with providers (Apple)
</strong></p>

<p align="center">
  <img alt="nix flake available" src="https://img.shields.io/badge/Flake-Available-blue?logo=nixos" />
  <a href="https://wakatime.com/badge/user/2a9c42cf-576a-484e-962a-e346e913d9ac/project/30c71bb6-172b-41f5-be33-8d7bf181212e">
    <img alt="time spent" src="https://wakatime.com/badge/user/2a9c42cf-576a-484e-962a-e346e913d9ac/project/30c71bb6-172b-41f5-be33-8d7bf181212e.svg" />
  </a>
</p>

This project contains the library `pkpass`, the toolkit cli `pkp` (`pkpass-cli`) and a server `pkpass-server`.

# Usage

## Generate a PKCS#12 archive to sign passes

```sh
# Generate a RSA private key
pkp crypto key --output pkpass.key
# Create a certificate signing request with this key or your own
pkp crypto request --private-key pkpass.key --output pkpass.csr
# and submit your certificate signing request to Apple.

# Create an archive containing all you need to sign
pkp crypto bundle --private-key pkpass.key --certificate path/to/pass.cer --output pkpass.p12
```

---

Work is licensed under [`CECILL-2.1`](https://choosealicense.com/licenses/cecill-2.1/), a French OSS license that allows modification and distribution of the software while requiring the same license for derived works.
