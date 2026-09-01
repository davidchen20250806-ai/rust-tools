use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Base64Request { pub text: String, pub action: String }
#[derive(Deserialize)]
pub struct JsEncRequest { pub js: String }
#[derive(Deserialize)]
pub struct JsonRequest { pub input: String }
#[derive(Serialize)]
pub struct JsonResponse { pub pretty: String, pub minified: String, pub error: Option<String> }
#[derive(Deserialize)]
pub struct UrlRequest { pub input: String }
#[derive(Serialize)]
pub struct UrlResponse { pub encoded: String, pub decoded: String, pub protocol: String, pub host: String, pub path: String, pub params: Vec<(String, String)> }
#[derive(Deserialize)]
pub struct YamlRequest { pub yaml: String }
#[derive(Deserialize)]
pub struct TomlRequest { pub toml: String }
#[derive(Serialize)]
pub struct YamlResponse { pub result: String, pub error: Option<String> }
#[derive(Deserialize)]
pub struct EscapeRequest { pub text: String, pub mode: String }
#[derive(Deserialize)]
pub struct CaseRequest { pub text: String, pub mode: String }
#[derive(Deserialize)]
pub struct UnitRequest { pub value: String, #[serde(rename = "type")] pub type_: String, pub from: String, pub to: String }
#[derive(Serialize)]
pub struct UnitResponse { pub result: f64, pub value: f64, pub from: String, pub to: String, #[serde(rename = "type")] pub type_: String }
#[derive(Deserialize)]
pub struct ColorRequest { pub input: String }
#[derive(Serialize)]
pub struct ColorResponse { pub valid: bool, pub hex: String, pub rgb: String, pub hsl: String, pub cmyk: String }
#[derive(Deserialize)]
pub struct DateRequest { pub input: String }
#[derive(Serialize)]
pub struct DateResponse { pub valid: bool, pub unix_sec: i64, pub unix_milli: i64, pub iso_8601: String, pub human_utc: String, pub error: Option<String> }
