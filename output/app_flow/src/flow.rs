

/// The AWS::AppFlow::Flow resource is an Amazon AppFlow resource type that    specifies a new flow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnFlow {


    /// 
    /// A user-entered description of the flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\w!@#\-.?,\s]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The configuration that controls how Amazon AppFlow places data in the destination    connector.
    /// 
    /// Required: Yes
    ///
    /// Type: List of DestinationFlowConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationFlowConfigList")]
    pub destination_flow_config_list: Vec<DestinationFlowConfig>,


    /// 
    /// The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens    (-) only.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9][\w!@#.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "FlowName")]
    pub flow_name: String,


    /// 
    /// Indicates the current status of the flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Active | Deleted | Deprecated | Draft | Errored | Suspended
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowStatus")]
    pub flow_status: Option<FlowFlowStatusEnum>,


    /// 
    /// The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for    encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS    key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 20
    ///
    /// Maximum: 2048
    ///
    /// Pattern: arn:aws:kms:.*:[0-9]+:.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KMSArn")]
    pub kmsarn: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: MetadataCatalogConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "MetadataCatalogConfig")]
    pub metadata_catalog_config: Option<MetadataCatalogConfig>,


    /// 
    /// Contains information about the configuration of the source connector used in the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: SourceFlowConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFlowConfig")]
    pub source_flow_config: SourceFlowConfig,


    /// 
    /// The tags used to organize, track, or control access for your flow.
    /// 
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,


    /// 
    /// A list of tasks that Amazon AppFlow performs while transferring the data in the flow    run.
    /// 
    /// Required: Yes
    ///
    /// Type: List of Task
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tasks")]
    pub tasks: Vec<Task>,


    /// 
    /// The trigger settings that determine how and when Amazon AppFlow runs the specified    flow.
    /// 
    /// Required: Yes
    ///
    /// Type: TriggerConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerConfig")]
    pub trigger_config: TriggerConfig,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum FlowFlowStatusEnum {

    /// Active
    #[serde(rename = "Active")]
    Active,

    /// Deleted
    #[serde(rename = "Deleted")]
    Deleted,

    /// Deprecated
    #[serde(rename = "Deprecated")]
    Deprecated,

    /// Draft
    #[serde(rename = "Draft")]
    Draft,

    /// Errored
    #[serde(rename = "Errored")]
    Errored,

    /// Suspended
    #[serde(rename = "Suspended")]
    Suspended,

}

impl Default for FlowFlowStatusEnum {
    fn default() -> Self {
        FlowFlowStatusEnum::Active
    }
}


