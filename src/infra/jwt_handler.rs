use crate::domain::interface::IJWTHandler;
use serde::*;

#[derive(Clone)]
pub struct JWTHandler {
    private_key: Vec<u8>,
    issuer: String,
}

impl JWTHandler {
    pub fn new(private_key: Vec<u8>) -> JWTHandler {
        JWTHandler {
            private_key: private_key,
            issuer: "example.com".to_owned(),
        }
    }
}

impl<Payload: Serialize + serde::de::DeserializeOwned> IJWTHandler<Payload> for JWTHandler {
    fn sign(&self, payload: Payload) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::ES384),
            &payload,
            self.private_key.as_slice(),
        )
    }

    fn verify(&self, jwt: &str) -> Result<Payload, jsonwebtoken::errors::Error> {
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::ES384);
        validation.iss = Some(self.issuer.clone());

        Ok(jsonwebtoken::decode(jwt, self.private_key.as_slice(), &validation)?.claims)
    }
}
