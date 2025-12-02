// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 conflow contributors

//! RSR Schema Registry
//!
//! Provides access to RSR schemas for validation and generation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::ConflowError;

/// Schema type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    /// CUE schema
    Cue,
    /// JSON Schema
    JsonSchema,
    /// Nickel contract
    Nickel,
}

/// Schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDefinition {
    /// Schema ID
    pub id: String,

    /// Schema type
    pub schema_type: SchemaType,

    /// Schema name
    pub name: String,

    /// Description
    pub description: String,

    /// Schema content (inline) or path
    pub source: SchemaSource,

    /// Version
    pub version: String,

    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Schema source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaSource {
    /// Inline schema content
    Inline { content: String },

    /// Path to schema file
    Path { path: PathBuf },

    /// URL to fetch schema
    Url { url: String },
}

/// RSR Schema Registry
pub struct RsrSchemaRegistry {
    schemas: HashMap<String, SchemaDefinition>,
    cache_dir: Option<PathBuf>,
}

impl RsrSchemaRegistry {
    /// Create a new schema registry
    pub fn new() -> Self {
        let mut registry = Self {
            schemas: HashMap::new(),
            cache_dir: None,
        };

        // Register built-in schemas
        registry.register_builtins();

        registry
    }

    /// Create with cache directory
    pub fn with_cache(cache_dir: PathBuf) -> Self {
        let mut registry = Self::new();
        registry.cache_dir = Some(cache_dir);
        registry
    }

    /// Register built-in RSR schemas
    fn register_builtins(&mut self) {
        // RSR Pipeline Schema
        self.schemas.insert(
            "rsr:pipeline".into(),
            SchemaDefinition {
                id: "rsr:pipeline".into(),
                schema_type: SchemaType::Cue,
                name: "RSR Pipeline Schema".into(),
                description: "Schema for .conflow.yaml pipeline definitions".into(),
                source: SchemaSource::Inline {
                    content: include_str!("../../cue/pipeline.cue").into(),
                },
                version: "1.0.0".into(),
                tags: vec!["conflow".into(), "pipeline".into()],
            },
        );

        // RSR Requirement Schema
        self.schemas.insert(
            "rsr:requirement".into(),
            SchemaDefinition {
                id: "rsr:requirement".into(),
                schema_type: SchemaType::Cue,
                name: "RSR Requirement Schema".into(),
                description: "Schema for RSR requirement definitions".into(),
                source: SchemaSource::Inline {
                    content: RSR_REQUIREMENT_SCHEMA.into(),
                },
                version: "1.0.0".into(),
                tags: vec!["rsr".into(), "requirement".into()],
            },
        );

        // RSR Config Schema
        self.schemas.insert(
            "rsr:config".into(),
            SchemaDefinition {
                id: "rsr:config".into(),
                schema_type: SchemaType::Cue,
                name: "RSR Configuration Schema".into(),
                description: "Schema for .rsr.yaml configuration files".into(),
                source: SchemaSource::Inline {
                    content: RSR_CONFIG_SCHEMA.into(),
                },
                version: "1.0.0".into(),
                tags: vec!["rsr".into(), "config".into()],
            },
        );

        // Kubernetes base schema
        self.schemas.insert(
            "k8s:base".into(),
            SchemaDefinition {
                id: "k8s:base".into(),
                schema_type: SchemaType::Cue,
                name: "Kubernetes Base Schema".into(),
                description: "Base schema for Kubernetes resources".into(),
                source: SchemaSource::Inline {
                    content: K8S_BASE_SCHEMA.into(),
                },
                version: "1.0.0".into(),
                tags: vec!["kubernetes".into(), "k8s".into()],
            },
        );

        // Terraform schema
        self.schemas.insert(
            "terraform:variables".into(),
            SchemaDefinition {
                id: "terraform:variables".into(),
                schema_type: SchemaType::Cue,
                name: "Terraform Variables Schema".into(),
                description: "Schema for Terraform variable definitions".into(),
                source: SchemaSource::Inline {
                    content: TERRAFORM_SCHEMA.into(),
                },
                version: "1.0.0".into(),
                tags: vec!["terraform".into(), "iac".into()],
            },
        );

        // Helm Values schema
        self.schemas.insert(
            "helm:values".into(),
            SchemaDefinition {
                id: "helm:values".into(),
                schema_type: SchemaType::Cue,
                name: "Helm Values Schema".into(),
                description: "Schema for Helm chart values.yaml files".into(),
                source: SchemaSource::Inline {
                    content: HELM_VALUES_SCHEMA.into(),
                },
                version: "1.0.0".into(),
                tags: vec!["helm".into(), "kubernetes".into()],
            },
        );

        // Docker Compose schema
        self.schemas.insert(
            "docker:compose".into(),
            SchemaDefinition {
                id: "docker:compose".into(),
                schema_type: SchemaType::Cue,
                name: "Docker Compose Schema".into(),
                description: "Schema for docker-compose.yaml files".into(),
                source: SchemaSource::Inline {
                    content: DOCKER_COMPOSE_SCHEMA.into(),
                },
                version: "1.0.0".into(),
                tags: vec!["docker".into(), "compose".into()],
            },
        );

        // GitHub Actions schema
        self.schemas.insert(
            "github:actions".into(),
            SchemaDefinition {
                id: "github:actions".into(),
                schema_type: SchemaType::Cue,
                name: "GitHub Actions Schema".into(),
                description: "Schema for GitHub Actions workflow files".into(),
                source: SchemaSource::Inline {
                    content: GITHUB_ACTIONS_SCHEMA.into(),
                },
                version: "1.0.0".into(),
                tags: vec!["github".into(), "ci".into()],
            },
        );

        // AWS CloudFormation schema
        self.schemas.insert(
            "aws:cloudformation".into(),
            SchemaDefinition {
                id: "aws:cloudformation".into(),
                schema_type: SchemaType::Cue,
                name: "AWS CloudFormation Schema".into(),
                description: "Schema for CloudFormation templates".into(),
                source: SchemaSource::Inline {
                    content: CLOUDFORMATION_SCHEMA.into(),
                },
                version: "1.0.0".into(),
                tags: vec!["aws".into(), "cloudformation".into(), "iac".into()],
            },
        );
    }

