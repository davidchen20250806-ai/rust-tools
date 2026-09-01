use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SqlRequest { pub sql: String }

#[derive(Deserialize)]
pub struct DiffRequest { pub old: String, pub new: String }
#[derive(Serialize)]
pub struct DiffChunk { pub tag: String, pub text: String }
#[derive(Serialize)]
pub struct DiffResponse { pub chunks: Vec<DiffChunk> }

#[derive(Deserialize)]
pub struct CronRequest { pub cron: String }
#[derive(Serialize)]
pub struct CronResponse { pub valid: bool, pub next_runs: Vec<String>, pub error: String }

#[derive(Deserialize)]
pub struct SubnetRequest { pub ip: String, pub cidr: u8 }
#[derive(Serialize)]
pub struct SubnetResponse { pub valid: bool, pub ip: String, pub cidr: String, pub mask: String, pub wildcard: String, pub network: String, pub broadcast: String, pub first_ip: String, pub last_ip: String, pub total_hosts: u64, pub usable_hosts: u64, pub ip_class: String, pub ip_type: String, pub binary_ip: String, pub binary_mask: String }

#[derive(Deserialize)]
pub struct JwtRequest { pub token: String }
#[derive(Serialize)]
pub struct JwtResponse { pub error: Option<String>, pub header: String, pub payload: String }

#[derive(Serialize)]
pub struct GenericResponse { pub result: String }

#[derive(Serialize)]
pub struct WhoamiResponse { pub ip: String, pub country: String, pub city: String, pub asn: String, pub user_agent: String, pub headers: std::collections::HashMap<String, String> }
