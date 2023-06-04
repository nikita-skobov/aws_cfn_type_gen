/// The AWS::LakeFormation::Resource represents the data ( buckets and folders) that is being registered with AWS Lake Formation.    During a stack operation, AWS CloudFormation calls the AWS Lake Formation RegisterResource API operation to register the resource.    To remove a Resource type, AWS CloudFormation calls the AWS Lake Formation DeregisterResource API operation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnResource {
    ///
    /// The Amazon Resource Name (ARN) of the resource.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceArn")]
    pub resource_arn: cfn_resources::StrVal,

    ///
    /// The IAM role that registered a resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<cfn_resources::StrVal>,

    ///
    /// Designates a trusted caller, an IAM principal, by registering this caller with the Data Catalog.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "UseServiceLinkedRole")]
    pub use_service_linked_role: bool,

    ///
    /// Allows Lake Formation to assume a role to access tables in a federated database.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "WithFederation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_federation: Option<bool>,
}

impl cfn_resources::CfnResource for CfnResource {
    fn type_string(&self) -> &'static str {
        "AWS::LakeFormation::Resource"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
