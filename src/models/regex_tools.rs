use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegexRequest { pub pattern: String, pub text: String }
#[derive(Serialize)]
pub struct RegexResponse { pub matches: Vec<String>, pub count: usize, pub error: Option<String> }

#[derive(Deserialize)]
pub struct RegexGenRequest { pub key: String }
#[derive(Serialize)]
pub struct RegexPatternResponse { pub pattern: String }

#[derive(Deserialize)]
pub struct RegexBuildRequest { pub starts_with: String, pub not_starts_with: String, pub ends_with: String, pub not_ends_with: String, pub contains: String, pub not_contains: String }
#[derive(Serialize)]
pub struct RegexBuildResponse { pub pattern: String, pub error: Option<String> }
