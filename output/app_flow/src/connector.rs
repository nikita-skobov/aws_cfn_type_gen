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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_label: Option<cfn_resources::StrVal>,

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
    pub connector_provisioning_type: ConnectorConnectorProvisioningTypeEnum,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    #[serde(skip_serializing)]
    pub att_connector_arn: CfnConnectorconnectorarn,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectorConnectorProvisioningTypeEnum {
    /// LAMBDA
    #[serde(rename = "LAMBDA")]
    Lambda,
}

impl Default for ConnectorConnectorProvisioningTypeEnum {
    fn default() -> Self {
        ConnectorConnectorProvisioningTypeEnum::Lambda
    }
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnectorconnectorarn;
impl CfnConnectorconnectorarn {
    pub fn att_name(&self) -> &'static str {
        r#"ConnectorArn"#
    }
}

impl cfn_resources::CfnResource for CfnConnector {
    fn type_string(&self) -> &'static str {
        "AWS::AppFlow::Connector"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.connector_label {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 256 as _ {
                    return Err(format!(
                        "Max validation failed on field 'connector_label'. {} is greater than 256",
                        s.len()
                    ));
                }
            }
        }

        self.connector_provisioning_config.validate()?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<LambdaConnectorProvisioningConfig>,
}

impl cfn_resources::CfnResource for ConnectorProvisioningConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.lambda.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
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
    pub lambda_arn: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for LambdaConnectorProvisioningConfig {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.lambda_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 512 as _ {
                return Err(format!(
                    "Max validation failed on field 'lambda_arn'. {} is greater than 512",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}
