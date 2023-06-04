/// The AWS::MediaConvert::JobTemplate resource is an AWS Elemental MediaConvert resource       type that you can use to generate transcoding jobs.
///
/// When you declare this entity in your AWS CloudFormation template, you pass in your       transcoding job settings in JSON or YAML format. This settings specification must be       formed in a particular way that conforms to AWS Elemental MediaConvert job validation. For       more information about creating a job template model for the SettingsJson       property, see the Remarks section later in this topic.
///
/// For information about job templates,       see Working with AWS Elemental MediaConvert Job Templates in the AWS Elemental MediaConvert User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnJobTemplate {
    ///
    /// Accelerated transcoding can significantly speed up jobs with long, visually complex       content. Outputs that use this feature incur pro-tier pricing. For information about       feature limitations, For more information, see Job Limitations for Accelerated Transcoding in AWS Elemental MediaConvert in the AWS Elemental MediaConvert User Guide.
    ///
    /// Required: No
    ///
    /// Type: AccelerationSettings
    ///
    /// Update requires: No interruption
    #[serde(rename = "AccelerationSettings")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub acceleration_settings: Option<AccelerationSettings>,

    ///
    /// Optional. A category for the job template you are creating
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub category: Option<cfn_resources::StrVal>,

    ///
    /// Optional. A description of the job template you are creating.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// Optional. Configuration for a destination queue to which the job can hop once a       customer-defined minimum wait time has passed. For more information, see Setting Up Queue Hopping to Avoid Long Waits in the AWS Elemental MediaConvert User Guide.
    ///
    /// Required: No
    ///
    /// Type: List of HopDestination
    ///
    /// Update requires: No interruption
    #[serde(rename = "HopDestinations")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub hop_destinations: Option<Vec<HopDestination>>,

    ///
    /// The name of the job template you are creating.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// Specify the relative priority for this job. In any given queue, the service begins       processing the job with the highest value first. When more than one job has the same       priority, the service begins processing the job that you submitted first. If you don't       specify a priority, the service uses the default value 0. Minimum: -50 Maximum:       50
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub priority: Option<i64>,

    ///
    /// Optional. The queue that jobs created from this template are assigned to. Specify the       Amazon Resource Name (ARN) of the queue. For example,       arn:aws:mediaconvert:us-west-2:505474453218:queues/Default. If you don't specify this,       jobs will go to the default queue.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub queue: Option<cfn_resources::StrVal>,

    ///
    /// Specify, in JSON format, the transcoding job settings for this job template. This       specification must conform to the AWS Elemental MediaConvert job validation. For       information about forming this specification, see the Remarks section later in this       topic.
    ///
    /// For more information about MediaConvert job templates, see Working with AWS Elemental MediaConvert Job Templates in the           AWS Elemental MediaConvert User Guide.
    ///
    /// Required: Yes
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "SettingsJson")]
    pub settings_json: serde_json::Value,

    ///
    /// Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events.       Set the interval, in seconds, between status updates. MediaConvert sends an update at       this interval from the time the service begins processing your job to the time it       completes the transcode or encounters an error.
    ///
    /// Specify one of the following enums:
    ///
    /// SECONDS_10
    ///
    /// SECONDS_12
    ///
    /// SECONDS_15
    ///
    /// SECONDS_20
    ///
    /// SECONDS_30
    ///
    /// SECONDS_60
    ///
    /// SECONDS_120
    ///
    /// SECONDS_180
    ///
    /// SECONDS_240
    ///
    /// SECONDS_300
    ///
    /// SECONDS_360
    ///
    /// SECONDS_420
    ///
    /// SECONDS_480
    ///
    /// SECONDS_540
    ///
    /// SECONDS_600
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "StatusUpdateInterval")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub status_update_interval: Option<cfn_resources::StrVal>,

    ///
    /// An array of key-value pairs to apply to this resource.
    ///
    /// For more information, see Tag.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<serde_json::Value>,

    #[serde(skip_serializing)]
    pub att_arn: CfnJobTemplatearn,

    #[serde(skip_serializing)]
    pub att_name: CfnJobTemplatename,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnJobTemplatearn;
impl CfnJobTemplatearn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnJobTemplatename;
impl CfnJobTemplatename {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnJobTemplate {
    fn type_string(&self) -> &'static str {
        "AWS::MediaConvert::JobTemplate"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.acceleration_settings
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.priority {
            if *the_val > 50 as _ {
                return Err(format!(
                    "Max validation failed on field 'priority'. {} is greater than 50",
                    the_val
                ));
            }
        }

        Ok(())
    }
}

/// Accelerated transcoding can significantly speed up jobs with long, visually complex       content. Outputs that use this feature incur pro-tier pricing. For information about       feature limitations, For more information, see       Job         Limitations for Accelerated Transcoding in AWS Elemental MediaConvert in the AWS Elemental MediaConvert User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct AccelerationSettings {
    ///
    /// Specify the conditions when the service will run your job with accelerated transcoding.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Mode")]
    pub mode: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for AccelerationSettings {
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

/// Optional. Configuration for a destination queue to which the job can hop once a       customer-defined minimum wait time has passed. For more information, see Setting Up Queue Hopping to Avoid Long Waits in the AWS Elemental MediaConvert User Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HopDestination {
    ///
    /// Optional. When you set up a job to use queue hopping, you can specify a different relative priority for the job in the destination queue. If you don't specify, the relative priority will remain the same as in the previous queue.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub priority: Option<i64>,

    ///
    /// Optional unless the job is submitted on the default queue. When you set up a job to use queue hopping, you can specify a destination queue. This queue cannot be the original queue to which the job is submitted. If the original queue isn't the default queue and you don't specify the destination queue, the job will move to the default queue.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Queue")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub queue: Option<cfn_resources::StrVal>,

    ///
    /// Required for setting up a job to use queue hopping. Minimum wait time in minutes until the job can hop to the destination queue. Valid range is 1 to 4320 minutes, inclusive.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "WaitMinutes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub wait_minutes: Option<i64>,
}

impl cfn_resources::CfnResource for HopDestination {
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