    /// Get a schema by ID
    pub fn get(&self, id: &str) -> Option<&SchemaDefinition> {
        self.schemas.get(id)
    }

    /// Get schema content
    pub fn get_content(&self, id: &str) -> Result<String, ConflowError> {
        let schema = self.schemas.get(id).ok_or_else(|| ConflowError::FileNotFound {
            path: PathBuf::from(id),
            help: Some("Schema not found in registry".into()),
        })?;

        match &schema.source {
            SchemaSource::Inline { content } => Ok(content.clone()),
            SchemaSource::Path { path } => {
                std::fs::read_to_string(path).map_err(|e| ConflowError::Io {
                    message: e.to_string(),
                })
            }
            SchemaSource::Url { url } => {
                // Would fetch from URL
                Err(ConflowError::ExecutionFailed {
                    message: format!("URL schemas not yet implemented: {}", url),
                    help: None,
                })
            }
        }
    }

    /// List all schemas
    pub fn list(&self) -> impl Iterator<Item = &SchemaDefinition> {
        self.schemas.values()
    }

    /// List schemas by tag
    pub fn by_tag(&self, tag: &str) -> Vec<&SchemaDefinition> {
        self.schemas
            .values()
            .filter(|s| s.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Register a custom schema
    pub fn register(&mut self, schema: SchemaDefinition) {
        self.schemas.insert(schema.id.clone(), schema);
    }

    /// Load schemas from a directory
    pub fn load_from_dir(&mut self, dir: &Path) -> Result<usize, ConflowError> {
        let mut count = 0;

        if !dir.exists() {
            return Ok(0);
        }

        for entry in std::fs::read_dir(dir).map_err(|e| ConflowError::Io {
            message: e.to_string(),
        })? {
            let entry = entry.map_err(|e| ConflowError::Io {
                message: e.to_string(),
            })?;

            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = std::fs::read_to_string(&path).map_err(|e| ConflowError::Io {
                    message: e.to_string(),
                })?;

                let schema: SchemaDefinition =
                    serde_yaml::from_str(&content).map_err(|e| ConflowError::Yaml {
                        message: e.to_string(),
                    })?;

                self.schemas.insert(schema.id.clone(), schema);
                count += 1;
            }
        }

        Ok(count)
    }

    /// Write schema to file
    pub fn write_to_file(&self, id: &str, path: &Path) -> Result<(), ConflowError> {
        let content = self.get_content(id)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ConflowError::Io {
                message: e.to_string(),
            })?;
        }

        std::fs::write(path, content).map_err(|e| ConflowError::Io {
            message: e.to_string(),
        })?;

        Ok(())
    }
}

impl Default for RsrSchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Built-in schema definitions

const RSR_REQUIREMENT_SCHEMA: &str = r#"
// RSR Requirement Schema
package rsr

