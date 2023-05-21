/// The AWS::Macie::AllowList resource specifies an allow list. In Amazon Macie, an allow list defines specific text or a text pattern for Macie to ignore when it inspects data sources for sensitive data. If data       matches text or a text pattern in an allow list, Macie doesn’t report the       data in sensitive data findings or sensitive data discovery results, even if the data       matches the criteria of a custom data identifier or a managed data identifier. You can       create and use allow lists in all the AWS Regions where Macie is currently available except the Asia Pacific (Osaka) Region.
///
/// Macie supports two types of allow lists:
///
/// For more information, see Defining sensitive data exceptions with         allow lists in the Amazon Macie User       Guide.
///
/// An AWS::Macie::Session resource must exist for an AWS account before you can create an AWS::Macie::AllowList       resource for the account. Use a DependsOn         attribute to ensure that an AWS::Macie::Session resource is       created before other Macie resources are created for an account. For       example, "DependsOn": "Session".
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnAllowList {
    ///
    /// The criteria that specify the text or text pattern to ignore. The criteria can be the       location and name of an Amazon S3 object that lists specific text to ignore         (S3WordsList), or a regular expression (Regex) that       defines a text pattern to ignore.
    ///
    /// Required: Yes
    ///
    /// Type: Criteria
    ///
    /// Update requires: No interruption
    #[serde(rename = "Criteria")]
    pub criteria: Criteria,

    ///
    /// A custom description of the allow list. The description can contain 1-512       characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// A custom name for the allow list. The name can contain 1-128 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// An array of key-value pairs to apply to the allow list.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnAllowList {
    fn type_string(&self) -> &'static str {
        "AWS::Macie::AllowList"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.criteria.validate()?;

        Ok(())
    }
}

/// Specifies the criteria for an allow list, which is a list that defines specific text       or a text pattern to ignore when inspecting data sources for sensitive data. The       criteria can be:
///
/// The criteria must specify either an S3 object or a regular expression. It can't       specify both.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Criteria {
    ///
    /// The regular expression (regex) that defines the text pattern to       ignore. The expression can contain 1-512 characters.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Regex")]
    pub regex: Option<String>,

    ///
    /// The location and name of an Amazon S3 object that lists specific text to       ignore.
    ///
    /// Required: No
    ///
    /// Type: S3WordsList
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3WordsList")]
    pub s3_words_list: Option<S3WordsList>,
}

impl cfn_resources::CfnResource for Criteria {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3_words_list
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the location and name of an Amazon Simple Storage Service (Amazon S3)       object that lists specific, predefined text to ignore when inspecting data sources for sensitive       data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3WordsList {
    ///
    /// The full name of the S3 bucket that contains the object. This value correlates to the         Name field of a bucket's properties in Amazon S3.
    ///
    /// This value is case sensitive. In addition, don't use wildcard characters or specify       partial values for the name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

    ///
    /// The full name of the S3 object. This value correlates to the Key field of       an object's properties in Amazon S3. If the name includes a path, include the       complete path. For example, AllowLists/Macie/MyList.txt.
    ///
    /// This value is case sensitive. In addition, don't use wildcard characters or specify       partial values for the name.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectKey")]
    pub object_key: String,
}

impl cfn_resources::CfnResource for S3WordsList {
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
