

/// Creates a signing profile. A signing profile is a code signing template that can be used to 			carry out a pre-defined signing job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnSigningProfile {


    /// 
    /// The ID of a platform that is available for use by a signing profile.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PlatformId")]
    pub platform_id: String,


    /// 
    /// A list of tags associated with the signing profile.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// The validity period override for any signature generated using this signing             profile. If unspecified, the default is 135 months.
    /// 
    /// Required: No
    ///
    /// Type: SignatureValidityPeriod
    ///
    /// Update requires: Replacement
    #[serde(rename = "SignatureValidityPeriod")]
    pub signature_validity_period: Option<SignatureValidityPeriod>,

}



impl cfn_resources::CfnResource for CfnSigningProfile {
    fn type_string() -> &'static str {
        "AWS::Signer::SigningProfile"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
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
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Value")]
    pub value: String,


    /// 
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    /// 
    /// Required: Yes
    /// 
    /// Type: String
    /// 
    #[serde(rename = "Key")]
    pub key: String,

}




/// The validity period for the signing job.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SignatureValidityPeriod {


    /// 
    /// The numerical value of the time unit for signature validity.
    /// 
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    pub value: Option<i64>,


    /// 
    /// The time unit for signature validity: DAYS | MONTHS | YEARS.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    pub cfn_type: Option<String>,

}


