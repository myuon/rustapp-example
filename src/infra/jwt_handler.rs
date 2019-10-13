use crate::domain::interface::IJWTHandler;
use serde::*;
use std::sync::Arc;

fn from_private(key: &biscuit::jws::Secret) -> biscuit::jws::Secret {
    match key {
        biscuit::jws::Secret::EcdsaKeyPair(p) => biscuit::jws::Secret::PublicKey(
            ring::signature::KeyPair::public_key(p.as_ref())
                .as_ref()
                .to_vec(),
        ),
        _ => unimplemented!(),
    }
}

struct KeyPair {
    public: biscuit::jws::Secret,
    private: biscuit::jws::Secret,
}

#[derive(Clone)]
pub struct JWTHandler {
    keypair: Arc<KeyPair>,
    issuer: String,
}

impl JWTHandler {
    pub fn new(private_key_file: &str) -> JWTHandler {
        let private = biscuit::jws::Secret::ecdsa_keypair_from_file(
            biscuit::jwa::SignatureAlgorithm::ES384,
            private_key_file,
        )
        .unwrap();
        let public = from_private(&private);

        JWTHandler {
            keypair: Arc::new(KeyPair {
                private: private,
                public: public,
            }),
            issuer: "example.com".to_owned(),
        }
    }
}

impl<Payload: Serialize + serde::de::DeserializeOwned + Clone> IJWTHandler<Payload> for JWTHandler {
    fn sign(&self, payload: Payload) -> Result<String, biscuit::errors::Error> {
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
        let token = jwt
            .into_encoded(&self.keypair.private)?
            .unwrap_encoded()
            .to_string();

        Ok(token)
    }

    fn verify(&self, jwt: &str) -> Result<Payload, biscuit::errors::Error> {
        let bjwt: biscuit::JWT<Payload, biscuit::Empty> = biscuit::JWT::new_encoded(jwt);
        let bjwt = bjwt.into_decoded(
            &self.keypair.public,
            biscuit::jwa::SignatureAlgorithm::ES384,
        )?;
        bjwt.validate(std::default::Default::default())?;

        Ok(bjwt.payload()?.private.clone())
    }
}
