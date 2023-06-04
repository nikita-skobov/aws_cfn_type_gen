/// The AWS::CodePipeline::Pipeline resource creates a CodePipeline pipeline    that describes how software changes go through a release process. For more information, see     What Is      CodePipeline? in the AWS CodePipeline User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPipeline {
    ///
    /// The S3 bucket where artifacts for the pipeline are stored.
    ///
    /// NoteYou must include either artifactStore or           artifactStores in your pipeline, but you cannot use both. If you         create a cross-region action in your pipeline, you must use           artifactStores.
    ///
    /// Required: Conditional
    ///
    /// Type: ArtifactStore
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArtifactStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_store: Option<ArtifactStore>,

    ///
    /// A mapping of artifactStore objects and their corresponding AWS       Regions. There must be an artifact store for the pipeline Region and for each       cross-region action in the pipeline.
    ///
    /// NoteYou must include either artifactStore or           artifactStores in your pipeline, but you cannot use both. If you         create a cross-region action in your pipeline, you must use           artifactStores.
    ///
    /// Required: Conditional
    ///
    /// Type: List of ArtifactStoreMap
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArtifactStores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_stores: Option<Vec<ArtifactStoreMap>>,

    ///
    /// Represents the input of a DisableStageTransition action.
    ///
    /// Required: No
    ///
    /// Type: List of StageTransition
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableInboundStageTransitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_inbound_stage_transitions: Option<Vec<StageTransition>>,

    ///
    /// The name of the pipeline.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [A-Za-z0-9.@\-_]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Indicates whether to rerun the CodePipeline pipeline after you update it.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RestartExecutionOnUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_execution_on_update: Option<bool>,

    ///
    /// The Amazon Resource Name (ARN) for CodePipeline to use to either perform       actions with no actionRoleArn, or to use to assume roles for actions with       an actionRoleArn.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:aws(-[\w]+)*:iam::[0-9]{12}:role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: cfn_resources::StrVal,

    ///
    /// Represents information about a stage and its definition.
    ///
    /// Required: Yes
    ///
    /// Type: List of StageDeclaration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Stages")]
    pub stages: Vec<StageDeclaration>,

    ///
    /// Specifies the tags applied to the pipeline.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_version: CfnPipelineversion,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPipelineversion;
impl CfnPipelineversion {
    pub fn att_name(&self) -> &'static str {
        r#"Version"#
    }
}

