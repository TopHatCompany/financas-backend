use crate::types::ErrorMessage;
use actix_web::{
    dev::Payload,
    http::{StatusCode, Uri},
    web, Error, FromRequest, HttpRequest, HttpResponse, ResponseError,
};
use actix_web_httpauth::{
    extractors::bearer::BearerAuth, headers::www_authenticate::bearer::Bearer,
};
use derive_more::derive::Display;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{self, AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use log::{debug, info};
use serde::Deserialize;
use std::{
    collections::HashSet,
    env::{self, temp_dir},
    future::Future,
    io::Write,
    path::Path,
    pin::Pin,
    string,
};

#[derive(Clone, Deserialize, Debug)]
pub struct Auth0Config {
    audience: String,
    domain: String,
}

impl Default for Auth0Config {
    fn default() -> Self {
        envy::prefixed("AUTH0_")
            .from_env::<Auth0Config>()
            .expect("Provide missing environment variables for Auth0Client")
    }
}

#[derive(Debug, Display)]
enum ClientError {
    #[display("authentication")]
    Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),
    #[display("decode")]
    Decode(jsonwebtoken::errors::Error),
    #[display("not_found")]
    NotFound(String),
    #[display("unsupported_algorithm")]
    UnsupportedAlgortithm(AlgorithmParameters),
}

#[derive(Debug, Display)]
enum ServerError {
    #[display("validation_error")]
    ValidationError(String), // errors related to signature validation (catch-all; fetching /jwks.json)
}
impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            _ => HttpResponse::InternalServerError().json("Internal Server Error"),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
impl ResponseError for ClientError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
            Self::Decode(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(
                    "Authorization header value must follow this format: Bearer access-token"
                        .to_string(),
                ),
                message: "Bad credentials".to_string(),
            }),
            Self::NotFound(msg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(msg.to_string()),
                message: "Bad credentials".to_string(),
            }),
            Self::UnsupportedAlgortithm(alg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(format!(
                    "Unsupported encryption algortithm expected RSA got {:?}",
                    alg
                )),
                message: "Bad credentials".to_string(),
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    // #[serde(rename = "sub")] // appears as "sub" in JWT
    // pub auth0_id: String,
    pub sub: String,
    _permissions: Option<HashSet<String>>,
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let client = req.app_data::<web::Data<awc::Client>>().unwrap().clone();
        let config = req.app_data::<web::Data<Auth0Config>>().unwrap().clone();
        let extractor = BearerAuth::extract(req);
        debug!("{:?}", extractor);
        debug!("{:?}", config);
        Box::pin(async move {
            let credentials = extractor.await.map_err(ClientError::Authentication)?;
            let token = credentials.token();
            let header = decode_header(token).map_err(ClientError::Decode)?;
            let kid = header.kid.ok_or_else(|| {
                ClientError::NotFound("kid not found in token header".to_string())
            })?;
            let domain = config.domain.as_str();
            // JSON Web Key Sets
            // fetch_latest_jwks_json() only if:
            // 1) ./jwks.json doesn't exist
            // 2) ./jwks.json exists but old
            let mut jwks_path = std::env::temp_dir();
            jwks_path.push("./jwks.json");
            if !Path::new(&jwks_path).exists() {
                info!("Backend is making a request to /.well-known/jwks.json");
                let mut new_cached_file =
                    std::fs::File::create(&jwks_path).expect("fail to open file");
                let jwks_response_bytes = client
                    .get(
                        Uri::builder()
                            .scheme("https")
                            .authority(domain)
                            .path_and_query("/.well-known/jwks.json")
                            .build()
                            .unwrap(),
                    )
                    .insert_header(("Accept", "application/json"))
                    .send()
                    .await
                    .map_err(|send_request_error| {
                        debug!("falhou aqui");
                        ServerError::ValidationError(send_request_error.to_string())
                    })?
                    .body()
                    .await
                    .map_err(|payload_error| {
                        debug!("falhou aqui2");
                        ServerError::ValidationError(payload_error.to_string())
                    })?;
                new_cached_file.write_all(&jwks_response_bytes)?;
            }
            let cached_file = std::fs::read_to_string(jwks_path).unwrap(); // cached_file should be Ok(_) by now
            let var_name = serde_json::from_str(&cached_file);
            let jwks: JwkSet = var_name
                .map_err(|serde_error| ServerError::ValidationError(serde_error.to_string()))?;
            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| ClientError::NotFound("No JWK found for kid".to_string()))?;
            match jwk.clone().algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let mut validation = Validation::new(Algorithm::RS256);
                    debug!("{:?}", rsa);
                    validation.set_audience(&[config.audience.clone()]);
                    validation.set_issuer(&[Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/")
                        .build()
                        .unwrap()]);
                    let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .map_err(ClientError::Decode)?;
                    debug!("token: {:?}", token);
                    let token =
                        decode::<Claims>(token, &key, &validation).map_err(ClientError::Decode)?;

                    debug!("decoded token: {:?}", token);
                    Ok(token.claims)
                }
                algorithm => Err(ClientError::UnsupportedAlgortithm(algorithm).into()),
            }
        })
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}
