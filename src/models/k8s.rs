use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct K8sRequest {
    #[serde(default = "default_kind")]
    pub kind: String,
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_namespace")]
    pub namespace: String,
    #[serde(default = "default_image")]
    pub image: String,
    #[serde(default = "default_replicas")]
    pub replicas: i32,
    #[serde(default = "default_port")]
    pub port: i32,
    #[serde(default = "default_target_port")]
    pub target_port: i32,
    #[serde(default = "default_service_type")]
    pub service_type: String,
    #[serde(default = "default_ingress_host")]
    pub ingress_host: String,
    #[serde(default = "default_ingress_path")]
    pub ingress_path: String,
    #[serde(default = "default_pull_policy")]
    pub pull_policy: String,
    pub cpu_limit: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_request: Option<String>,
    pub memory_request: Option<String>,
    #[serde(default)]
    pub env: Vec<K8sEnvVar>,
    #[serde(default = "default_schedule")]
    pub schedule: String,
    #[serde(default = "default_restart_policy")]
    pub restart_policy: String,
}

#[derive(Deserialize, Serialize)]
pub struct K8sEnvVar {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct K8sCmdRequest {
    pub action: String,
    #[serde(default = "default_namespace")]
    pub namespace: String,
    #[serde(default = "default_resource_type")]
    pub resource_type: String,
    #[serde(default)]
    pub resource_name: String,
    #[serde(default)]
    pub replicas: i32,
    #[serde(default)]
    pub local_port: i32,
    #[serde(default)]
    pub remote_port: i32,
    #[serde(default)]
    pub output_format: String,
}

#[derive(Serialize)]
pub struct K8sCmdResponse {
    pub command: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnsibleRequest {
    #[serde(default = "default_play_name")]
    pub play_name: String,
    #[serde(default = "default_hosts")]
    pub hosts: String,
    #[serde(default)]
    pub r#become: bool,
    #[serde(default = "default_true")]
    pub gather_facts: bool,
    #[serde(default)]
    pub vars: String,
    #[serde(default)]
    pub tasks: String,
    #[serde(default)]
    pub handlers: String,
}

pub fn default_kind() -> String { "Deployment".to_string() }
pub fn default_name() -> String { "app-name".to_string() }
pub fn default_namespace() -> String { "default".to_string() }
pub fn default_image() -> String { "nginx:latest".to_string() }
pub fn default_replicas() -> i32 { 1 }
pub fn default_port() -> i32 { 80 }
pub fn default_target_port() -> i32 { 80 }
pub fn default_service_type() -> String { "ClusterIP".to_string() }
pub fn default_ingress_host() -> String { "example.com".to_string() }
pub fn default_ingress_path() -> String { "/".to_string() }
pub fn default_pull_policy() -> String { "IfNotPresent".to_string() }
pub fn default_schedule() -> String { "*/1 * * * *".to_string() }
pub fn default_restart_policy() -> String { "Always".to_string() }
pub fn default_play_name() -> String { "Ansible Playbook".to_string() }
pub fn default_hosts() -> String { "all".to_string() }
pub fn default_resource_type() -> String { "pod".to_string() }
pub fn default_true() -> bool { true }

pub fn generate_k8s_yaml(data: &K8sRequest) -> String {
    let mut yaml = String::new();

    if data.kind == "Deployment" {
        yaml.push_str(&format!(
            r#"apiVersion: apps/v1
kind: Deployment
metadata:
  name: {}
  namespace: {}
  labels:
    app: {}
spec:
  replicas: {}
  selector:
    matchLabels:
      app: {}
  template:
    metadata:
      labels:
        app: {}
    spec:
      containers:
      - name: {}
        image: {}
        imagePullPolicy: {}
        ports:
        - containerPort: {}
"#,
            data.name,
            data.namespace,
            data.name,
            data.replicas,
            data.name,
            data.name,
            data.name,
            data.image,
            data.pull_policy,
            data.port
        ));

        let has_cpu_limit = data.cpu_limit.as_ref().filter(|s| !s.is_empty()).is_some();
        let has_mem_limit = data
            .memory_limit
            .as_ref()
            .filter(|s| !s.is_empty())
            .is_some();
        let has_cpu_req = data
            .cpu_request
            .as_ref()
            .filter(|s| !s.is_empty())
            .is_some();
        let has_mem_req = data
            .memory_request
            .as_ref()
            .filter(|s| !s.is_empty())
            .is_some();

        if has_cpu_limit || has_mem_limit || has_cpu_req || has_mem_req {
            yaml.push_str("        resources:\n");
            if has_cpu_limit || has_mem_limit {
                yaml.push_str("          limits:\n");
                if has_cpu_limit {
                    yaml.push_str(&format!(
                        "            cpu: {}\n",
                        data.cpu_limit.as_ref().unwrap()
                    ));
                }
                if has_mem_limit {
                    yaml.push_str(&format!(
                        "            memory: {}\n",
                        data.memory_limit.as_ref().unwrap()
                    ));
                }
            }
            if has_cpu_req || has_mem_req {
                yaml.push_str("          requests:\n");
                if has_cpu_req {
                    yaml.push_str(&format!(
                        "            cpu: {}\n",
                        data.cpu_request.as_ref().unwrap()
                    ));
                }
                if has_mem_req {
                    yaml.push_str(&format!(
                        "            memory: {}\n",
                        data.memory_request.as_ref().unwrap()
                    ));
                }
            }
        }

        let valid_env: Vec<&K8sEnvVar> = data
            .env
            .iter()
            .filter(|e| !e.key.is_empty() && !e.value.is_empty())
            .collect();

        if !valid_env.is_empty() {
            yaml.push_str("        env:\n");
            for e in valid_env {
                yaml.push_str(&format!(
                    "        - name: {}\n          value: \"{}\"\n",
                    e.key, e.value
                ));
            }
        }

        yaml.push_str(&format!("      restartPolicy: {}", data.restart_policy));
    } else if data.kind == "Service" {
        yaml.push_str(&format!(
            r#"apiVersion: v1
kind: Service
metadata:
  name: {}
  namespace: {}
  labels:
    app: {}
spec:
  type: {}
  selector:
    app: {}
  ports:
  - protocol: TCP
    port: {}
    targetPort: {}
"#,
            data.name,
            data.namespace,
            data.name,
            data.service_type,
            data.name,
            data.port,
            data.target_port
        ));
    } else if data.kind == "Ingress" {
        yaml.push_str(&format!(
            r#"apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: {}
  namespace: {}
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  rules:
  - host: {}
    http:
      paths:
      - path: {}
        pathType: Prefix
        backend:
          service:
            name: {}
            port:
              number: {}
"#,
            data.name, data.namespace, data.ingress_host, data.ingress_path, data.name, data.port
        ));
    } else if data.kind == "CronJob" {
        yaml.push_str(&format!(
            r#"apiVersion: batch/v1
kind: CronJob
metadata:
  name: {}
  namespace: {}
spec:
  schedule: "{}"
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: {}
            image: {}
            imagePullPolicy: {}
            command:
            - /bin/sh
            - -c
            - "echo Hello Kubernetes"
          restartPolicy: OnFailure
"#,
            data.name, data.namespace, data.schedule, data.name, data.image, data.pull_policy
        ));
    } else if data.kind == "ConfigMap" {
        yaml.push_str(&format!(
            r#"apiVersion: v1
kind: ConfigMap
metadata:
  name: {}
  namespace: {}
data:
"#,
            data.name, data.namespace
        ));
        if !data.env.is_empty() {
            for e in &data.env {
                if !e.key.is_empty() && !e.value.is_empty() {
                    yaml.push_str(&format!("  {}: \"{}\"\n", e.key, e.value));
                }
            }
        } else {
            yaml.push_str("  config.json: |\n    {\n      \"key\": \"value\"\n    }");
        }
    } else if data.kind == "Secret" {
        yaml.push_str(&format!(
            r#"apiVersion: v1
kind: Secret
metadata:
  name: {}
  namespace: {}
type: Opaque
data:
  # Data should be base64 encoded
"#,
            data.name, data.namespace
        ));
        if !data.env.is_empty() {
            use base64::{engine::general_purpose, Engine as _};
            for e in &data.env {
                if !e.key.is_empty() && !e.value.is_empty() {
                    let b64 = general_purpose::STANDARD.encode(&e.value);
                    yaml.push_str(&format!("  {}: {}\n", e.key, b64));
                }
            }
        } else {
            yaml.push_str("  username: YWRtaW4=");
        }
    }

    yaml
}

pub fn generate_k8s_cmd(data: &K8sCmdRequest) -> (String, String) {
    let ns = if data.namespace.is_empty() {
        "default"
    } else {
        &data.namespace
    };
    let name = if data.resource_name.is_empty() {
        "app"
    } else {
        &data.resource_name
    };
    let rtype = if data.resource_type.is_empty() {
        "pod"
    } else {
        &data.resource_type
    };

    let output = if data.output_format.is_empty() {
        String::new()
    } else {
        format!(" -o {}", data.output_format)
    };

    match data.action.as_str() {
        "get" => (
            format!("kubectl get {} -n {}{}", rtype, ns, output),
            format!("获取 {} 列表", rtype),
        ),
        "describe" => (
            format!("kubectl describe {} {} -n {}", rtype, name, ns),
            format!("查看 {} {} 的详细信息", rtype, name),
        ),
        "delete" => (
            format!("kubectl delete {} {} -n {}", rtype, name, ns),
            format!("删除 {} {}", rtype, name),
        ),
        "logs" => {
            let target = if rtype == "pod" {
                name.to_string()
            } else {
                format!("{}/{}", rtype, name)
            };
            (
                format!("kubectl logs -f {} -n {}", target, ns),
                format!("查看 {} {} 的日志", rtype, name),
            )
        }
        "exec" => {
            if rtype == "pod" {
                (
                    format!("kubectl exec -it {} -n {} -- /bin/sh", name, ns),
                    format!("进入 Pod {} 的 Shell", name),
                )
            } else {
                (
                    format!("# 错误: exec 命令仅适用于 Pod\n# 当前选择资源: {}", rtype),
                    format!("无法对 {} 执行 exec", rtype),
                )
            }
        }
        "scale" => {
            if [
                "deployment",
                "statefulset",
                "replicaset",
                "replicationcontroller",
            ]
            .contains(&rtype)
            {
                (
                    format!(
                        "kubectl scale {} {} --replicas={} -n {}",
                        rtype, name, data.replicas, ns
                    ),
                    format!("将 {} {} 伸缩到 {} 个副本", rtype, name, data.replicas),
                )
            } else {
                (
                    format!("# 错误: 资源类型 '{}' 不支持伸缩 (Scale)\n# 仅支持: Deployment, StatefulSet, ReplicaSet", rtype),
                    format!("无法对 {} 进行伸缩操作", rtype),
                )
            }
        }
        "port_forward" => (
            format!(
                "kubectl port-forward {} {}:{} -n {}",
                if rtype == "pod" {
                    name.to_string()
                } else {
                    format!("{}/{}", rtype, name)
                },
                data.local_port,
                data.remote_port,
                ns
            ),
            format!(
                "端口转发 {} {} -> {}",
                rtype, data.local_port, data.remote_port
            ),
        ),
        "rollout_restart" | "rollout_status" | "rollout_history" | "rollout_undo" => {
            if ["deployment", "statefulset", "daemonset"].contains(&rtype) {
                let (cmd_suffix, desc_prefix) = match data.action.as_str() {
                    "rollout_restart" => ("restart", "重启 (滚动更新)"),
                    "rollout_status" => ("status", "查看滚动更新状态"),
                    "rollout_history" => ("history", "查看历史版本"),
                    "rollout_undo" => ("undo", "回滚到上一个版本"),
                    _ => ("", ""),
                };
                (
                    format!(
                        "kubectl rollout {} {} {} -n {}",
                        cmd_suffix, rtype, name, ns
                    ),
                    format!("{} {} {}", desc_prefix, rtype, name),
                )
            } else {
                (
                    format!("# 错误: 资源类型 '{}' 不支持 Rollout 操作\n# 仅支持: Deployment, StatefulSet, DaemonSet", rtype),
                    format!("无法对 {} 执行 Rollout", rtype),
                )
            }
        }
        _ => ("kubectl --help".to_string(), "显示帮助信息".to_string()),
    }
}

pub fn generate_ansible_yaml(data: &AnsibleRequest) -> String {
    let mut yaml = String::new();
    yaml.push_str(&format!("- name: {}\n", data.play_name));
    yaml.push_str(&format!("  hosts: {}\n", data.hosts));
    if data.r#become {
        yaml.push_str("  become: yes\n");
    }
    if !data.gather_facts {
        yaml.push_str("  gather_facts: no\n");
    }
    if !data.vars.trim().is_empty() {
        yaml.push_str("  vars:\n");
        for line in data.vars.lines() {
            yaml.push_str(&format!("    {}\n", line));
        }
    }
    yaml.push_str("  tasks:\n");

    if data.tasks.trim().is_empty() {
        yaml.push_str("    - name: Ping hosts\n      ping:\n");
    } else {
        for line in data.tasks.lines() {
            yaml.push_str(&format!("    {}\n", line));
        }
    }

    if !data.handlers.trim().is_empty() {
        yaml.push_str("  handlers:\n");
        for line in data.handlers.lines() {
            yaml.push_str(&format!("    {}\n", line));
        }
    }
    yaml
}
