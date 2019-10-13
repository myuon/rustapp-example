# rustapp-example

## build

```sh
$ rustup override add nightly-2019-10-04
$ cargo install diesel_cli --no-default-features --features mysql
```

## JWT key

This app uses P384 ECDSA private Key in PKCS8 DER format.

```sh
$ openssl ecparam -genkey -name secp384r1 | openssl pkcs8 -topk8 -nocrypt -outform DER > key/secp384r1.priv.key
```
