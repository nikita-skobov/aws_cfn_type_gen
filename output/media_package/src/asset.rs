

/// Creates an asset to ingest VOD content.
///
/// After it's created, the asset starts ingesting content and generates playback URLs for the packaging configurations associated with it. When ingest is complete, downstream         devices use the appropriate URL to request VOD content from AWS Elemental MediaPackage.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAsset {


    /// 
    /// The ARN for the IAM role that provides AWS Elemental MediaPackage access to the Amazon S3 bucket where the source content is stored. Valid format: arn:aws:iam::{accountID}:role/{name}
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceRoleArn")]
    pub source_role_arn: String,


    /// 
    /// The ARN for the source content in Amazon S3.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceArn")]
    pub source_arn: String,


    /// 
    /// Unique identifier for this asset, as it's configured in the key provider service.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,


    /// 
    /// Unique identifier that you assign to the asset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// The tags to assign to the asset.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The ID of the packaging group associated with this asset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PackagingGroupId")]
    pub packaging_group_id: String,


    /// 
    /// List of playback endpoints that are available for this asset.
    /// 
    /// Required: No
    ///
    /// Type: List of EgressEndpoint
    ///
    /// Update requires: No interruption
    #[serde(rename = "EgressEndpoints")]
    pub egress_endpoints: Option<Vec<EgressEndpoint>>,

}



impl cfn_resources::CfnResource for CfnAsset {
    fn type_string() -> &'static str {
        "AWS::MediaPackage::Asset"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The playback endpoint for a packaging configuration on an asset.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EgressEndpoint {


    /// 
    /// The URL that's used to request content from this endpoint.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Url")]
    pub url: String,


    /// 
    /// The ID of a packaging configuration that's applied to this asset.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PackagingConfigurationId")]
    pub packaging_configuration_id: String,

}




/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Tag {


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,


    /// 
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,

}


