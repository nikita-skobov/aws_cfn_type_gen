/// The AWS::MSK::ServerlessCluster resource Property description not available. for MSK.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServerlessCluster {
    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: ClientAuthentication
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClientAuthentication")]
    pub client_authentication: ClientAuthentication,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ClusterName")]
    pub cluster_name: cfn_resources::StrVal,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of VpcConfig
    ///
    /// Update requires: Replacement
    #[serde(rename = "VpcConfigs")]
    pub vpc_configs: Vec<VpcConfig>,

    #[serde(skip_serializing)]
    pub att_arn: CfnServerlessClusterarn,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnServerlessClusterarn;
impl CfnServerlessClusterarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnServerlessCluster {
    fn type_string(&self) -> &'static str {
        "AWS::MSK::ServerlessCluster"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.client_authentication.validate()?;

        Ok(())
    }
}

/// Includes all client authentication information.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ClientAuthentication {
    ///
    /// Details for client authentication using SASL. To turn on SASL, you must also turn on EncryptionInTransit by setting inCluster to true. You must set clientBroker to either TLS or TLS_PLAINTEXT. If you choose TLS_PLAINTEXT, then you must also set unauthenticated to true.
    ///
    /// Required: Yes
    ///
    /// Type: Sasl
    ///
    /// Update requires: Replacement
    #[serde(rename = "Sasl")]
    pub sasl: Sasl,
}

impl cfn_resources::CfnResource for ClientAuthentication {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.sasl.validate()?;

        Ok(())
    }
}

/// Details for SASL/IAM client authentication.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Iam {
    ///
    /// SASL/IAM authentication is enabled or not.
    ///
    /// Required: Yes
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

impl cfn_resources::CfnResource for Iam {
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

/// Details for client authentication using SASL. To turn on SASL, you must also turn on EncryptionInTransit by setting inCluster to true. You must set clientBroker to either TLS or TLS_PLAINTEXT. If you choose TLS_PLAINTEXT, then you must also set unauthenticated to true.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Sasl {
    ///
    /// Details for ClientAuthentication using IAM.
    ///
    /// Required: Yes
    ///
    /// Type: Iam
    ///
    /// Update requires: Replacement
    #[serde(rename = "Iam")]
    pub iam: Iam,
}

impl cfn_resources::CfnResource for Sasl {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.iam.validate()?;

        Ok(())
    }
}

/// The VpcConfig property type specifies Property description not available. for an AWS::MSK::ServerlessCluster.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VpcConfig {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,

    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

impl cfn_resources::CfnResource for VpcConfig {
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
