
pub mod cfn_environment_account_connection {

#[derive(serde::Serialize, Default)]
pub struct CfnEnvironmentAccountConnection {
    /// The Amazon Resource Name (ARN) of the IAM service role that's created in the environment account. AWS Proton uses this role to provision infrastructure resources in the associated environment account.
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// <p>An optional list of metadata items that you can associate with the Proton environment account connection. A tag is a key-value pair.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/resources.html">Proton resources and tagging</a> in the
    /// <i>Proton User Guide</i>.</p>
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The environment account that's connected to the environment account connection.
    #[serde(rename = "EnvironmentAccountId")]
    pub environment_account_id: Option<String>,
    /// The Amazon Resource Name (ARN) of the IAM service role that AWS Proton uses when provisioning directly defined components in the associated environment account. It determines the scope of infrastructure that a component can provision in the account.
    #[serde(rename = "ComponentRoleArn")]
    pub component_role_arn: Option<String>,
    /// The name of the AWS Proton environment that's created in the associated management account.
    #[serde(rename = "EnvironmentName")]
    pub environment_name: Option<String>,
    /// The ID of the management account that accepts or rejects the environment account connection. You create an manage the AWS Proton environment in this account. If the management account accepts the environment account connection, AWS Proton can use the associated IAM role to provision environment infrastructure resources in the associated environment account.
    #[serde(rename = "ManagementAccountId")]
    pub management_account_id: Option<String>,
    /// The Amazon Resource Name (ARN) of an IAM service role in the environment account. AWS Proton uses this role to provision infrastructure resources using CodeBuild-based provisioning in the associated environment account.
    #[serde(rename = "CodebuildRoleArn")]
    pub codebuild_role_arn: Option<String>,

}

pub type Status = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}


}

pub mod cfn_environment_template {

#[derive(serde::Serialize, Default)]
pub struct CfnEnvironmentTemplate {
    /// <p>A description of the environment template.</p>
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// <p>A customer provided encryption key that Proton uses to encrypt data.</p>
    #[serde(rename = "EncryptionKey")]
    pub encryption_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// <p>The environment template name as displayed in the developer interface.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// <p>An optional list of metadata items that you can associate with the Proton environment template. A tag is a key-value pair.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/resources.html">Proton resources and tagging</a> in the
    /// <i>Proton User Guide</i>.</p>
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Provisioning")]
    pub provisioning: Option<Provisioning>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "Key")]
    pub key: String,

}
pub type Provisioning = String;

}

pub mod cfn_service_template {

#[derive(serde::Serialize, Default)]
pub struct CfnServiceTemplate {
    /// <p>The name of the service template as displayed in the developer interface.</p>
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
    /// <p>An optional list of metadata items that you can associate with the Proton service template. A tag is a key-value pair.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/resources.html">Proton resources and tagging</a> in the
    /// <i>Proton User Guide</i>.</p>
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// No documentation provided by AWS
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// <p>A customer provided encryption key that's used to encrypt data.</p>
    #[serde(rename = "EncryptionKey")]
    pub encryption_key: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "PipelineProvisioning")]
    pub pipeline_provisioning: Option<Provisioning>,
    /// <p>A description of the service template.</p>
    #[serde(rename = "Description")]
    pub description: Option<String>,

}

pub type Provisioning = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}


}
