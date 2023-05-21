/// The AWS::RDS::DBProxyEndpoint resource creates or updates a DB proxy endpoint. You can use custom proxy endpoints to access a proxy through a different       VPC than the proxy's default VPC.
///
/// For more information about RDS Proxy, see                AWS::RDS::DBProxy.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDBProxyEndpoint {
    ///
    /// The name of the DB proxy endpoint to create.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBProxyEndpointName")]
    pub dbproxy_endpoint_name: cfn_resources::StrVal,

    ///
    /// The name of the DB proxy associated with the DB proxy endpoint that you create.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DBProxyName")]
    pub dbproxy_name: cfn_resources::StrVal,

    ///
    /// An optional set of key-value pairs to associate arbitrary data of your choosing with the proxy.
    ///
    /// Required: No
    ///
    /// Type: List of TagFormat
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagFormat>>,

    ///
    /// A value that indicates whether the DB proxy endpoint can be used for read/write or read-only operations.
    ///
    /// Valid Values: READ_WRITE | READ_ONLY
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_role: Option<DBProxyEndpointTargetRoleEnum>,

    ///
    /// The VPC security group IDs for the DB proxy endpoint that you create. You can     specify a different set of security group IDs than for the original DB proxy.     The default is the default security group for the VPC.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,

    ///
    /// The VPC subnet IDs for the DB proxy endpoint that you create. You can specify a     different set of subnet IDs than for the original DB proxy.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcSubnetIds")]
    pub vpc_subnet_ids: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub enum DBProxyEndpointTargetRoleEnum {
    /// READ_WRITE
    #[serde(rename = "READ_WRITE")]
    Readwrite,

    /// READ_ONLY
    #[serde(rename = "READ_ONLY")]
    Readonly,
}

impl Default for DBProxyEndpointTargetRoleEnum {
    fn default() -> Self {
        DBProxyEndpointTargetRoleEnum::Readwrite
    }
}

impl cfn_resources::CfnResource for CfnDBProxyEndpoint {
    fn type_string(&self) -> &'static str {
        "AWS::RDS::DBProxyEndpoint"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Metadata assigned to a DB proxy endpoint consisting of a key-value pair.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TagFormat {
    ///
    /// A value is the optional value of the tag. The string value can be 1-256 Unicode       characters in length and can't be prefixed with aws:. The string can contain only the       set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex:       "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<cfn_resources::StrVal>,

    ///
    /// Metadata assigned to a DB instance consisting of a key-value pair.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for TagFormat {
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