impl cfn_resources::CfnResource for CfnFlow {
    fn type_string() -> &'static str {
        "AWS::AppFlow::Flow"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The aggregation settings that you can use to customize the output format of your flow    data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AggregationConfig {


    /// 
    /// Specifies whether Amazon AppFlow aggregates the flow records into a single file, or    leave them unaggregated.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: None | SingleFile
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationType")]
    pub aggregation_type: Option<AggregationConfigAggregationTypeEnum>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetFileSize")]
    pub target_file_size: Option<i64>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum AggregationConfigAggregationTypeEnum {

    /// None
    #[serde(rename = "None")]
    None,

    /// SingleFile
    #[serde(rename = "SingleFile")]
    Singlefile,

}

impl Default for AggregationConfigAggregationTypeEnum {
    fn default() -> Self {
        AggregationConfigAggregationTypeEnum::None
    }
}



/// The properties that are applied when Amplitude is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct AmplitudeSourceProperties {


    /// 
    /// The object specified in the Amplitude flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The operation to be performed on the provided source fields.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectorOperator {


    /// 
    /// The operation to be performed on the provided Amplitude source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BETWEEN
    ///
    /// Update requires: No interruption
    #[serde(rename = "Amplitude")]
    pub amplitude: Option<ConnectorOperatorAmplitudeEnum>,


    /// 
    /// Operators supported by the custom connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<ConnectorOperatorCustomConnectorEnum>,


    /// 
    /// The operation to be performed on the provided Datadog source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datadog")]
    pub datadog: Option<ConnectorOperatorDatadogEnum>,


    /// 
    /// The operation to be performed on the provided Dynatrace source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<ConnectorOperatorDynatraceEnum>,


    /// 
    /// The operation to be performed on the provided Google Analytics source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: BETWEEN | PROJECTION
    ///
    /// Update requires: No interruption
    #[serde(rename = "GoogleAnalytics")]
    pub google_analytics: Option<ConnectorOperatorGoogleAnalyticsEnum>,


    /// 
    /// The operation to be performed on the provided Infor Nexus source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<ConnectorOperatorInforNexusEnum>,


    /// 
    /// The operation to be performed on the provided Marketo source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | GREATER_THAN | LESS_THAN | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    pub marketo: Option<ConnectorOperatorMarketoEnum>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pardot")]
    pub pardot: Option<String>,


    /// 
    /// The operation to be performed on the provided Amazon S3 source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<ConnectorOperatorS3Enum>,


    /// 
    /// The operation to be performed on the provided SAPOData source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<ConnectorOperatorSAPODataEnum>,


    /// 
    /// The operation to be performed on the provided Salesforce source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<ConnectorOperatorSalesforceEnum>,


    /// 
    /// The operation to be performed on the provided ServiceNow source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<ConnectorOperatorServiceNowEnum>,


    /// 
    /// The operation to be performed on the provided Singular source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Singular")]
    pub singular: Option<ConnectorOperatorSingularEnum>,


    /// 
    /// The operation to be performed on the provided Slack source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slack")]
    pub slack: Option<ConnectorOperatorSlackEnum>,


    /// 
    /// The operation to be performed on the provided Trend Micro source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | DIVISION | EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trendmicro")]
    pub trendmicro: Option<ConnectorOperatorTrendmicroEnum>,


    /// 
    /// The operation to be performed on the provided Veeva source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | BETWEEN | CONTAINS | DIVISION | EQUAL_TO | GREATER_THAN | GREATER_THAN_OR_EQUAL_TO | LESS_THAN | LESS_THAN_OR_EQUAL_TO | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | NOT_EQUAL_TO | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Veeva")]
    pub veeva: Option<ConnectorOperatorVeevaEnum>,


    /// 
    /// The operation to be performed on the provided Zendesk source fields.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: ADDITION | DIVISION | GREATER_THAN | MASK_ALL | MASK_FIRST_N | MASK_LAST_N | MULTIPLICATION | NO_OP | PROJECTION | SUBTRACTION | VALIDATE_NON_NEGATIVE | VALIDATE_NON_NULL | VALIDATE_NON_ZERO | VALIDATE_NUMERIC
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ConnectorOperatorZendeskEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorAmplitudeEnum {

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

}

impl Default for ConnectorOperatorAmplitudeEnum {
    fn default() -> Self {
        ConnectorOperatorAmplitudeEnum::Between
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorCustomConnectorEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// CONTAINS
    #[serde(rename = "CONTAINS")]
    Contains,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// GREATER_THAN_OR_EQUAL_TO
    #[serde(rename = "GREATER_THAN_OR_EQUAL_TO")]
    Greaterthanorequalto,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,

    /// LESS_THAN_OR_EQUAL_TO
    #[serde(rename = "LESS_THAN_OR_EQUAL_TO")]
    Lessthanorequalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// NOT_EQUAL_TO
    #[serde(rename = "NOT_EQUAL_TO")]
    Notequalto,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorCustomConnectorEnum {
    fn default() -> Self {
        ConnectorOperatorCustomConnectorEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorDatadogEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorDatadogEnum {
    fn default() -> Self {
        ConnectorOperatorDatadogEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorDynatraceEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorDynatraceEnum {
    fn default() -> Self {
        ConnectorOperatorDynatraceEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorGoogleAnalyticsEnum {

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

}

impl Default for ConnectorOperatorGoogleAnalyticsEnum {
    fn default() -> Self {
        ConnectorOperatorGoogleAnalyticsEnum::Between
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorInforNexusEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorInforNexusEnum {
    fn default() -> Self {
        ConnectorOperatorInforNexusEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorMarketoEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorMarketoEnum {
    fn default() -> Self {
        ConnectorOperatorMarketoEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorS3Enum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// GREATER_THAN_OR_EQUAL_TO
    #[serde(rename = "GREATER_THAN_OR_EQUAL_TO")]
    Greaterthanorequalto,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,

    /// LESS_THAN_OR_EQUAL_TO
    #[serde(rename = "LESS_THAN_OR_EQUAL_TO")]
    Lessthanorequalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// NOT_EQUAL_TO
    #[serde(rename = "NOT_EQUAL_TO")]
    Notequalto,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorS3Enum {
    fn default() -> Self {
        ConnectorOperatorS3Enum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorSAPODataEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// CONTAINS
    #[serde(rename = "CONTAINS")]
    Contains,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// GREATER_THAN_OR_EQUAL_TO
    #[serde(rename = "GREATER_THAN_OR_EQUAL_TO")]
    Greaterthanorequalto,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,

    /// LESS_THAN_OR_EQUAL_TO
    #[serde(rename = "LESS_THAN_OR_EQUAL_TO")]
    Lessthanorequalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// NOT_EQUAL_TO
    #[serde(rename = "NOT_EQUAL_TO")]
    Notequalto,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorSAPODataEnum {
    fn default() -> Self {
        ConnectorOperatorSAPODataEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorSalesforceEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// CONTAINS
    #[serde(rename = "CONTAINS")]
    Contains,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// GREATER_THAN_OR_EQUAL_TO
    #[serde(rename = "GREATER_THAN_OR_EQUAL_TO")]
    Greaterthanorequalto,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,

    /// LESS_THAN_OR_EQUAL_TO
    #[serde(rename = "LESS_THAN_OR_EQUAL_TO")]
    Lessthanorequalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// NOT_EQUAL_TO
    #[serde(rename = "NOT_EQUAL_TO")]
    Notequalto,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorSalesforceEnum {
    fn default() -> Self {
        ConnectorOperatorSalesforceEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorServiceNowEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// CONTAINS
    #[serde(rename = "CONTAINS")]
    Contains,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// GREATER_THAN_OR_EQUAL_TO
    #[serde(rename = "GREATER_THAN_OR_EQUAL_TO")]
    Greaterthanorequalto,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,

    /// LESS_THAN_OR_EQUAL_TO
    #[serde(rename = "LESS_THAN_OR_EQUAL_TO")]
    Lessthanorequalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// NOT_EQUAL_TO
    #[serde(rename = "NOT_EQUAL_TO")]
    Notequalto,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorServiceNowEnum {
    fn default() -> Self {
        ConnectorOperatorServiceNowEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorSingularEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorSingularEnum {
    fn default() -> Self {
        ConnectorOperatorSingularEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorSlackEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// GREATER_THAN_OR_EQUAL_TO
    #[serde(rename = "GREATER_THAN_OR_EQUAL_TO")]
    Greaterthanorequalto,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,

    /// LESS_THAN_OR_EQUAL_TO
    #[serde(rename = "LESS_THAN_OR_EQUAL_TO")]
    Lessthanorequalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorSlackEnum {
    fn default() -> Self {
        ConnectorOperatorSlackEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorTrendmicroEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorTrendmicroEnum {
    fn default() -> Self {
        ConnectorOperatorTrendmicroEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorVeevaEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// BETWEEN
    #[serde(rename = "BETWEEN")]
    Between,

    /// CONTAINS
    #[serde(rename = "CONTAINS")]
    Contains,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// EQUAL_TO
    #[serde(rename = "EQUAL_TO")]
    Equalto,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// GREATER_THAN_OR_EQUAL_TO
    #[serde(rename = "GREATER_THAN_OR_EQUAL_TO")]
    Greaterthanorequalto,

    /// LESS_THAN
    #[serde(rename = "LESS_THAN")]
    Lessthan,

    /// LESS_THAN_OR_EQUAL_TO
    #[serde(rename = "LESS_THAN_OR_EQUAL_TO")]
    Lessthanorequalto,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// NOT_EQUAL_TO
    #[serde(rename = "NOT_EQUAL_TO")]
    Notequalto,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorVeevaEnum {
    fn default() -> Self {
        ConnectorOperatorVeevaEnum::Addition
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorOperatorZendeskEnum {

    /// ADDITION
    #[serde(rename = "ADDITION")]
    Addition,

    /// DIVISION
    #[serde(rename = "DIVISION")]
    Division,

    /// GREATER_THAN
    #[serde(rename = "GREATER_THAN")]
    Greaterthan,

    /// MASK_ALL
    #[serde(rename = "MASK_ALL")]
    Maskall,

    /// MASK_FIRST_N
    #[serde(rename = "MASK_FIRST_N")]
    Maskfirstn,

    /// MASK_LAST_N
    #[serde(rename = "MASK_LAST_N")]
    Masklastn,

    /// MULTIPLICATION
    #[serde(rename = "MULTIPLICATION")]
    Multiplication,

    /// NO_OP
    #[serde(rename = "NO_OP")]
    Noop,

    /// PROJECTION
    #[serde(rename = "PROJECTION")]
    Projection,

    /// SUBTRACTION
    #[serde(rename = "SUBTRACTION")]
    Subtraction,

    /// VALIDATE_NON_NEGATIVE
    #[serde(rename = "VALIDATE_NON_NEGATIVE")]
    Validatenonnegative,

    /// VALIDATE_NON_NULL
    #[serde(rename = "VALIDATE_NON_NULL")]
    Validatenonnull,

    /// VALIDATE_NON_ZERO
    #[serde(rename = "VALIDATE_NON_ZERO")]
    Validatenonzero,

    /// VALIDATE_NUMERIC
    #[serde(rename = "VALIDATE_NUMERIC")]
    Validatenumeric,

}

impl Default for ConnectorOperatorZendeskEnum {
    fn default() -> Self {
        ConnectorOperatorZendeskEnum::Addition
    }
}



/// The properties that are applied when the custom connector is being used as a    destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomConnectorDestinationProperties {


    /// 
    /// The custom properties that are specific to the connector when it's used as a destination    in the flow.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomProperties")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The entity specified in the custom connector as a destination in the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityName")]
    pub entity_name: String,


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the custom connector as destination.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The name of the field that Amazon AppFlow uses as an ID when performing a write    operation such as update, delete, or upsert.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,


    /// 
    /// Specifies the type of write operation to be performed in the custom connector when it's    used as destination.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DELETE | INSERT | UPDATE | UPSERT
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<CustomConnectorDestinationPropertiesWriteOperationTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum CustomConnectorDestinationPropertiesWriteOperationTypeEnum {

    /// DELETE
    #[serde(rename = "DELETE")]
    Delete,

    /// INSERT
    #[serde(rename = "INSERT")]
    Insert,

    /// UPDATE
    #[serde(rename = "UPDATE")]
    Update,

    /// UPSERT
    #[serde(rename = "UPSERT")]
    Upsert,

}

impl Default for CustomConnectorDestinationPropertiesWriteOperationTypeEnum {
    fn default() -> Self {
        CustomConnectorDestinationPropertiesWriteOperationTypeEnum::Delete
    }
}



/// The properties that are applied when the custom connector is being used as a    source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CustomConnectorSourceProperties {


    /// 
    /// Custom properties that are required to use the custom connector as a source.
    /// 
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomProperties")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,


    /// 
    /// The entity specified in the custom connector as a source in the flow.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "EntityName")]
    pub entity_name: String,

}




/// The properties that are applied when Datadog is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatadogSourceProperties {


    /// 
    /// The object specified in the Datadog flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// This stores the information that is required to query a particular connector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DestinationConnectorProperties {


    /// 
    /// The properties that are required to query the custom Connector.
    /// 
    /// Required: No
    ///
    /// Type: CustomConnectorDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorDestinationProperties>,


    /// 
    /// The properties required to query Amazon EventBridge.
    /// 
    /// Required: No
    ///
    /// Type: EventBridgeDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "EventBridge")]
    pub event_bridge: Option<EventBridgeDestinationProperties>,


    /// 
    /// The properties required to query Amazon Lookout for Metrics.
    /// 
    /// Required: No
    ///
    /// Type: LookoutMetricsDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "LookoutMetrics")]
    pub lookout_metrics: Option<LookoutMetricsDestinationProperties>,


    /// 
    /// The properties required to query Marketo.
    /// 
    /// Required: No
    ///
    /// Type: MarketoDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoDestinationProperties>,


    /// 
    /// The properties required to query Amazon Redshift.
    /// 
    /// Required: No
    ///
    /// Type: RedshiftDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Redshift")]
    pub redshift: Option<RedshiftDestinationProperties>,


    /// 
    /// The properties required to query Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: S3DestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<S3DestinationProperties>,


    /// 
    /// The properties required to query SAPOData.
    /// 
    /// Required: No
    ///
    /// Type: SAPODataDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataDestinationProperties>,


    /// 
    /// The properties required to query Salesforce.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceDestinationProperties>,


    /// 
    /// The properties required to query Snowflake.
    /// 
    /// Required: No
    ///
    /// Type: SnowflakeDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Snowflake")]
    pub snowflake: Option<SnowflakeDestinationProperties>,


    /// 
    /// The properties required to query Upsolver.
    /// 
    /// Required: No
    ///
    /// Type: UpsolverDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Upsolver")]
    pub upsolver: Option<UpsolverDestinationProperties>,


    /// 
    /// The properties required to query Zendesk.
    /// 
    /// Required: No
    ///
    /// Type: ZendeskDestinationProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskDestinationProperties>,

}




/// Contains information about the configuration of destination connectors present in the    flow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DestinationFlowConfig {


    /// 
    /// The API version that the destination connector uses.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiVersion")]
    pub api_version: Option<String>,


    /// 
    /// The name of the connector profile. This name must be unique for each connector profile in    the AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\w/!@#+=.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProfileName")]
    pub connector_profile_name: Option<String>,


    /// 
    /// The type of destination connector, such as Sales force, Amazon S3, and so on.
    /// 
    /// Allowed Values: EventBridge | Redshift | S3 | Salesforce |     Snowflake
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorType")]
    pub connector_type: DestinationFlowConfigConnectorTypeEnum,


    /// 
    /// This stores the information that is required to query a particular connector.
    /// 
    /// Required: Yes
    ///
    /// Type: DestinationConnectorProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationConnectorProperties")]
    pub destination_connector_properties: DestinationConnectorProperties,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum DestinationFlowConfigConnectorTypeEnum {

    /// EventBridge
    #[serde(rename = "EventBridge")]
    Eventbridge,

    /// Redshift
    #[serde(rename = "Redshift")]
    Redshift,

    /// S3
    #[serde(rename = "S3")]
    S3,

    /// Salesforce
    #[serde(rename = "Salesforce")]
    Salesforce,

    /// Snowflake
    #[serde(rename = "Snowflake")]
    Snowflake,

}

impl Default for DestinationFlowConfigConnectorTypeEnum {
    fn default() -> Self {
        DestinationFlowConfigConnectorTypeEnum::Eventbridge
    }
}



/// The properties that are applied when Dynatrace is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DynatraceSourceProperties {


    /// 
    /// The object specified in the Dynatrace flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The settings that determine how Amazon AppFlow handles an error when placing data in    the destination. For example, this setting would determine if the flow should fail after one    insertion error, or continue and attempt to insert every record regardless of the initial    failure. ErrorHandlingConfig is a part of the destination connector details.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ErrorHandlingConfig {


    /// 
    /// Specifies the name of the Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,


    /// 
    /// Specifies the Amazon S3 bucket prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// Specifies if the flow should fail after the first instance of a failure when attempting    to place data in the destination.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "FailOnFirstError")]
    pub fail_on_first_error: Option<bool>,

}




/// The properties that are applied when Amazon EventBridge is being used as a    destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EventBridgeDestinationProperties {


    /// 
    /// The object specified in the Amplitude flow source.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The object specified in the Amazon EventBridge flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The GlueDataCatalog property type specifies Property description not available. for an AWS::AppFlow::Flow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GlueDataCatalog {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RoleArn")]
    pub role_arn: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TablePrefix")]
    pub table_prefix: String,

}




/// The properties that are applied when Google Analytics is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GoogleAnalyticsSourceProperties {


    /// 
    /// The object specified in the Google Analytics flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// Specifies the configuration used when importing incremental records from the source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct IncrementalPullConfig {


    /// 
    /// A field that specifies the date time or timestamp field as the criteria to use when    importing incremental records from the source.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatetimeTypeFieldName")]
    pub datetime_type_field_name: Option<String>,

}




/// The properties that are applied when Infor Nexus is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InforNexusSourceProperties {


    /// 
    /// The object specified in the Infor Nexus flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The properties that are applied when Amazon Lookout for Metrics is used as a destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LookoutMetricsDestinationProperties {


    /// 
    /// The object specified in the Amazon Lookout for Metrics flow destination.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: Option<String>,

}




/// The properties that Amazon AppFlow applies when you use Marketo as a flow    destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MarketoDestinationProperties {


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the destination. For example, this setting would determine if the flow should fail after one    insertion error, or continue and attempt to insert every record regardless of the initial    failure. ErrorHandlingConfig is a part of the destination connector details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The object specified in the Marketo flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The properties that are applied when Marketo is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MarketoSourceProperties {


    /// 
    /// The object specified in the Marketo flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The MetadataCatalogConfig property type specifies Property description not available. for an AWS::AppFlow::Flow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct MetadataCatalogConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: GlueDataCatalog
    ///
    /// Update requires: No interruption
    #[serde(rename = "GlueDataCatalog")]
    pub glue_data_catalog: Option<GlueDataCatalog>,

}




/// The PardotSourceProperties property type specifies Property description not available. for an AWS::AppFlow::Flow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PardotSourceProperties {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// Specifies elements that Amazon AppFlow includes in the file and folder names in the flow    destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrefixConfig {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PathPrefixHierarchy")]
    pub path_prefix_hierarchy: Option<Vec<String>>,


    /// 
    /// Determines the level of granularity for the date and time that's included in the prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DAY | HOUR | MINUTE | MONTH | YEAR
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixFormat")]
    pub prefix_format: Option<PrefixConfigPrefixFormatEnum>,


    /// 
    /// Determines the format of the prefix, and whether it applies to the file name, file path,    or both.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: FILENAME | PATH | PATH_AND_FILENAME
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixType")]
    pub prefix_type: Option<PrefixConfigPrefixTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum PrefixConfigPrefixFormatEnum {

    /// DAY
    #[serde(rename = "DAY")]
    Day,

    /// HOUR
    #[serde(rename = "HOUR")]
    Hour,

    /// MINUTE
    #[serde(rename = "MINUTE")]
    Minute,

    /// MONTH
    #[serde(rename = "MONTH")]
    Month,

    /// YEAR
    #[serde(rename = "YEAR")]
    Year,

}

impl Default for PrefixConfigPrefixFormatEnum {
    fn default() -> Self {
        PrefixConfigPrefixFormatEnum::Day
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum PrefixConfigPrefixTypeEnum {

    /// FILENAME
    #[serde(rename = "FILENAME")]
    Filename,

    /// PATH
    #[serde(rename = "PATH")]
    Path,

    /// PATH_AND_FILENAME
    #[serde(rename = "PATH_AND_FILENAME")]
    Pathandfilename,

}

impl Default for PrefixConfigPrefixTypeEnum {
    fn default() -> Self {
        PrefixConfigPrefixTypeEnum::Filename
    }
}



/// The properties that are applied when Amazon Redshift is being used as a destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RedshiftDestinationProperties {


    /// 
    /// The object key for the bucket in which Amazon AppFlow places the destination files.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the Amazon Redshift destination. For example, this setting would determine if the flow    should fail after one insertion error, or continue and attempt to insert every record    regardless of the initial failure. ErrorHandlingConfig is a part of the    destination connector details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The intermediate bucket that Amazon AppFlow uses when moving data into Amazon Redshift.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntermediateBucketName")]
    pub intermediate_bucket_name: String,


    /// 
    /// The object specified in the Amazon Redshift flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The properties that are applied when Amazon S3 is used as a destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3DestinationProperties {


    /// 
    /// The Amazon S3 bucket name in which Amazon AppFlow places the transferred    data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,


    /// 
    /// The object key for the destination bucket in which Amazon AppFlow places the files.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The configuration that determines how Amazon AppFlow should format the flow output    data when Amazon S3 is used as the destination.
    /// 
    /// Required: No
    ///
    /// Type: S3OutputFormatConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3OutputFormatConfig")]
    pub s3_output_format_config: Option<S3OutputFormatConfig>,

}




/// When you use Amazon S3 as the source, the configuration format that you provide    the flow input data.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3InputFormatConfig {


    /// 
    /// The file type that Amazon AppFlow gets from your Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3InputFileType")]
    pub s3_input_file_type: Option<S3InputFormatConfigS3InputFileTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum S3InputFormatConfigS3InputFileTypeEnum {

    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// JSON
    #[serde(rename = "JSON")]
    Json,

}

impl Default for S3InputFormatConfigS3InputFileTypeEnum {
    fn default() -> Self {
        S3InputFormatConfigS3InputFileTypeEnum::Csv
    }
}



/// The configuration that determines how Amazon AppFlow should format the flow output    data when Amazon S3 is used as the destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3OutputFormatConfig {


    /// 
    /// The aggregation settings that you can use to customize the output format of your flow    data.
    /// 
    /// Required: No
    ///
    /// Type: AggregationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationConfig")]
    pub aggregation_config: Option<AggregationConfig>,


    /// 
    /// Indicates the file type that Amazon AppFlow places in the Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON | PARQUET
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileType")]
    pub file_type: Option<S3OutputFormatConfigFileTypeEnum>,


    /// 
    /// Determines the prefix that Amazon AppFlow applies to the folder name in the Amazon S3 bucket. You can name folders according to the flow frequency and date.
    /// 
    /// Required: No
    ///
    /// Type: PrefixConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixConfig")]
    pub prefix_config: Option<PrefixConfig>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreserveSourceDataTyping")]
    pub preserve_source_data_typing: Option<bool>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum S3OutputFormatConfigFileTypeEnum {

    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// JSON
    #[serde(rename = "JSON")]
    Json,

    /// PARQUET
    #[serde(rename = "PARQUET")]
    Parquet,

}

impl Default for S3OutputFormatConfigFileTypeEnum {
    fn default() -> Self {
        S3OutputFormatConfigFileTypeEnum::Csv
    }
}



/// The properties that are applied when Amazon S3 is being used as the flow source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct S3SourceProperties {


    /// 
    /// The Amazon S3 bucket name where the source files are stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,


    /// 
    /// The object key for the Amazon S3 bucket in which the source files are stored.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: String,


    /// 
    /// When you use Amazon S3 as the source, the configuration format that you provide    the flow input data.
    /// 
    /// Required: No
    ///
    /// Type: S3InputFormatConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3InputFormatConfig")]
    pub s3_input_format_config: Option<S3InputFormatConfig>,

}




/// The properties that are applied when using SAPOData as a flow destination
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SAPODataDestinationProperties {


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the destination. For example, this setting would determine if the flow should fail after one    insertion error, or continue and attempt to insert every record regardless of the initial    failure. ErrorHandlingConfig is a part of the destination connector details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// A list of field names that can be used as an ID field when performing a write operation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,


    /// 
    /// The object path specified in the SAPOData flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectPath")]
    pub object_path: String,


    /// 
    /// Determines how Amazon AppFlow handles the success response that it gets from the    connector after placing data.
    /// 
    /// For example, this setting would determine where to write the response from a destination    connector upon a successful insert operation.
    /// 
    /// Required: No
    ///
    /// Type: SuccessResponseHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuccessResponseHandlingConfig")]
    pub success_response_handling_config: Option<SuccessResponseHandlingConfig>,


    /// 
    /// The possible write operations in the destination connector. When this value is not    provided, this defaults to the INSERT operation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<String>,

}




/// The properties that are applied when using SAPOData as a flow source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SAPODataSourceProperties {


    /// 
    /// The object path specified in the SAPOData flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ObjectPath")]
    pub object_path: String,

}




/// The properties that are applied when Salesforce is being used as a destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceDestinationProperties {


    /// 
    /// Specifies which Salesforce API is used by Amazon AppFlow when your flow transfers    data to Salesforce.
    /// 
    /// AUTOMATIC                  The default. Amazon AppFlow selects which API to use based on the number of       records that your flow transfers to Salesforce. If your flow transfers fewer than 1,000       records, Amazon AppFlow uses Salesforce REST API. If your flow transfers 1,000       records or more, Amazon AppFlow uses Salesforce Bulk API 2.0.          Each of these Salesforce APIs structures data differently. If Amazon AppFlow       selects the API automatically, be aware that, for recurring flows, the data output might       vary from one flow run to the next. For example, if a flow runs daily, it might use REST       API on one day to transfer 900 records, and it might use Bulk API 2.0 on the next day to       transfer 1,100 records. For each of these flow runs, the respective Salesforce API       formats the data differently. Some of the differences include how dates are formatted       and null values are represented. Also, Bulk API 2.0 doesn't transfer Salesforce compound       fields.          By choosing this option, you optimize flow performance for both small and large data       transfers, but the tradeoff is inconsistent formatting in the output.                       BULKV2                  Amazon AppFlow uses only Salesforce Bulk API 2.0. This API runs asynchronous       data transfers, and it's optimal for large sets of data. By choosing this option, you       ensure that your flow writes consistent output, but you optimize performance only for       large data transfers.          Note that Bulk API 2.0 does not transfer Salesforce compound fields.                       REST_SYNC                  Amazon AppFlow uses only Salesforce REST API. By choosing this option, you       ensure that your flow writes consistent output, but you decrease performance for large       data transfers that are better suited for Bulk API 2.0. In some cases, if your flow       attempts to transfer a vary large set of data, it might fail with a timed out       error.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | BULKV2 | REST_SYNC
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTransferApi")]
    pub data_transfer_api: Option<SalesforceDestinationPropertiesDataTransferApiEnum>,


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the Salesforce destination. For example, this setting would determine if the flow should fail    after one insertion error, or continue and attempt to insert every record regardless of the    initial failure. ErrorHandlingConfig is a part of the destination connector    details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The name of the field that Amazon AppFlow uses as an ID when performing a write    operation such as update or delete.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,


    /// 
    /// The object specified in the Salesforce flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,


    /// 
    /// This specifies the type of write operation to be performed in Salesforce. When the value    is UPSERT, then idFieldNames is required.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: DELETE | INSERT | UPDATE | UPSERT
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<SalesforceDestinationPropertiesWriteOperationTypeEnum>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SalesforceDestinationPropertiesDataTransferApiEnum {

    /// AUTOMATIC
    #[serde(rename = "AUTOMATIC")]
    Automatic,

    /// BULKV2
    #[serde(rename = "BULKV2")]
    Bulkv2,

    /// REST_SYNC
    #[serde(rename = "REST_SYNC")]
    Restsync,

}

impl Default for SalesforceDestinationPropertiesDataTransferApiEnum {
    fn default() -> Self {
        SalesforceDestinationPropertiesDataTransferApiEnum::Automatic
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum SalesforceDestinationPropertiesWriteOperationTypeEnum {

    /// DELETE
    #[serde(rename = "DELETE")]
    Delete,

    /// INSERT
    #[serde(rename = "INSERT")]
    Insert,

    /// UPDATE
    #[serde(rename = "UPDATE")]
    Update,

    /// UPSERT
    #[serde(rename = "UPSERT")]
    Upsert,

}

impl Default for SalesforceDestinationPropertiesWriteOperationTypeEnum {
    fn default() -> Self {
        SalesforceDestinationPropertiesWriteOperationTypeEnum::Delete
    }
}



/// The properties that are applied when Salesforce is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SalesforceSourceProperties {


    /// 
    /// Specifies which Salesforce API is used by Amazon AppFlow when your flow transfers    data from Salesforce.
    /// 
    /// AUTOMATIC                  The default. Amazon AppFlow selects which API to use based on the number of       records that your flow transfers from Salesforce. If your flow transfers fewer than       1,000,000 records, Amazon AppFlow uses Salesforce REST API. If your flow transfers       1,000,000 records or more, Amazon AppFlow uses Salesforce Bulk API 2.0.          Each of these Salesforce APIs structures data differently. If Amazon AppFlow       selects the API automatically, be aware that, for recurring flows, the data output might       vary from one flow run to the next. For example, if a flow runs daily, it might use REST       API on one day to transfer 900,000 records, and it might use Bulk API 2.0 on the next       day to transfer 1,100,000 records. For each of these flow runs, the respective       Salesforce API formats the data differently. Some of the differences include how dates       are formatted and null values are represented. Also, Bulk API 2.0 doesn't transfer       Salesforce compound fields.          By choosing this option, you optimize flow performance for both small and large data       transfers, but the tradeoff is inconsistent formatting in the output.                       BULKV2                  Amazon AppFlow uses only Salesforce Bulk API 2.0. This API runs asynchronous       data transfers, and it's optimal for large sets of data. By choosing this option, you       ensure that your flow writes consistent output, but you optimize performance only for       large data transfers.          Note that Bulk API 2.0 does not transfer Salesforce compound fields.                       REST_SYNC                  Amazon AppFlow uses only Salesforce REST API. By choosing this option, you       ensure that your flow writes consistent output, but you decrease performance for large       data transfers that are better suited for Bulk API 2.0. In some cases, if your flow       attempts to transfer a vary large set of data, it might fail wituh a timed out       error.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: AUTOMATIC | BULKV2 | REST_SYNC
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataTransferApi")]
    pub data_transfer_api: Option<SalesforceSourcePropertiesDataTransferApiEnum>,


    /// 
    /// The flag that enables dynamic fetching of new (recently added) fields in the Salesforce    objects while running a flow.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "EnableDynamicFieldUpdate")]
    pub enable_dynamic_field_update: Option<bool>,


    /// 
    /// Indicates whether Amazon AppFlow includes deleted files in the flow run.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeDeletedRecords")]
    pub include_deleted_records: Option<bool>,


    /// 
    /// The object specified in the Salesforce flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SalesforceSourcePropertiesDataTransferApiEnum {

    /// AUTOMATIC
    #[serde(rename = "AUTOMATIC")]
    Automatic,

    /// BULKV2
    #[serde(rename = "BULKV2")]
    Bulkv2,

    /// REST_SYNC
    #[serde(rename = "REST_SYNC")]
    Restsync,

}

impl Default for SalesforceSourcePropertiesDataTransferApiEnum {
    fn default() -> Self {
        SalesforceSourcePropertiesDataTransferApiEnum::Automatic
    }
}



/// Specifies the configuration details of a schedule-triggered flow as defined by the user.    Currently, these settings only apply to the Scheduled trigger type.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ScheduledTriggerProperties {


    /// 
    /// Specifies whether a scheduled flow has an incremental data transfer or a complete data    transfer for each flow run.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: Complete | Incremental
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataPullMode")]
    pub data_pull_mode: Option<ScheduledTriggerPropertiesDataPullModeEnum>,


    /// 
    /// Specifies the date range for the records to import from the connector in the first flow    run.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "FirstExecutionFrom")]
    pub first_execution_from: Option<f64>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "FlowErrorDeactivationThreshold")]
    pub flow_error_deactivation_threshold: Option<i64>,


    /// 
    /// The time at which the scheduled flow ends. The time is formatted as a timestamp that    follows the ISO 8601 standard, such as 2022-04-27T13:00:00-07:00.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleEndTime")]
    pub schedule_end_time: Option<f64>,


    /// 
    /// The scheduling expression that determines the rate at which the schedule will run, for    example rate(5minutes).
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    pub schedule_expression: String,


    /// 
    /// Specifies the optional offset that is added to the time interval for a schedule-triggered    flow.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleOffset")]
    pub schedule_offset: Option<f64>,


    /// 
    /// The time at which the scheduled flow starts. The time is formatted as a timestamp that    follows the ISO 8601 standard, such as 2022-04-26T13:00:00-07:00.
    /// 
    /// Required: No
    ///
    /// Type: Double
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleStartTime")]
    pub schedule_start_time: Option<f64>,


    /// 
    /// Specifies the time zone used when referring to the dates and times of a scheduled flow,    such as America/New_York. This time zone is only a descriptive label. It doesn't    affect how Amazon AppFlow interprets the timestamps that you specify to schedule the    flow.
    /// 
    /// If you want to schedule a flow by using times in a particular time zone, indicate the time    zone as a UTC offset in your timestamps. For example, the UTC offsets for the     America/New_York timezone are -04:00 EDT and -05:00     EST.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ScheduledTriggerPropertiesDataPullModeEnum {

    /// Complete
    #[serde(rename = "Complete")]
    Complete,

    /// Incremental
    #[serde(rename = "Incremental")]
    Incremental,

}

impl Default for ScheduledTriggerPropertiesDataPullModeEnum {
    fn default() -> Self {
        ScheduledTriggerPropertiesDataPullModeEnum::Complete
    }
}



/// The properties that are applied when ServiceNow is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ServiceNowSourceProperties {


    /// 
    /// The object specified in the ServiceNow flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The properties that are applied when Singular is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SingularSourceProperties {


    /// 
    /// The object specified in the Singular flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The properties that are applied when Slack is being used as a source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SlackSourceProperties {


    /// 
    /// The object specified in the Slack flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The properties that are applied when Snowflake is being used as a destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SnowflakeDestinationProperties {


    /// 
    /// The object key for the destination bucket in which Amazon AppFlow places the files.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the Snowflake destination. For example, this setting would determine if the flow should fail    after one insertion error, or continue and attempt to insert every record regardless of the    initial failure. ErrorHandlingConfig is a part of the destination connector    details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// The intermediate bucket that Amazon AppFlow uses when moving data into Snowflake.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "IntermediateBucketName")]
    pub intermediate_bucket_name: String,


    /// 
    /// The object specified in the Snowflake flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// Specifies the information that is required to query a particular connector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceConnectorProperties {


    /// 
    /// Specifies the information that is required for querying Amplitude.
    /// 
    /// Required: No
    ///
    /// Type: AmplitudeSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Amplitude")]
    pub amplitude: Option<AmplitudeSourceProperties>,


    /// 
    /// The properties that are applied when the custom connector is being used as a    source.
    /// 
    /// Required: No
    ///
    /// Type: CustomConnectorSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "CustomConnector")]
    pub custom_connector: Option<CustomConnectorSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Datadog.
    /// 
    /// Required: No
    ///
    /// Type: DatadogSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Datadog")]
    pub datadog: Option<DatadogSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Dynatrace.
    /// 
    /// Required: No
    ///
    /// Type: DynatraceSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Dynatrace")]
    pub dynatrace: Option<DynatraceSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Google Analytics.
    /// 
    /// Required: No
    ///
    /// Type: GoogleAnalyticsSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "GoogleAnalytics")]
    pub google_analytics: Option<GoogleAnalyticsSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Infor Nexus.
    /// 
    /// Required: No
    ///
    /// Type: InforNexusSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "InforNexus")]
    pub infor_nexus: Option<InforNexusSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Marketo.
    /// 
    /// Required: No
    ///
    /// Type: MarketoSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Marketo")]
    pub marketo: Option<MarketoSourceProperties>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: PardotSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Pardot")]
    pub pardot: Option<PardotSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: S3SourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3")]
    pub s3: Option<S3SourceProperties>,


    /// 
    /// The properties that are applied when using SAPOData as a flow source.
    /// 
    /// Required: No
    ///
    /// Type: SAPODataSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SAPOData")]
    pub sapodata: Option<SAPODataSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Salesforce.
    /// 
    /// Required: No
    ///
    /// Type: SalesforceSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Salesforce")]
    pub salesforce: Option<SalesforceSourceProperties>,


    /// 
    /// Specifies the information that is required for querying ServiceNow.
    /// 
    /// Required: No
    ///
    /// Type: ServiceNowSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServiceNow")]
    pub service_now: Option<ServiceNowSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Singular.
    /// 
    /// Required: No
    ///
    /// Type: SingularSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Singular")]
    pub singular: Option<SingularSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Slack.
    /// 
    /// Required: No
    ///
    /// Type: SlackSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Slack")]
    pub slack: Option<SlackSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Trend Micro.
    /// 
    /// Required: No
    ///
    /// Type: TrendmicroSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Trendmicro")]
    pub trendmicro: Option<TrendmicroSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Veeva.
    /// 
    /// Required: No
    ///
    /// Type: VeevaSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Veeva")]
    pub veeva: Option<VeevaSourceProperties>,


    /// 
    /// Specifies the information that is required for querying Zendesk.
    /// 
    /// Required: No
    ///
    /// Type: ZendeskSourceProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "Zendesk")]
    pub zendesk: Option<ZendeskSourceProperties>,

}




/// Contains information about the configuration of the source connector used in the flow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SourceFlowConfig {


    /// 
    /// The API version of the connector when it's used as a source in the flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ApiVersion")]
    pub api_version: Option<String>,


    /// 
    /// The name of the connector profile. This name must be unique for each connector profile in    the AWS account.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [\w/!@#+=.-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProfileName")]
    pub connector_profile_name: Option<String>,


    /// 
    /// The type of connector, such as Salesforce, Amplitude, and so on.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Amplitude | CustomConnector | CustomerProfiles | Datadog | Dynatrace | EventBridge | Googleanalytics | Honeycode | Infornexus | LookoutMetrics | Marketo | Pardot | Redshift | S3 | Salesforce | SAPOData | Servicenow | Singular | Slack | Snowflake | Trendmicro | Upsolver | Veeva | Zendesk
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorType")]
    pub connector_type: SourceFlowConfigConnectorTypeEnum,


    /// 
    /// Defines the configuration for a scheduled incremental data pull. If a valid configuration    is provided, the fields specified in the configuration are used when querying for the    incremental data pull.
    /// 
    /// Required: No
    ///
    /// Type: IncrementalPullConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncrementalPullConfig")]
    pub incremental_pull_config: Option<IncrementalPullConfig>,


    /// 
    /// Specifies the information that is required to query a particular source connector.
    /// 
    /// Required: Yes
    ///
    /// Type: SourceConnectorProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceConnectorProperties")]
    pub source_connector_properties: SourceConnectorProperties,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum SourceFlowConfigConnectorTypeEnum {

    /// Amplitude
    #[serde(rename = "Amplitude")]
    Amplitude,

    /// CustomConnector
    #[serde(rename = "CustomConnector")]
    Customconnector,

    /// CustomerProfiles
    #[serde(rename = "CustomerProfiles")]
    Customerprofiles,

    /// Datadog
    #[serde(rename = "Datadog")]
    Datadog,

    /// Dynatrace
    #[serde(rename = "Dynatrace")]
    Dynatrace,

    /// EventBridge
    #[serde(rename = "EventBridge")]
    Eventbridge,

    /// Googleanalytics
    #[serde(rename = "Googleanalytics")]
    Googleanalytics,

    /// Honeycode
    #[serde(rename = "Honeycode")]
    Honeycode,

    /// Infornexus
    #[serde(rename = "Infornexus")]
    Infornexus,

    /// LookoutMetrics
    #[serde(rename = "LookoutMetrics")]
    Lookoutmetrics,

    /// Marketo
    #[serde(rename = "Marketo")]
    Marketo,

    /// Pardot
    #[serde(rename = "Pardot")]
    Pardot,

    /// Redshift
    #[serde(rename = "Redshift")]
    Redshift,

    /// S3
    #[serde(rename = "S3")]
    S3,

    /// Salesforce
    #[serde(rename = "Salesforce")]
    Salesforce,

    /// SAPOData
    #[serde(rename = "SAPOData")]
    Sapodata,

    /// Servicenow
    #[serde(rename = "Servicenow")]
    Servicenow,

    /// Singular
    #[serde(rename = "Singular")]
    Singular,

    /// Slack
    #[serde(rename = "Slack")]
    Slack,

    /// Snowflake
    #[serde(rename = "Snowflake")]
    Snowflake,

    /// Trendmicro
    #[serde(rename = "Trendmicro")]
    Trendmicro,

    /// Upsolver
    #[serde(rename = "Upsolver")]
    Upsolver,

    /// Veeva
    #[serde(rename = "Veeva")]
    Veeva,

    /// Zendesk
    #[serde(rename = "Zendesk")]
    Zendesk,

}

impl Default for SourceFlowConfigConnectorTypeEnum {
    fn default() -> Self {
        SourceFlowConfigConnectorTypeEnum::Amplitude
    }
}



/// Determines how Amazon AppFlow handles the success response that it gets from the    connector after placing data.
///
/// For example, this setting would determine where to write the response from the destination    connector upon a successful insert operation.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SuccessResponseHandlingConfig {


    /// 
    /// The name of the Amazon S3 bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 3
    ///
    /// Maximum: 63
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: Option<String>,


    /// 
    /// The Amazon S3 bucket prefix.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,

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




/// A class for modeling different type of tasks. Task implementation varies based on the     TaskType.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Task {


    /// 
    /// The operation to be performed on the provided source fields.
    /// 
    /// Required: No
    ///
    /// Type: ConnectorOperator
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorOperator")]
    pub connector_operator: Option<ConnectorOperator>,


    /// 
    /// A field in a destination connector, or a field value against which Amazon AppFlow    validates a source field.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "DestinationField")]
    pub destination_field: Option<String>,


    /// 
    /// The source fields to which a particular task is applied.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SourceFields")]
    pub source_fields: Vec<String>,


    /// 
    /// A map used to store task-related information. The execution service looks for particular    information based on the TaskType.
    /// 
    /// Required: No
    ///
    /// Type: List of TaskPropertiesObject
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskProperties")]
    pub task_properties: Option<Vec<TaskPropertiesObject>>,


    /// 
    /// Specifies the particular task implementation that Amazon AppFlow performs.
    /// 
    /// Allowed values: Arithmetic | Filter |     Map | Map_all | Mask | Merge |     Truncate | Validate
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TaskType")]
    pub task_type: TaskTaskTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TaskTaskTypeEnum {

    /// Arithmetic
    #[serde(rename = "Arithmetic")]
    Arithmetic,

    /// Filter
    #[serde(rename = "Filter")]
    Filter,

    /// Map
    #[serde(rename = "Map")]
    Map,

    /// Map_all
    #[serde(rename = "Map_all")]
    Mapall,

    /// Mask
    #[serde(rename = "Mask")]
    Mask,

    /// Merge
    #[serde(rename = "Merge")]
    Merge,

    /// Truncate
    #[serde(rename = "Truncate")]
    Truncate,

    /// Validate
    #[serde(rename = "Validate")]
    Validate,

}

impl Default for TaskTaskTypeEnum {
    fn default() -> Self {
        TaskTaskTypeEnum::Arithmetic
    }
}



/// A map used to store task-related information. The execution service looks for particular    information based on the TaskType.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TaskPropertiesObject {


    /// 
    /// The task property key.
    /// 
    /// Allowed Values: VALUE | VALUES | DATA_TYPE | UPPER_BOUND |     LOWER_BOUND | SOURCE_DATA_TYPE | DESTINATION_DATA_TYPE | VALIDATION_ACTION | MASK_VALUE |     MASK_LENGTH | TRUNCATE_LENGTH | MATH_OPERATION_FIELDS_ORDER | CONCAT_FORMAT |     SUBFIELD_CATEGORY_MAP | EXCLUDE_SOURCE_FIELDS_LIST
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    pub key: TaskPropertiesObjectKeyEnum,


    /// 
    /// The task property value.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    pub value: String,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TaskPropertiesObjectKeyEnum {

    /// VALUE
    #[serde(rename = "VALUE")]
    Value,

    /// VALUES
    #[serde(rename = "VALUES")]
    Values,

    /// DATA_TYPE
    #[serde(rename = "DATA_TYPE")]
    Datatype,

    /// UPPER_BOUND
    #[serde(rename = "UPPER_BOUND")]
    Upperbound,

    /// LOWER_BOUND
    #[serde(rename = "LOWER_BOUND")]
    Lowerbound,

    /// SOURCE_DATA_TYPE
    #[serde(rename = "SOURCE_DATA_TYPE")]
    Sourcedatatype,

    /// DESTINATION_DATA_TYPE
    #[serde(rename = "DESTINATION_DATA_TYPE")]
    Destinationdatatype,

    /// VALIDATION_ACTION
    #[serde(rename = "VALIDATION_ACTION")]
    Validationaction,

    /// MASK_VALUE
    #[serde(rename = "MASK_VALUE")]
    Maskvalue,

    /// MASK_LENGTH
    #[serde(rename = "MASK_LENGTH")]
    Masklength,

    /// TRUNCATE_LENGTH
    #[serde(rename = "TRUNCATE_LENGTH")]
    Truncatelength,

    /// MATH_OPERATION_FIELDS_ORDER
    #[serde(rename = "MATH_OPERATION_FIELDS_ORDER")]
    Mathoperationfieldsorder,

    /// CONCAT_FORMAT
    #[serde(rename = "CONCAT_FORMAT")]
    Concatformat,

    /// SUBFIELD_CATEGORY_MAP
    #[serde(rename = "SUBFIELD_CATEGORY_MAP")]
    Subfieldcategorymap,

    /// EXCLUDE_SOURCE_FIELDS_LIST
    #[serde(rename = "EXCLUDE_SOURCE_FIELDS_LIST")]
    Excludesourcefieldslist,

}

impl Default for TaskPropertiesObjectKeyEnum {
    fn default() -> Self {
        TaskPropertiesObjectKeyEnum::Value
    }
}



/// The properties that are applied when using Trend Micro as a flow source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TrendmicroSourceProperties {


    /// 
    /// The object specified in the Trend Micro flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The trigger settings that determine how and when Amazon AppFlow runs the specified    flow.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TriggerConfig {


    /// 
    /// Specifies the configuration details of a schedule-triggered flow as defined by the user.    Currently, these settings only apply to the Scheduled trigger type.
    /// 
    /// Required: No
    ///
    /// Type: ScheduledTriggerProperties
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerProperties")]
    pub trigger_properties: Option<ScheduledTriggerProperties>,


    /// 
    /// Specifies the type of flow trigger. This can be OnDemand,     Scheduled, or Event.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: Event | OnDemand | Scheduled
    ///
    /// Update requires: No interruption
    #[serde(rename = "TriggerType")]
    pub trigger_type: TriggerConfigTriggerTypeEnum,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum TriggerConfigTriggerTypeEnum {

    /// Event
    #[serde(rename = "Event")]
    Event,

    /// OnDemand
    #[serde(rename = "OnDemand")]
    Ondemand,

    /// Scheduled
    #[serde(rename = "Scheduled")]
    Scheduled,

}

impl Default for TriggerConfigTriggerTypeEnum {
    fn default() -> Self {
        TriggerConfigTriggerTypeEnum::Event
    }
}



/// The properties that are applied when Upsolver is used as a destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UpsolverDestinationProperties {


    /// 
    /// The Upsolver Amazon S3 bucket name in which Amazon AppFlow places the    transferred data.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 16
    ///
    /// Maximum: 63
    ///
    /// Pattern: ^(upsolver-appflow)\S*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketName")]
    pub bucket_name: String,


    /// 
    /// The object key for the destination Upsolver Amazon S3 bucket in which Amazon AppFlow places the files.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: .*
    ///
    /// Update requires: No interruption
    #[serde(rename = "BucketPrefix")]
    pub bucket_prefix: Option<String>,


    /// 
    /// The configuration that determines how data is formatted when Upsolver is used as the flow    destination.
    /// 
    /// Required: Yes
    ///
    /// Type: UpsolverS3OutputFormatConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3OutputFormatConfig")]
    pub s3_output_format_config: UpsolverS3OutputFormatConfig,

}




/// The configuration that determines how Amazon AppFlow formats the flow output data    when Upsolver is used as the destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct UpsolverS3OutputFormatConfig {


    /// 
    /// The aggregation settings that you can use to customize the output format of your flow    data.
    /// 
    /// Required: No
    ///
    /// Type: AggregationConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "AggregationConfig")]
    pub aggregation_config: Option<AggregationConfig>,


    /// 
    /// Indicates the file type that Amazon AppFlow places in the Upsolver Amazon S3    bucket.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: CSV | JSON | PARQUET
    ///
    /// Update requires: No interruption
    #[serde(rename = "FileType")]
    pub file_type: Option<UpsolverS3OutputFormatConfigFileTypeEnum>,


    /// 
    /// Specifies elements that Amazon AppFlow includes in the file and folder names in the flow    destination.
    /// 
    /// Required: Yes
    ///
    /// Type: PrefixConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "PrefixConfig")]
    pub prefix_config: PrefixConfig,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum UpsolverS3OutputFormatConfigFileTypeEnum {

    /// CSV
    #[serde(rename = "CSV")]
    Csv,

    /// JSON
    #[serde(rename = "JSON")]
    Json,

    /// PARQUET
    #[serde(rename = "PARQUET")]
    Parquet,

}

impl Default for UpsolverS3OutputFormatConfigFileTypeEnum {
    fn default() -> Self {
        UpsolverS3OutputFormatConfigFileTypeEnum::Csv
    }
}



/// The properties that are applied when using Veeva as a flow source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VeevaSourceProperties {


    /// 
    /// The document type specified in the Veeva document extract flow.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: [\s\w_-]+
    ///
    /// Update requires: No interruption
    #[serde(rename = "DocumentType")]
    pub document_type: Option<String>,


    /// 
    /// Boolean value to include All Versions of files in Veeva document extract flow.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeAllVersions")]
    pub include_all_versions: Option<bool>,


    /// 
    /// Boolean value to include file renditions in Veeva document extract flow.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeRenditions")]
    pub include_renditions: Option<bool>,


    /// 
    /// Boolean value to include source files in Veeva document extract flow.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "IncludeSourceFiles")]
    pub include_source_files: Option<bool>,


    /// 
    /// The object specified in the Veeva flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}




/// The properties that are applied when Zendesk is used as a destination.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ZendeskDestinationProperties {


    /// 
    /// The settings that determine how Amazon AppFlow handles an error when placing data in    the destination. For example, this setting would determine if the flow should fail after one    insertion error, or continue and attempt to insert every record regardless of the initial    failure. ErrorHandlingConfig is a part of the destination connector details.
    /// 
    /// Required: No
    ///
    /// Type: ErrorHandlingConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ErrorHandlingConfig")]
    pub error_handling_config: Option<ErrorHandlingConfig>,


    /// 
    /// A list of field names that can be used as an ID field when performing a write operation.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "IdFieldNames")]
    pub id_field_names: Option<Vec<String>>,


    /// 
    /// The object specified in the Zendesk flow destination.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,


    /// 
    /// The possible write operations in the destination connector. When this value is not    provided, this defaults to the INSERT operation.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "WriteOperationType")]
    pub write_operation_type: Option<String>,

}




/// The properties that are applied when using Zendesk as a flow source.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ZendeskSourceProperties {


    /// 
    /// The object specified in the Zendesk flow source.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: \S+
    ///
    /// Update requires: No interruption
    #[serde(rename = "Object")]
    pub object: String,

}


