

/// Creates a new connector profile associated with your AWS account. There is    a soft quota of 100 connector profiles per AWS account. If you need more    connector profiles than this quota allows, you can submit a request to the Amazon AppFlow    team through the Amazon AppFlow support channel. In each connector profile that you    create, you can provide the credentials and properties for only one connector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnector {


    /// 
    /// The label used for registering the connector.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 256
    ///
    /// Pattern: [a-zA-Z0-9][\w!@#.-]+
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorLabel")]
    pub connector_label: Option<String>,


    /// 
    /// The provisioning type used to register the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: LAMBDA
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProvisioningType")]
    pub connector_provisioning_type: String,


    /// 
    /// A description about the connector runtime setting.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\s\w/!@#+=.-]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// The configuration required for registering the connector.
    /// 
    /// Required: Yes
    ///
    /// Type: ConnectorProvisioningConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectorProvisioningConfig")]
    pub connector_provisioning_config: ConnectorProvisioningConfig,

}

impl cfn_resources::CfnResource for CfnConnector {
    fn type_string() -> &'static str {
        "AWS::AppFlow::Connector"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// Contains information about the configuration of the connector being registered.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectorProvisioningConfig {


    /// 
    /// Contains information about the configuration of the lambda which is being registered as    the connector.
    /// 
    /// Required: No
    ///
    /// Type: LambdaConnectorProvisioningConfig
    ///
    /// Update requires: No interruption
    #[serde(rename = "Lambda")]
    pub lambda: Option<LambdaConnectorProvisioningConfig>,

}


/// Contains information about the configuration of the lambda which is being registered as    the connector.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LambdaConnectorProvisioningConfig {


    /// 
    /// Lambda ARN of the connector being registered.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 512
    ///
    /// Pattern: arn:aws:.*:.*:[0-9]+:.*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LambdaArn")]
    pub lambda_arn: String,

}
