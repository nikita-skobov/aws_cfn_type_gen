/// The     AWS::Greengrass::ConnectorDefinitionVersion resource represents a connector definition version for AWS IoT Greengrass.     A connector definition version contains a list of connectors.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectorDefinitionVersion {
    ///
    /// The ID of the connector definition associated with this version. This value is a GUID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorDefinitionId")]
    pub connector_definition_id: String,

    ///
    /// The connectors in this version. Only one instance of a given connector can be added to          the connector definition version at a time.
    ///
    /// Required: Yes
    ///
    /// Type: List of Connector
    ///
    /// Update requires: Replacement
    #[serde(rename = "Connectors")]
    pub connectors: Vec<Connector>,
}

impl cfn_resources::CfnResource for CfnConnectorDefinitionVersion {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::ConnectorDefinitionVersion"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Connectors are modules that provide       built-in integration with local infrastructure, device protocols, AWS, and other cloud services. 	For more information, 	see Integrate with Services and Protocols Using Greengrass Connectors in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Connectors 		 property of the AWS::Greengrass::ConnectorDefinitionVersion resource contains a      list of Connector property types.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Connector {
    ///
    /// The Amazon Resource Name (ARN) of the connector.
    ///
    /// For more information about connectors provided by AWS, see Greengrass Connectors Provided by AWS.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ConnectorArn")]
    pub connector_arn: String,

    ///
    /// A descriptive or arbitrary ID for the connector. This value must be unique within       the connector definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: String,

    ///
    /// The parameters or configuration that the connector uses.
    ///
    /// For more information about connectors provided by AWS, see Greengrass Connectors Provided by AWS.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,
}

impl cfn_resources::CfnResource for Connector {
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
