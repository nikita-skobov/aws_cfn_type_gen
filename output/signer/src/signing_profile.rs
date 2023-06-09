/// Creates a signing profile. A signing profile is a code signing template that can be used to 			carry out a pre-defined signing job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
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
    pub platform_id: cfn_resources::StrVal,

    ///
    /// The validity period override for any signature generated using this signing             profile. If unspecified, the default is 135 months.
    ///
    /// Required: No
    ///
    /// Type: SignatureValidityPeriod
    ///
    /// Update requires: Replacement
    #[serde(rename = "SignatureValidityPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_validity_period: Option<SignatureValidityPeriod>,

    ///
    /// A list of tags associated with the signing profile.
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
    pub att_arn: CfnSigningProfilearn,

    #[serde(skip_serializing)]
    pub att_profile_name: CfnSigningProfileprofilename,

    #[serde(skip_serializing)]
    pub att_profile_version: CfnSigningProfileprofileversion,

    #[serde(skip_serializing)]
    pub att_profile_version_arn: CfnSigningProfileprofileversionarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSigningProfilearn;
impl CfnSigningProfilearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSigningProfileprofilename;
impl CfnSigningProfileprofilename {
    pub fn att_name(&self) -> &'static str {
        r#"ProfileName"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSigningProfileprofileversion;
impl CfnSigningProfileprofileversion {
    pub fn att_name(&self) -> &'static str {
        r#"ProfileVersion"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnSigningProfileprofileversionarn;
impl CfnSigningProfileprofileversionarn {
    pub fn att_name(&self) -> &'static str {
        r#"ProfileVersionArn"#
    }
}

impl cfn_resources::CfnResource for CfnSigningProfile {
    fn type_string(&self) -> &'static str {
        "AWS::Signer::SigningProfile"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.signature_validity_period
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The validity period for the signing job.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SignatureValidityPeriod {
    ///
    /// The time unit for signature validity: DAYS | MONTHS | YEARS.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfn_type: Option<cfn_resources::StrVal>,

    ///
    /// The numerical value of the time unit for signature validity.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: Replacement
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

impl cfn_resources::CfnResource for SignatureValidityPeriod {
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
