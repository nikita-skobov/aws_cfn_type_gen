/// The     AWS::Greengrass::ConnectorDefinition resource represents a connector definition for AWS IoT Greengrass.   Connector definitions are used to organize your connector definition versions.
///
/// Connector definitions can reference multiple connector definition versions. All connector definition versions      must be associated with a connector definition. Each connector definition version can contain one or more connectors.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectorDefinition {
    ///
    /// The connector definition version to include when the connector definition is created.          A connector definition version contains a list of          connector property types.
    ///
    /// NoteTo associate a connector definition version after the connector definition is created, 				   create an AWS::Greengrass::ConnectorDefinitionVersion  				   resource and specify the ID of this connector definition.
    ///
    /// Required: No
    ///
    /// Type: ConnectorDefinitionVersion
    ///
    /// Update requires: Replacement
    #[serde(rename = "InitialVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_version: Option<ConnectorDefinitionVersion>,

    ///
    /// The name of the connector definition.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Application-specific metadata to attach to the connector definition. 		  You can use tags in IAM policies to control access to AWS IoT Greengrass resources. 		  You can also use tags to categorize your resources. For more information, see 		  Tagging Your AWS IoT Greengrass 		  Resources in the AWS IoT Greengrass Version 1 Developer Guide.
    ///
    /// This Json property type is processed as a map of key-value pairs. It uses the following format, which 		    is different from most Tags implementations in AWS CloudFormation templates.
    ///
    /// "Tags": {   "KeyName0": "value",   "KeyName1": "value",   "KeyName2": "value" }
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    #[serde(skip_serializing)]
    pub att_arn: CfnConnectorDefinitionarn,

    #[serde(skip_serializing)]
    pub att_id: CfnConnectorDefinitionid,

    #[serde(skip_serializing)]
    pub att_latest_version_arn: CfnConnectorDefinitionlatestversionarn,

    #[serde(skip_serializing)]
    pub att_name: CfnConnectorDefinitionname,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectorDefinitionarn;
impl CfnConnectorDefinitionarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectorDefinitionid;
impl CfnConnectorDefinitionid {
    pub fn att_name(&self) -> &'static str {
        r#"Id"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectorDefinitionlatestversionarn;
impl CfnConnectorDefinitionlatestversionarn {
    pub fn att_name(&self) -> &'static str {
        r#"LatestVersionArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectorDefinitionname;
impl CfnConnectorDefinitionname {
    pub fn att_name(&self) -> &'static str {
        r#"Name"#
    }
}

impl cfn_resources::CfnResource for CfnConnectorDefinition {
    fn type_string(&self) -> &'static str {
        "AWS::Greengrass::ConnectorDefinition"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.initial_version
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Connectors are modules that provide       built-in integration with local infrastructure, device protocols, AWS, and other cloud services. 	For more information, 	see Integrate with Services and Protocols Using Greengrass Connectors in the AWS IoT Greengrass Version 1 Developer Guide.
///
/// In an AWS CloudFormation template, the Connectors 		 property of the ConnectorDefinitionVersion property type contains a list       of Connector property types.
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
    pub connector_arn: cfn_resources::StrVal,

    ///
    /// A descriptive or arbitrary ID for the connector. This value must be unique within       the connector definition version. Maximum length is 128 characters with pattern [a-zA-Z0-9:_-]+.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Id")]
    pub id: cfn_resources::StrVal,

    ///
    /// The parameters or configuration used by the connector.
    ///
    /// For more information about connectors provided by AWS, see Greengrass Connectors Provided by AWS.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// A connector definition version contains a list of connectors.
///
/// In an AWS CloudFormation template, ConnectorDefinitionVersion is the property type of the InitialVersion property      in the AWS::Greengrass::ConnectorDefinition resource.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectorDefinitionVersion {
    ///
    /// The connectors in this version. Only one instance of a given connector can be added to           a connector definition version at a time.
    ///
    /// Required: Yes
    ///
    /// Type: List of Connector
    ///
    /// Update requires: Replacement
    #[serde(rename = "Connectors")]
    pub connectors: Vec<Connector>,
}

impl cfn_resources::CfnResource for ConnectorDefinitionVersion {
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
