use crate::domain::interface::IJWTHandler;
use serde::*;

#[derive(Clone)]
pub struct JWTHandler {
    private_key: String,
    issuer: String,
}

impl JWTHandler {
    pub fn new(private_key: String) -> JWTHandler {
        JWTHandler {
            private_key: private_key,
            issuer: "example.com".to_owned(),
        }
    }
}

impl<Payload: Serialize + serde::de::DeserializeOwned> IJWTHandler<Payload> for JWTHandler {
    fn sign(&self, payload: Payload) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::sign(
            &serde_json::to_string(&payload)?,
            self.private_key.as_bytes(),
            jsonwebtoken::Algorithm::ES384,
        )
    }

    fn verify(&self, jwt: &str) -> Result<Payload, jsonwebtoken::errors::Error> {
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::ES384);
        validation.iss = Some(self.issuer.clone());

        Ok(jsonwebtoken::decode(jwt, self.private_key.as_bytes(), &validation)?.claims)
    }
}