#Requirement: {
    id:          string & =~"^RSR-[A-Z]+-[0-9]+$"
    name:        string
    class:       "mandatory" | "preferential" | "advisory"
    description: string

    validation: {
        file_exists?:   [...string]
        file_absent?:   [...string]
        patterns?:      [...#PatternCheck]
        cue_validate?:  [...#CueValidation]
        conflow_valid?: bool
        shell_check?:   string
    }

    remediation: {
        auto_fix?:      bool
        templates?:     [...#Template]
        manual_steps?:  [...string]
        docs_url?:      string
    }

    related?: [...string]
    tags?:    [...string]
}

#PatternCheck: {
    file:         string
    pattern:      string
    should_match: bool | *true
}

#CueValidation: {
    files:  [...string]
    schema: string
}

#Template: {
    name:             string
    description:      string
    conflow_template?: string
    generates?:       [...string]
}
"#;

const RSR_CONFIG_SCHEMA: &str = r#"
// RSR Configuration Schema
// For .rsr.yaml files
package rsr

#Config: {
    // RSR version
    version: "1" | *"1"

    // Project metadata
    project: {
        name:        string
        description?: string
        tier?:       1 | 2 | 3 | 4
    }

    // Requirements configuration
    requirements?: {
        // Skip specific requirements
        skip?: [...string]

        // Custom requirement definitions
        custom?: [...#Requirement]
    }

    // Integration settings
    integrations?: {
        conflow?: {
            enabled: bool | *true
            pipeline?: string
        }

        ci?: {
            provider?: "github" | "gitlab" | "jenkins"
            config?:   string
        }
    }

    // Compliance targets
    compliance?: {
        target_level?: "basic" | "good" | "excellent"
        exceptions?:   [...{
            requirement: string
            reason:      string
            expires?:    string
        }]
    }
}
"#;

const K8S_BASE_SCHEMA: &str = r#"
// Kubernetes Base Schema
package k8s

#Resource: {
    apiVersion: string
    kind:       string
    metadata:   #Metadata
}

#Metadata: {
    name:        string
    namespace?:  string
    labels?:     [string]: string
    annotations?: [string]: string
}

#Deployment: #Resource & {
    apiVersion: "apps/v1"
    kind:       "Deployment"
    spec:       #DeploymentSpec
}

#DeploymentSpec: {
    replicas?: int & >=0
    selector:  #Selector
    template:  #PodTemplateSpec
}

#Selector: {
    matchLabels: [string]: string
}

#PodTemplateSpec: {
    metadata: #Metadata
    spec:     #PodSpec
}

#PodSpec: {
    containers: [...#Container]
}

#Container: {
    name:  string
    image: string
    ports?: [...#ContainerPort]
    env?:   [...#EnvVar]
    resources?: #ResourceRequirements
}

#ContainerPort: {
    containerPort: int & >=1 & <=65535
    protocol?:     "TCP" | "UDP" | *"TCP"
}

#EnvVar: {
    name:  string
    value?: string
    valueFrom?: {
        secretKeyRef?: {
            name: string
            key:  string
        }
        configMapKeyRef?: {
            name: string
            key:  string
        }
    }
}

#ResourceRequirements: {
    limits?: {
        cpu?:    string
        memory?: string
    }
    requests?: {
        cpu?:    string
        memory?: string
    }
}
"#;

const TERRAFORM_SCHEMA: &str = r#"
// Terraform Variables Schema
package terraform

#Variables: {
    // AWS/Cloud region
    region?: string

    // Environment
    environment: "dev" | "staging" | "prod"

    // Instance type
    instance_type?: string | *"t3.micro"

    // Enable monitoring
    monitoring?: bool | *true

    // Tags
    tags?: [string]: string
}

#Backend: {
    bucket:         string
    key:            string
    region:         string
    encrypt?:       bool | *true
    dynamodb_table?: string
}

#Provider: {
    source:  string
    version: string
}
"#;

const HELM_VALUES_SCHEMA: &str = r#"
// Helm Values Schema
package helm

