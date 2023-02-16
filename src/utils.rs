use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: Vec<String>,
    exp: usize,
    iat: usize,
    iss: String,
    sub: String,
    scope: String,
}

#[derive(Debug)]
struct JWK {
    n: String,
    e: String,
}

fn get_claims(token: &str) {
    let jwk = JWK {
        n: "2WaOSAkuosGqVHqTdEVN92OxWmXH4tllS1m80gOFL8X7Ee-kXfakoe83QjYq8uCfvRQM8VL07dhp1-A9ug14xaDd-mmE2WCk77BvwpS5tYidQGiPYCCF9rcwc0_H1QLFjwWII_S6I1xVqASD0cq1-ByvKugqkk03Qc9euS_U_OEpFga3HeowWQ-xzIq46QIOovxuYGf18WO3O31r-slqentFEAfE0xoSrIKADGE3GqZngQrHVP54rnm3KA0OXm3NRo-czgMASGBsdUG6w9T1TJuAAtXNYXwnrZed9-6u88OI830C5STOuHCIxundU_aL9yHdx5HIdOE_kcfph4Owew".to_string(),
        e: "AQAB".to_string(),
    };
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&["https://oliverflecke.eu.auth0.com/"]);

    let token = decode::<Claims>(
        token,
        &DecodingKey::from_rsa_components(jwk.n.as_ref(), jwk.e.as_ref()).unwrap(),
        &validation,
    )
    .unwrap();

    println!("{token:?}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_token() {
        let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjBVTVplWW5mWGYxMFlHcFUtVVZTUyJ9.eyJpc3MiOiJodHRwczovL29saXZlcmZsZWNrZS5ldS5hdXRoMC5jb20vIiwic3ViIjoiZ2l0aHVifDcyMjc2NTgiLCJhdWQiOlsiaHR0cHM6Ly9maW5hbmNlLm9saXZlcmZsZWNrZS5tZS8iLCJodHRwczovL29saXZlcmZsZWNrZS5ldS5hdXRoMC5jb20vdXNlcmluZm8iXSwiaWF0IjoxNjc2NTUyNjU0LCJleHAiOjE2NzY2MzkwNTQsImF6cCI6InZCTXhsSFh3Ym5FQ0RyWUtZaUt2c1dxUnhhSjAyVFdmIiwic2NvcGUiOiJvcGVuaWQgYWNjb3VudDpyZWFkIn0.L3FCpDCtoO4Cf5DBE1q0CG1_K3gS--736Zot1Ypg9V-cEm59aCC6nMtEHWcQsLiH6VbKKm3snz5lUpIJAIflFPMvCxaIFPGO9kvRQjJ0-1YRRyuqhOFQAbhEVdCZZ4JPFfbCK2UhGifjfRYl0uKHW0QUU_0pluMRy62yP5fCvGJx1ryXNiuzqUZFraeDacfAfNascAghA9LqQsWOXsyXmxEaLoiJuu-dT3YE_5HwJRNYOf8H4BQZSf17L1W0TbAxsi_Skn5-h54tglsCno9JCTCfonJ8K4_QzjxJTXcdetWja41SeEHl3MmH-FWeg7gvM7ErsEZ7pz3GDxrgrmmeuA";

        get_claims(token);
    }
}
