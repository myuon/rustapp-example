use crate::domain::interface::IJWTHandler;
use serde::*;

fn from_private(key: biscuit::jws::Secret) -> biscuit::jws::Secret {
    match key {
        biscuit::jws::Secret::EcdsaKeyPair(p) => biscuit::jws::Secret::PublicKey(
            ring::signature::KeyPair::public_key(p.as_ref())
                .as_ref()
                .to_vec(),
        ),
        _ => unimplemented!(),
    }
}

#[derive(Clone)]
pub struct JWTHandler {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    issuer: String,
}

impl JWTHandler {
    pub fn new(private_key: Vec<u8>, public_key: Vec<u8>) -> JWTHandler {
        JWTHandler {
            private_key: private_key,
            public_key: public_key,
            issuer: "example.com".to_owned(),
        }
    }
}

impl<Payload: Serialize + serde::de::DeserializeOwned + Clone> IJWTHandler<Payload> for JWTHandler {
    fn sign(&self, payload: Payload) -> Result<String, biscuit::errors::Error> {
        let key = biscuit::jws::Secret::ecdsa_keypair_from_file(
            biscuit::jwa::SignatureAlgorithm::ES384,
            "key/secp384r1.priv.key",
        )?;
        let jwt = biscuit::JWT::new_decoded(
            From::from(biscuit::jws::Header::from_registered_header(
                biscuit::jws::RegisteredHeader {
                    algorithm: biscuit::jwa::SignatureAlgorithm::ES384,
                    ..std::default::Default::default()
                },
            )),
            biscuit::ClaimsSet {
                private: payload,
                registered: std::default::Default::default(),
            },
        );
        let token = jwt.into_encoded(&key)?.unwrap_encoded().to_string();
        warn!("{:?}", token);

        Ok(token)
    }

    fn verify(&self, jwt: &str) -> Result<Payload, frank_jwt::Error> {
        let key = from_private(
            biscuit::jws::Secret::ecdsa_keypair_from_file(
                biscuit::jwa::SignatureAlgorithm::ES384,
                "key/secp384r1.priv.key",
            )
            .unwrap(),
        );
        let bjwt: biscuit::JWT<Payload, biscuit::Empty> = biscuit::JWT::new_encoded(jwt);
        let bjwt = bjwt
            .into_decoded(&key, biscuit::jwa::SignatureAlgorithm::ES384)
            .unwrap();
        bjwt.validate(std::default::Default::default()).unwrap();

        Ok(bjwt.payload().unwrap().private.clone())
    }
}