#Values: {
    // Replica count
    replicaCount?: int & >=0 | *1

    // Image configuration
    image?: {
        repository: string
        tag?:       string | *"latest"
        pullPolicy?: "Always" | "IfNotPresent" | "Never" | *"IfNotPresent"
    }

    // Image pull secrets
    imagePullSecrets?: [...{name: string}]

    // Service account
    serviceAccount?: {
        create?: bool | *true
        annotations?: [string]: string
        name?: string
    }

    // Service configuration
    service?: {
        type?: "ClusterIP" | "NodePort" | "LoadBalancer" | *"ClusterIP"
        port?: int & >=1 & <=65535 | *80
    }

    // Ingress configuration
    ingress?: {
        enabled?: bool | *false
        className?: string
        annotations?: [string]: string
        hosts?: [...{
            host: string
            paths?: [...{
                path: string
                pathType?: "Prefix" | "Exact" | "ImplementationSpecific"
            }]
        }]
        tls?: [...{
            secretName: string
            hosts: [...string]
        }]
    }

    // Resource limits
    resources?: {
        limits?: {
            cpu?:    string
            memory?: string
        }
        requests?: {
            cpu?:    string
            memory?: string
        }
    }

    // Autoscaling
    autoscaling?: {
        enabled?: bool | *false
        minReplicas?: int & >=1
        maxReplicas?: int & >=1
        targetCPUUtilizationPercentage?: int & >=1 & <=100
    }
}
"#;

const DOCKER_COMPOSE_SCHEMA: &str = r#"
// Docker Compose Schema
package compose

#Compose: {
    version?: string
    services: [string]: #Service
    volumes?: [string]: #Volume | null
    networks?: [string]: #Network | null
    configs?: [string]: #Config
    secrets?: [string]: #Secret
}

#Service: {
    image?:       string
    build?:       string | #Build
    container_name?: string
    command?:     string | [...string]
    entrypoint?:  string | [...string]
    ports?:       [...string | #Port]
    expose?:      [...(string | int)]
    environment?: [...string] | {[string]: string | null}
    env_file?:    string | [...string]
    volumes?:     [...string | #VolumeMount]
    depends_on?:  [...string] | {[string]: #DependsOn}
    networks?:    [...string] | {[string]: #NetworkConfig | null}
    restart?:     "no" | "always" | "on-failure" | "unless-stopped"
    healthcheck?: #Healthcheck
    deploy?:      #Deploy
    labels?:      [...string] | {[string]: string}
    logging?:     #Logging
    extra_hosts?: [...string]
    dns?:         string | [...string]
    working_dir?: string
    user?:        string
    privileged?:  bool
    stdin_open?:  bool
    tty?:         bool
}

#Build: {
    context:    string
    dockerfile?: string
    args?:      [...string] | {[string]: string}
    target?:    string
    cache_from?: [...string]
}

#Port: {
    target:    int
    published?: int | string
    protocol?:  "tcp" | "udp"
    mode?:     "host" | "ingress"
}

#VolumeMount: {
    type:     "volume" | "bind" | "tmpfs"
    source:   string
    target:   string
    read_only?: bool
}

#DependsOn: {
    condition: "service_started" | "service_healthy" | "service_completed_successfully"
}

#NetworkConfig: {
    aliases?: [...string]
    ipv4_address?: string
}

#Healthcheck: {
    test:        [...string]
    interval?:   string
    timeout?:    string
    retries?:    int
    start_period?: string
}

#Deploy: {
    replicas?: int
    resources?: {
        limits?: {
            cpus?: string
            memory?: string
        }
        reservations?: {
            cpus?: string
            memory?: string
        }
    }
    restart_policy?: {
        condition?: "none" | "on-failure" | "any"
        delay?:     string
        max_attempts?: int
        window?:    string
    }
}

#Logging: {
    driver: string
    options?: [string]: string
}

#Volume: {
    driver?: string
    driver_opts?: [string]: string
    external?: bool
    labels?: [string]: string
    name?: string
}

#Network: {
    driver?: string
    driver_opts?: [string]: string
    external?: bool
    internal?: bool
    labels?: [string]: string
    name?: string
}

#Config: {
    file?: string
    external?: bool
    name?: string
}

#Secret: {
    file?: string
    external?: bool
    name?: string
}
"#;

const GITHUB_ACTIONS_SCHEMA: &str = r#"
// GitHub Actions Workflow Schema
package github

#Workflow: {
    name?: string

    on: #Trigger | [...#Event] | {[#Event]: #TriggerConfig | null}

    env?: [string]: string

    defaults?: {
        run?: {
            shell?: string
            working-directory?: string
        }
    }

    concurrency?: string | {
        group: string
        cancel-in-progress?: bool
    }

    jobs: [string]: #Job
}

#Event: "push" | "pull_request" | "workflow_dispatch" | "schedule" |
        "release" | "issues" | "issue_comment" | "create" | "delete" |
        "fork" | "watch" | "workflow_call" | "repository_dispatch"

#Trigger: #Event | [...#Event]

#TriggerConfig: {
    branches?: [...string]
    branches-ignore?: [...string]
    paths?: [...string]
    paths-ignore?: [...string]
    tags?: [...string]
    tags-ignore?: [...string]
    types?: [...string]
}

