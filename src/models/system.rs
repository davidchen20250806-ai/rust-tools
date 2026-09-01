use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ChmodRequest { pub octal: String, pub file: String }
#[derive(Serialize)]
pub struct ChmodResponse { pub valid: bool, pub command: String }
#[derive(Deserialize)]
pub struct TarRequest { pub op: String, pub comp: String, pub verbose: bool, pub archive: String, pub files: String }
#[derive(Serialize)]
pub struct TarResponse { pub command: String }
#[derive(Deserialize)]
pub struct PsRequest { pub format: String, pub sort: String, pub tree: bool, pub filter: String, pub wide: bool, pub threads: bool, pub user: String, pub pid: String }
#[derive(Serialize)]
pub struct PsResponse { pub command: String }
#[derive(Deserialize)]
pub struct TcpdumpRequest { pub interface: String, pub protocol: String, pub host: String, pub port: String, pub verbose: bool, pub ascii: bool, pub hex: bool, pub write_file: String, pub count: String }
#[derive(Serialize)]
pub struct TcpdumpResponse { pub command: String }
#[derive(Deserialize)]
pub struct GitRequest { pub cmd: String, pub target: String, pub msg: String, pub remote: String, pub branch: String, pub opt_force: bool, pub opt_rebase: bool, pub opt_all: bool, pub opt_amend: bool, pub opt_hard: bool, pub opt_new_branch: bool, pub opt_tags: bool, pub opt_oneline: bool, pub opt_graph: bool }
#[derive(Serialize)]
pub struct GitResponse { pub command: String }
#[derive(Deserialize)]
pub struct GitCmdRequest { pub action: String, pub tag: String, pub msg: String, pub branch: String }
#[derive(Serialize)]
pub struct GitCmdResponse { pub command: String, pub description: String }
#[derive(Deserialize)]
pub struct StraceRequest { pub target: String, pub is_pid: bool, pub follow: bool, pub summary: bool, pub output_file: String, pub filter: String, pub string_limit: String, pub timestamp: bool }
#[derive(Serialize)]
pub struct StraceResponse { pub command: String }
#[derive(Deserialize)]
pub struct IostatRequest { pub interval: String, pub count: String, pub human: bool, pub extended: bool, pub unit: String, pub partitions: bool, pub timestamp: bool, pub device: String }
#[derive(Serialize)]
pub struct IostatResponse { pub command: String }
#[derive(Deserialize)]
pub struct NiceRequest { pub mode: String, pub priority: i32, pub command: String, pub target_type: String, pub target: String }
#[derive(Serialize)]
pub struct NiceResponse { pub command: String }
#[derive(Deserialize)]
pub struct LsRequest { pub path: String, pub all: bool, pub long: bool, pub human: bool, pub time: bool, pub reverse: bool, pub recursive: bool, pub inode: bool, pub directory: bool, pub color: bool }
#[derive(Serialize)]
pub struct LsResponse { pub command: String }
#[derive(Deserialize)]
pub struct FirewallRequest { pub op: String, pub zone: String, pub target_type: String, pub target: String, pub permanent: bool }
#[derive(Serialize)]
pub struct FirewallResponse { pub command: String }
#[derive(Deserialize)]
pub struct SystemctlRequest { pub operation: String, pub service: String, pub user_mode: bool, pub now: bool, pub force: bool, pub global: bool }
#[derive(Serialize)]
pub struct SystemctlResponse { pub command: String }
#[derive(Deserialize)]
pub struct FindRequest { pub path: String, pub name: String, pub iname: bool, pub target_type: String, pub size: String, pub mtime: String, pub empty: bool, pub exec: String }
#[derive(Serialize)]
pub struct FindResponse { pub command: String }
#[derive(Deserialize, Serialize)]
pub struct DockerfileStage { #[serde(default)] pub image: String, #[serde(default, rename = "as")] pub as_: String, #[serde(default)] pub workdir: String, #[serde(default)] pub copy: String, #[serde(default)] pub run: String, #[serde(default)] pub env: String, #[serde(default)] pub expose: String, #[serde(default)] pub cmd: String, #[serde(default)] pub entrypoint: String, #[serde(default)] pub user: String, #[serde(default)] pub volume: String, #[serde(default)] pub arg: String, #[serde(default)] pub label: String, #[serde(default)] pub healthcheck: String }
#[derive(Deserialize)]
pub struct DockerfileRequest { pub stages: Vec<DockerfileStage> }
#[derive(Deserialize, Serialize, Clone)]
pub struct NginxLocation { pub path: String, pub proxy: String, pub root: String, pub spa: bool }
#[derive(Deserialize)]
pub struct NginxRequest { pub domain: String, pub port: u16, pub root: String, pub locations: Vec<NginxLocation>, pub upstream: String, pub https: bool, pub force_https: bool, pub ssl_cert: String, pub ssl_key: String, pub gzip: bool, pub client_max_body_size: String, pub keepalive_timeout: String, pub proxy_connect_timeout: String, pub proxy_read_timeout: String, pub proxy_send_timeout: String }
#[derive(Deserialize)]
pub struct RsyncRequest { pub source: String, pub user: String, pub host: String, pub port: String, pub remote_path: String, pub archive: bool, pub compress: bool, pub verbose: bool, pub delete: bool, pub dry_run: bool, pub progress: bool, pub ssh: bool, pub exclude: String }
#[derive(Serialize)]
pub struct RsyncResponse { pub command: String, pub ssh_config: String }
#[derive(Deserialize)]
pub struct CurlRequest { pub method: String, pub url: String, pub headers: String, pub body: String }
#[derive(Serialize)]
pub struct CurlResponse { pub command: String, pub python: String }
#[derive(Deserialize)]
pub struct AwkRequest { pub separator: String, pub variable: String, pub code: String, pub file: String }
#[derive(Serialize)]
pub struct AwkResponse { pub command: String }
#[derive(Deserialize)]
pub struct SedRequest { pub operation: String, pub pattern: String, pub replacement: String, pub flags: String, pub inplace: bool, pub file: String }
#[derive(Serialize)]
pub struct SedResponse { pub command: String }