impl cfn_resources::CfnResource for CfnPipeline {
    fn type_string(&self) -> &'static str {
        "AWS::CodePipeline::Pipeline"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.artifact_store
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 1024 as _ {
                return Err(format!(
                    "Max validation failed on field 'role_arn'. {} is greater than 1024",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents information about an action declaration.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ActionDeclaration {
    ///
    /// Specifies the action type and the provider of the action.
    ///
    /// Required: Yes
    ///
    /// Type: ActionTypeId
    ///
    /// Update requires: No interruption
    #[serde(rename = "ActionTypeId")]
    pub action_type_id: ActionTypeId,

    ///
    /// The action's configuration. These are key-value pairs that specify input values for       an action. For more information, see Action Structure Requirements in CodePipeline. For the list of       configuration properties for the AWS CloudFormation action type in CodePipeline, see         Configuration Properties Reference in the         AWS CloudFormation User         Guide. For template snippets with examples, see Using Parameter Override Functions with CodePipeline Pipelines in the               AWS CloudFormation User Guide.
    ///
    /// The values can be represented in either JSON or YAML format. For example, the JSON       configuration item format is as follows:
    ///
    /// JSON:
    ///
    /// "Configuration" : { Key : Value },
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,

    ///
    /// The name or ID of the artifact consumed by the action, such as a test or build    artifact. While the field is not a required parameter, most actions have an action    configuration that requires a specified quantity of input artifacts. To refer to the action    configuration specification by action provider, see the Action structure reference    in the AWS CodePipeline User Guide.
    ///
    /// NoteFor a CodeBuild action with multiple input artifacts, one of your input sources must be     designated the PrimarySource. For more information, see the CodeBuild action      reference page in the AWS CodePipeline User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of InputArtifact
    ///
    /// Update requires: No interruption
    #[serde(rename = "InputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<Vec<InputArtifact>>,

    ///
    /// The action declaration's name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [A-Za-z0-9.@\-_]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// The variable namespace associated with the action. All variables produced as output by       this action fall under this namespace.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [A-Za-z0-9@\-_]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<cfn_resources::StrVal>,

    ///
    /// The name or ID of the result of the action declaration, such as a test or build    artifact. While the field is not a required parameter, most actions have an action    configuration that requires a specified quantity of output artifacts. To refer to the action    configuration specification by action provider, see the Action structure reference in the AWS CodePipeline User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of OutputArtifact
    ///
    /// Update requires: No interruption
    #[serde(rename = "OutputArtifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<Vec<OutputArtifact>>,

    ///
    /// The action declaration's AWS Region, such as us-east-1.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the IAM service role that performs the declared action. This is assumed       through the roleArn for the pipeline.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: arn:aws(-[\w]+)*:iam::[0-9]{12}:role/.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// The order in which actions are run.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Maximum: 999
    ///
    /// Update requires: No interruption
    #[serde(rename = "RunOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_order: Option<i64>,
}

impl cfn_resources::CfnResource for ActionDeclaration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.action_type_id.validate()?;

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.namespace {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 100 as _ {
                    return Err(format!(
                        "Max validation failed on field 'namespace'. {} is greater than 100",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.namespace {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'namespace'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.region {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 30 as _ {
                    return Err(format!(
                        "Max validation failed on field 'region'. {} is greater than 30",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.region {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 4 as _ {
                    return Err(format!(
                        "Min validation failed on field 'region'. {} is less than 4",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.role_arn {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'role_arn'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.run_order {
            if *the_val > 999 as _ {
                return Err(format!(
                    "Max validation failed on field 'run_order'. {} is greater than 999",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.run_order {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'run_order'. {} is less than 1",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Represents information about an action type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ActionTypeId {
    ///
    /// A category defines what kind of action can be taken in the stage, and constrains the    provider type for the action. Valid categories are limited to one of the values    below.
    ///
    /// Source        Build        Test        Deploy        Invoke        Approval
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    pub category: cfn_resources::StrVal,

    ///
    /// The creator of the action being called. There are three valid values for the         Owner field in the action category section within your pipeline       structure: AWS, ThirdParty, and Custom. For more       information, see Valid Action Types and Providers in CodePipeline.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Owner")]
    pub owner: cfn_resources::StrVal,

    ///
    /// The provider of the service being called by the action. Valid providers are       determined by the action category. For example, an action in the Deploy category type       might have a provider of CodeDeploy, which would be specified as CodeDeploy. For       more information, see Valid Action Types and Providers in CodePipeline.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Provider")]
    pub provider: cfn_resources::StrVal,

    ///
    /// A string that describes the action version.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Version")]
    pub version: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ActionTypeId {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// The S3 bucket where artifacts for the pipeline are stored.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ArtifactStore {
    ///
    /// The encryption key used to encrypt the data in the artifact store, such as an AWS Key    Management Service (AWS KMS) key. If this is undefined, the default key for Amazon S3 is used.    To see an example artifact store encryption key field, see the example structure here: AWS::CodePipeline::Pipeline.
    ///
    /// Required: No
    ///
    /// Type: EncryptionKey
    ///
    /// Update requires: No interruption
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<EncryptionKey>,

    ///
    /// The S3 bucket used for storing the artifacts for a pipeline. You can specify the       name of an S3 bucket but not a folder in the bucket. A folder to contain the pipeline       artifacts is created for you based on the name of the pipeline. You can use any S3       bucket in the same AWS Region as the pipeline to store your pipeline       artifacts.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: [a-zA-Z0-9\-\.]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Location")]
    pub location: cfn_resources::StrVal,

    ///
    /// The type of the artifact store, such as S3.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: S3
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: ArtifactStoreTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ArtifactStoreTypeEnum {
    /// S3
    #[serde(rename = "S3")]
    S3,
}

impl Default for ArtifactStoreTypeEnum {
    fn default() -> Self {
        ArtifactStoreTypeEnum::S3
    }
}

impl cfn_resources::CfnResource for ArtifactStore {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.encryption_key
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        let the_val = &self.location;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 63 as _ {
                return Err(format!(
                    "Max validation failed on field 'location'. {} is greater than 63",
                    s.len()
                ));
            }
        }

        let the_val = &self.location;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 3 as _ {
                return Err(format!(
                    "Min validation failed on field 'location'. {} is less than 3",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// A mapping of artifactStore objects and their corresponding AWS       Regions. There must be an artifact store for the pipeline Region and for each       cross-region action in the pipeline.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ArtifactStoreMap {
    ///
    /// Represents information about the S3 bucket where artifacts are stored for the       pipeline.
    ///
    /// NoteYou must include either artifactStore or           artifactStores in your pipeline, but you cannot use both. If you         create a cross-region action in your pipeline, you must use           artifactStores.
    ///
    /// Required: Conditional
    ///
    /// Type: ArtifactStore
    ///
    /// Update requires: No interruption
    #[serde(rename = "ArtifactStore")]
    pub artifact_store: ArtifactStore,

    ///
    /// The action declaration's AWS Region, such as us-east-1.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 4
    ///
    /// Maximum: 30
    ///
    /// Update requires: No interruption
    #[serde(rename = "Region")]
    pub region: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for ArtifactStoreMap {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.artifact_store.validate()?;

        let the_val = &self.region;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 30 as _ {
                return Err(format!(
                    "Max validation failed on field 'region'. {} is greater than 30",
                    s.len()
                ));
            }
        }

        let the_val = &self.region;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 4 as _ {
                return Err(format!(
                    "Min validation failed on field 'region'. {} is less than 4",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Reserved for future use.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BlockerDeclaration {
    ///
    /// Reserved for future use.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Reserved for future use.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Schedule
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: BlockerDeclarationTypeEnum,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum BlockerDeclarationTypeEnum {
    /// Schedule
    #[serde(rename = "Schedule")]
    Schedule,
}

impl Default for BlockerDeclarationTypeEnum {
    fn default() -> Self {
        BlockerDeclarationTypeEnum::Schedule
    }
}

impl cfn_resources::CfnResource for BlockerDeclaration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents information about the key used to encrypt data in the artifact store, such    as an AWS Key Management Service (AWS KMS) key.
///
/// EncryptionKey is a property of the ArtifactStore property type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EncryptionKey {
    ///
    /// The ID used to identify the key. For an AWS KMS key, you can use the key ID, the       key ARN, or the alias ARN.
    ///
    /// NoteAliases are recognized only in the account that created the AWS KMS         key. For cross-account actions, you can only use the key ID or key ARN to identify         the key. Cross-account actions involve using the role from the other account         (AccountB), so specifying the key ID will use the key from the other account         (AccountB).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The type of encryption key, such as an AWS KMS key. When creating or       updating a pipeline, the value must be set to 'KMS'.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Type")]
    pub cfn_type: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for EncryptionKey {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Represents information about an artifact to be worked on, such as a test or build       artifact.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InputArtifact {
    ///
    /// The name of the artifact to be worked on (for example, "My App").
    ///
    /// Artifacts are the files that are worked on by actions in the pipeline. See the       action configuration for each action for details about artifact parameters. For example,       the S3 source action input artifact is a file name (or file path), and the files are       generally provided as a ZIP file. Example artifact name: SampleApp_Windows.zip
    ///
    /// The input artifact of an action must exactly match the output artifact declared in       a preceding action, but the input artifact does not have to be the next action in strict       sequence from the action that provided the output artifact. Actions in parallel can       declare different output artifacts, which are in turn consumed by different following       actions.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [a-zA-Z0-9_\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for InputArtifact {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents information about the output of an action.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OutputArtifact {
    ///
    /// The name of the output of an artifact, such as "My App".
    ///
    /// The output artifact name must exactly match the input artifact declared for a    downstream action. However, the downstream action's input artifact does not have to be the    next action in strict sequence from the action that provided the output artifact. Actions in    parallel can declare different output artifacts, which are in turn consumed by different    following actions.
    ///
    /// Output artifact names must be unique within a pipeline.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [a-zA-Z0-9_\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for OutputArtifact {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Represents information about a stage and its definition.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StageDeclaration {
    ///
    /// The actions included in a stage.
    ///
    /// Required: Yes
    ///
    /// Type: List of ActionDeclaration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Actions")]
    pub actions: Vec<ActionDeclaration>,

    ///
    /// Reserved for future use.
    ///
    /// Required: No
    ///
    /// Type: List of BlockerDeclaration
    ///
    /// Update requires: No interruption
    #[serde(rename = "Blockers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockers: Option<Vec<BlockerDeclaration>>,

    ///
    /// The name of the stage.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [A-Za-z0-9.@\-_]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for StageDeclaration {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// The name of the pipeline in which you want to disable the flow of artifacts from       one stage to another.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StageTransition {
    ///
    /// The reason given to the user that a stage is disabled, such as waiting for manual       approval or manual tests. This message is displayed in the pipeline console       UI.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 300
    ///
    /// Pattern: [a-zA-Z0-9!@ \(\)\.\*\?\-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Reason")]
    pub reason: cfn_resources::StrVal,

    ///
    /// The name of the stage where you want to disable the inbound or outbound transition       of artifacts.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: [A-Za-z0-9.@\-_]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "StageName")]
    pub stage_name: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for StageTransition {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.reason;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 300 as _ {
                return Err(format!(
                    "Max validation failed on field 'reason'. {} is greater than 300",
                    s.len()
                ));
            }
        }

        let the_val = &self.reason;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'reason'. {} is less than 1",
                    s.len()
                ));
            }
        }

        let the_val = &self.stage_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'stage_name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.stage_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'stage_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
