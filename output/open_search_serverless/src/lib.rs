
pub mod cfn_access_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnAccessPolicy {
    /// The JSON policy document that is the content for the policy
    #[serde(rename = "Policy")]
    pub policy: String,
    /// The name of the policy
    #[serde(rename = "Name")]
    pub name: String,
    /// The description of the policy
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: AccessPolicyType,

}

pub type AccessPolicyType = String;

}

pub mod cfn_collection {

#[derive(serde::Serialize, Default)]
pub struct CfnCollection {
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: Option<CollectionType>,
    /// List of tags to be added to the resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
    /// The name of the collection.
    /// 
    /// The name must meet the following criteria:
    /// Unique to your account and AWS Region
    /// Starts with a lowercase letter
    /// Contains only lowercase letters a-z, the numbers 0-9 and the hyphen (-)
    /// Contains between 3 and 32 characters
    #[serde(rename = "Name")]
    pub name: String,
    /// The description of the collection
    #[serde(rename = "Description")]
    pub description: Option<String>,

}


#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}
pub type CollectionType = String;

}

pub mod cfn_security_config {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityConfig {
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: Option<SecurityConfigType>,
    /// Security config description
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "SamlOptions")]
    pub saml_options: Option<SamlConfigOptions>,
    /// The friendly name of the security config
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

pub type SecurityConfigType = String;
#[derive(serde::Serialize, Default)]
pub struct SamlConfigOptions {
    #[serde(rename = "GroupAttribute")]
    pub group_attribute: Option<String>,
    #[serde(rename = "SessionTimeout")]
    pub session_timeout: Option<usize>,
    #[serde(rename = "Metadata")]
    pub metadata: String,
    #[serde(rename = "UserAttribute")]
    pub user_attribute: Option<String>,

}


}

pub mod cfn_security_policy {

#[derive(serde::Serialize, Default)]
pub struct CfnSecurityPolicy {
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: SecurityPolicyType,
    /// The description of the policy
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The name of the policy
    #[serde(rename = "Name")]
    pub name: String,
    /// The JSON policy document that is the content for the policy
    #[serde(rename = "Policy")]
    pub policy: String,

}

pub type SecurityPolicyType = String;

}

pub mod cfn_vpc_endpoint {

#[derive(serde::Serialize, Default)]
pub struct CfnVpcEndpoint {
    /// The ID of one or more subnets in which to create an endpoint network interface
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// The ID of the VPC in which the endpoint will be used.
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// The ID of one or more security groups to associate with the endpoint network interface
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Option<Vec<String>>,
    /// The name of the VPC Endpoint
    #[serde(rename = "Name")]
    pub name: String,

}



}
