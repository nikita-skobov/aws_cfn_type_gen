/// Creates a project, which is the logical object in Evidently that can contain features, launches, and       experiments. Use projects to group similar features together.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnProject {
    ///
    /// Use this parameter if the project will use client-side evaluation powered by AWS AppConfig. Client-side       evaluation allows your application to assign variations to user       sessions locally instead of by calling the EvaluateFeature operation. This       mitigates the latency and availability risks that come with an API call. For more information,       see         Use client-side evaluation - powered by AWS AppConfig.
    ///
    /// This parameter is a structure that       contains information about the AWS AppConfig application that will be used as for client-side evaluation.
    ///
    /// To create a project that uses client-side evaluation, you must have the       evidently:ExportProjectAsConfiguration permission.
    ///
    /// Required: No
    ///
    /// Type: AppConfigResourceObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "AppConfigResource")]
    pub app_config_resource: Option<AppConfigResourceObject>,

    ///
    /// A structure that contains information about where Evidently is to store       evaluation events for longer term storage, if you choose to do so. If you choose       not to store these events, Evidently deletes them after using them to produce metrics and other experiment       results that you can view.
    ///
    /// You can't specify both CloudWatchLogs and S3Destination in the same operation.
    ///
    /// Required: No
    ///
    /// Type: DataDeliveryObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataDelivery")]
    pub data_delivery: Option<DataDeliveryObject>,

    ///
    /// An optional description of the project.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,

    ///
    /// The name for the project. It can include up to 127 characters.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

    ///
    /// Assigns one or more tags (key-value pairs) to the project.
    ///
    /// Tags can help you organize and categorize your resources. You can also use them to scope user         permissions by granting a user         permission to access or change only resources with certain tag values.
    ///
    /// Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.
    ///
    /// You can associate as many as 50 tags with a project.
    ///
    /// For more information, see Tagging AWS resources.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,
}

impl cfn_resources::CfnResource for CfnProject {
    fn type_string(&self) -> &'static str {
        "AWS::Evidently::Project"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.app_config_resource
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.data_delivery
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// This is a structure that defines the configuration of how your application       integrates with AWS AppConfig to run client-side evaluation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AppConfigResourceObject {
    ///
    /// The ID of the AWS AppConfig application to use for client-side evaluation.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApplicationId")]
    pub application_id: String,

    ///
    /// The ID of the AWS AppConfig environment to use for client-side evaluation.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnvironmentId")]
    pub environment_id: String,
}

impl cfn_resources::CfnResource for AppConfigResourceObject {
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

/// A structure that contains information about where Evidently is to store       evaluation events for longer term storage.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataDeliveryObject {
    ///
    /// If the project stores evaluation events in CloudWatch Logs, this structure       stores the log group name.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogGroup")]
    pub log_group: Option<String>,

    ///
    /// If the project stores evaluation events in an Amazon S3 bucket, this structure       stores the bucket name and bucket prefix.
    ///
    /// Required: No
    ///
    /// Type: S3Destination
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<S3Destination>,
}

impl cfn_resources::CfnResource for DataDeliveryObject {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// If the project stores evaluation events in an Amazon S3 bucket, this structure       stores the bucket name and bucket prefix.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3Destination {
    ///
    /// The name of the bucket in which Evidently stores evaluation events.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

    ///
    /// The bucket prefix in which Evidently stores evaluation events.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
}

impl cfn_resources::CfnResource for S3Destination {
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
