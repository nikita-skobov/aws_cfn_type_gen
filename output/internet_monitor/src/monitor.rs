/// The AWS::InternetMonitor::Monitor resource is an Internet Monitor resource type that contains information about how 		you create a monitor in Amazon CloudWatch Internet Monitor. A monitor in Internet Monitor provides visibility into performance and 		availability between your applications hosted on AWS and your end users, using a traffic profile that it creates 		based on the application resources that you add: Virtual Private Clouds (VPCs), Amazon CloudFront distributions, or WorkSpaces directories.
///
/// Internet Monitor also alerts you to internet issues that impact your application in the city-networks (geographies and networks) 			where your end users use it. With Internet Monitor, you can quickly pinpoint the locations and providers that are affected, so 			that you can address the issue.
///
/// For more information, see 		Using Amazon CloudWatch Internet Monitor in the Amazon CloudWatch User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnMonitor {
    ///
    /// Publish internet measurements for a monitor for all city-networks (up to the 500,000 service limit) to another location, such as an Amazon S3 bucket. 			Measurements are also published to Amazon CloudWatch Logs for the first 500 (by traffic volume) city-networks (client locations and ASNs, typically 			internet service providers or ISPs).
    ///
    /// Required: No
    ///
    /// Type: InternetMeasurementsLogDelivery
    ///
    /// Update requires: No interruption
    #[serde(rename = "InternetMeasurementsLogDelivery")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub internet_measurements_log_delivery: Option<InternetMeasurementsLogDelivery>,

    ///
    /// The maximum number of city-networks to monitor for your resources. A city-network is the location (city) where 			clients access your application resources from and the network, such as an internet service provider, that clients 			access the resources through.
    ///
    /// For more information, see 			Choosing a city-network maximum value in Using Amazon CloudWatch Internet Monitor.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "MaxCityNetworksToMonitor")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub max_city_networks_to_monitor: Option<i64>,

    ///
    /// The name of the monitor. A monitor name can contain only alphanumeric characters, dashes (-), periods (.), 			and underscores (_).
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "MonitorName")]
    pub monitor_name: cfn_resources::StrVal,

    ///
    /// The resources that have been added for the monitor, listed by their Amazon Resource Names (ARNs).
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub resources: Option<Vec<String>>,

    ///
    /// The resources to add to a monitor, which you provide as a set of Amazon Resource Names (ARNs).
    ///
    /// You can add a combination of Virtual Private Clouds (VPCs) and Amazon CloudFront distributions, or you can add WorkSpaces directories. 		You can't add all three types of resources.
    ///
    /// NoteIf you add only VPC resources, at least one VPC must have an Internet Gateway attached to it, to make sure that it has internet connectivity.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourcesToAdd")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub resources_to_add: Option<Vec<String>>,

    ///
    /// The resources to remove from a monitor, which you provide as a set of Amazon Resource Names (ARNs).
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ResourcesToRemove")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub resources_to_remove: Option<Vec<String>>,

    ///
    /// The status of a monitor. The accepted values that you can specify for Status are ACTIVE and INACTIVE.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub status: Option<cfn_resources::StrVal>,

    ///
    /// The tags for a monitor, listed as a set of key:value pairs.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    ///
    /// The percentage of the internet-facing traffic for your application that you want to monitor. You can also, optionally, set 			a limit for the number of city-networks (client locations and ASNs, typically internet service providers) that Internet Monitor will monitor traffic for. 			The city-networks maximum limit caps the number of city-networks that Internet Monitor monitors for your application, regardless of 			the percentage of traffic that you choose to monitor.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrafficPercentageToMonitor")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub traffic_percentage_to_monitor: Option<i64>,

    #[serde(skip_serializing)]
    pub att_created_at: CfnMonitorcreatedat,

    #[serde(skip_serializing)]
    pub att_modified_at: CfnMonitormodifiedat,

    #[serde(skip_serializing)]
    pub att_monitor_arn: CfnMonitormonitorarn,

    #[serde(skip_serializing)]
    pub att_processing_status: CfnMonitorprocessingstatus,

    #[serde(skip_serializing)]
    pub att_processing_status_info: CfnMonitorprocessingstatusinfo,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMonitorcreatedat;
impl CfnMonitorcreatedat {
    pub fn att_name(&self) -> &'static str {
        r#"CreatedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMonitormodifiedat;
impl CfnMonitormodifiedat {
    pub fn att_name(&self) -> &'static str {
        r#"ModifiedAt"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMonitormonitorarn;
impl CfnMonitormonitorarn {
    pub fn att_name(&self) -> &'static str {
        r#"MonitorArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMonitorprocessingstatus;
impl CfnMonitorprocessingstatus {
    pub fn att_name(&self) -> &'static str {
        r#"ProcessingStatus"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnMonitorprocessingstatusinfo;
impl CfnMonitorprocessingstatusinfo {
    pub fn att_name(&self) -> &'static str {
        r#"ProcessingStatusInfo"#
    }
}

impl cfn_resources::CfnResource for CfnMonitor {
    fn type_string(&self) -> &'static str {
        "AWS::InternetMonitor::Monitor"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.internet_measurements_log_delivery
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The InternetMeasurementsLogDelivery property type specifies Property description not available. for an AWS::InternetMonitor::Monitor.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct InternetMeasurementsLogDelivery {
    ///
    /// The configuration information for publishing Amazon CloudWatch Internet Monitor internet measurements to Amazon S3. The configuration 			includes the bucket name and (optionally) bucket prefix for the S3 bucket to store the measurements, and the delivery status. 			The delivery status is ENABLED if you choose to deliver internet measurements to an S3 bucket, and DISABLED otherwise.
    ///
    /// Required: No
    ///
    /// Type: S3Config
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Config")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub s3_config: Option<S3Config>,
}

impl cfn_resources::CfnResource for InternetMeasurementsLogDelivery {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.s3_config
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The configuration for publishing Amazon CloudWatch Internet Monitor internet measurements to Amazon S3. The configuration 			includes the bucket name and (optionally) bucket prefix for the S3 bucket to store the measurements, and the delivery status. 			The delivery status is ENABLED if you choose to deliver internet measurements to S3 logs, and DISABLED otherwise.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct S3Config {
    ///
    /// The Amazon S3 bucket name for internet measurements publishing.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub bucket_name: Option<cfn_resources::StrVal>,

    ///
    /// An optional Amazon S3 bucket prefix for internet measurements publishing.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub bucket_prefix: Option<cfn_resources::StrVal>,

    ///
    /// The status of publishing Internet Monitor internet measurements to an Amazon S3 bucket. The delivery status is ENABLED 			if you choose to deliver internet measurements to an S3 bucket, and DISABLED otherwise.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "LogDeliveryStatus")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub log_delivery_status: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for S3Config {
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
