

/// The AWS::MSK::ServerlessCluster resource Property description not available. for MSK.
#[derive(Default, serde::Serialize)]
pub struct CfnServerlessCluster {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: Map of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Tags")]
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
    pub cluster_name: String,

}


/// Details for SASL/IAM client authentication.
#[derive(Default, serde::Serialize)]
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


/// Includes all client authentication information.
#[derive(Default, serde::Serialize)]
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


/// The VpcConfig property type specifies Property description not available. for an AWS::MSK::ServerlessCluster.
#[derive(Default, serde::Serialize)]
pub struct VpcConfig {


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroups")]
    pub security_groups: Option<Vec<String>>,

}


/// Details for client authentication using SASL. To turn on SASL, you must also turn on EncryptionInTransit by setting inCluster to true. You must set clientBroker to either TLS or TLS_PLAINTEXT. If you choose TLS_PLAINTEXT, then you must also set unauthenticated to true.
#[derive(Default, serde::Serialize)]
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