#Job: {
    name?: string
    needs?: string | [...string]
    runs-on: string | [...string]

    if?: string

    permissions?: "read-all" | "write-all" | {
        actions?: #Permission
        contents?: #Permission
        deployments?: #Permission
        issues?: #Permission
        packages?: #Permission
        pull-requests?: #Permission
        security-events?: #Permission
        statuses?: #Permission
    }

    environment?: string | {
        name: string
        url?: string
    }

    concurrency?: string | {
        group: string
        cancel-in-progress?: bool
    }

    outputs?: [string]: string

    env?: [string]: string

    defaults?: {
        run?: {
            shell?: string
            working-directory?: string
        }
    }

    strategy?: {
        matrix?: [string]: [...] | {
            include?: [...{[string]: _}]
            exclude?: [...{[string]: _}]
            [string]: [...]
        }
        fail-fast?: bool
        max-parallel?: int
    }

    continue-on-error?: bool

    container?: string | #Container

    services?: [string]: #Container

    steps: [...#Step]
}

#Permission: "read" | "write" | "none"

#Step: {
    id?: string
    if?: string
    name?: string
    uses?: string
    run?: string
    shell?: string
    with?: [string]: _
    env?: [string]: string
    continue-on-error?: bool
    timeout-minutes?: number
    working-directory?: string
}

#Container: {
    image: string
    credentials?: {
        username: string
        password: string
    }
    env?: [string]: string
    ports?: [...(int | string)]
    volumes?: [...string]
    options?: string
}
"#;

const CLOUDFORMATION_SCHEMA: &str = r#"
// AWS CloudFormation Template Schema
package cloudformation

#Template: {
    AWSTemplateFormatVersion?: "2010-09-09"
    Description?: string

    Metadata?: [string]: _

    Parameters?: [string]: #Parameter

    Mappings?: [string]: [string]: [string]: string

    Conditions?: [string]: _

    Transform?: string | [...string]

    Resources: [string]: #Resource

    Outputs?: [string]: #Output
}

#Parameter: {
    Type: "String" | "Number" | "List<Number>" | "CommaDelimitedList" |
          "AWS::SSM::Parameter::Name" | "AWS::SSM::Parameter::Value<String>" |
          "AWS::EC2::AvailabilityZone::Name" | "AWS::EC2::Image::Id" |
          "AWS::EC2::Instance::Id" | "AWS::EC2::KeyPair::KeyName" |
          "AWS::EC2::SecurityGroup::GroupName" | "AWS::EC2::SecurityGroup::Id" |
          "AWS::EC2::Subnet::Id" | "AWS::EC2::VPC::Id" |
          "List<AWS::EC2::AvailabilityZone::Name>" | "List<AWS::EC2::Image::Id>" |
          "List<AWS::EC2::Instance::Id>" | "List<AWS::EC2::SecurityGroup::GroupName>" |
          "List<AWS::EC2::SecurityGroup::Id>" | "List<AWS::EC2::Subnet::Id>" |
          "List<AWS::EC2::VPC::Id>"

    Default?: _
    Description?: string
    AllowedPattern?: string
    AllowedValues?: [...]
    ConstraintDescription?: string
    MaxLength?: int
    MinLength?: int
    MaxValue?: number
    MinValue?: number
    NoEcho?: bool
}

#Resource: {
    Type: string
    Properties?: [string]: _
    DependsOn?: string | [...string]
    Condition?: string
    CreationPolicy?: _
    DeletionPolicy?: "Delete" | "Retain" | "Snapshot"
    UpdatePolicy?: _
    UpdateReplacePolicy?: "Delete" | "Retain" | "Snapshot"
    Metadata?: [string]: _
}

#Output: {
    Value: _
    Description?: string
    Export?: {
        Name: _
    }
    Condition?: string
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_builtins() {
        let registry = RsrSchemaRegistry::new();

        assert!(registry.get("rsr:pipeline").is_some());
        assert!(registry.get("rsr:requirement").is_some());
        assert!(registry.get("rsr:config").is_some());
        assert!(registry.get("k8s:base").is_some());
    }

    #[test]
    fn test_get_content() {
        let registry = RsrSchemaRegistry::new();

        let content = registry.get_content("rsr:pipeline").unwrap();
        assert!(content.contains("#Pipeline"));
    }

    #[test]
    fn test_by_tag() {
        let registry = RsrSchemaRegistry::new();

        let rsr_schemas = registry.by_tag("rsr");
        assert!(rsr_schemas.len() >= 2);
    }
}
