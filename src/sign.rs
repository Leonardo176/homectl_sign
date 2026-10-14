use anyhow::{Context, Result};
use base64::prelude::*;
use ed25519_dalek::{
    Signer, SigningKey,
    pkcs8::{DecodePrivateKey, EncodePublicKey, spki::der::pem::LineEnding},
};
use serde_json::{Value, json, to_string_pretty};

const FIELDS_TO_NOT_SIGN: [&str; 4] = ["binding", "status", "signature", "secret"];

fn normalize(identity: &str) -> Result<String> {
    let identity: Value =
        serde_json::from_str(identity).context("Failed to parse identity file")?;
    let mut filtered_id = json!({});

    for (key, value) in identity
        .as_object()
        .context("Identity file is not an object")?
        .iter()
    {
        let mut to_sign = true;

        for field in FIELDS_TO_NOT_SIGN {
            if key == field {
                to_sign = false;
            }
        }

        if to_sign {
            filtered_id[key] = value.clone();
        }
    }

    filtered_id.sort_all_objects();

    Ok(filtered_id.to_string())
}

pub fn sign_identity(identity: &str, pem_signing_key: &str) -> Result<String> {
    let identity = normalize(identity)?;

    let signing_key =
        SigningKey::from_pkcs8_pem(pem_signing_key).context("Failed to parse signing key")?;
    let signature = signing_key.sign(identity.as_bytes());
    let encoded_signature = BASE64_STANDARD.encode(signature.to_bytes());

    let mut new_id: Value = serde_json::from_str(&identity)?;
    new_id["signature"] = json!([{
        "data": encoded_signature,
        "key": &signing_key.verifying_key().to_public_key_pem(LineEnding::LF)?
    }]);

    new_id.sort_all_objects();

    Ok(to_string_pretty(&new_id)?)
}
