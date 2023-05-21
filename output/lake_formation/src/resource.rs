

/// The AWS::LakeFormation::Resource represents the data ( buckets and folders) that is being registered with AWS Lake Formation.    During a stack operation, AWS CloudFormation calls the AWS Lake Formation RegisterResource API operation to register the resource.    To remove a Resource type, AWS CloudFormation calls the AWS Lake Formation DeregisterResource API operation.
#[derive(Default, serde::Serialize)]
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
    pub resource_arn: String,


    /// 
    /// Allows Lake Formation to assume a role to access tables in a federated database.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "WithFederation")]
    pub with_federation: Option<bool>,


    /// 
    /// The IAM role that registered a resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,


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

}
