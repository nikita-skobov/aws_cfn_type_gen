/// Creates a new MSK configuration. To see an example of how to use this operation, first save the following text to a file and name the file config-file.txt.
///
/// Now run the following Python 3.6 script in the folder where you saved config-file.txt. This script uses the properties specified in config-file.txt to create a configuration named SalesClusterConfiguration. This configuration can work with Apache Kafka versions 1.1.1 and 2.1.0.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnConfiguration {
    ///
    /// The description of the configuration.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "KafkaVersionsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_versions_list: Option<Vec<String>>,

    ///
    /// The name of the configuration. Configuration names are strings that match the regex "^[0-9A-Za-z][0-9A-Za-z-]{0,}$".
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: cfn_resources::StrVal,

    ///
    /// Contents of the server.properties file. When using the API, you must ensure that the contents of the file are base64 encoded.         When using the console, the SDK, or the CLI, the contents of server.properties can be in plaintext.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ServerProperties")]
    pub server_properties: cfn_resources::StrVal,

    #[serde(skip_serializing)]
    pub att_arn: CfnConfigurationarn,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnConfigurationarn;
impl CfnConfigurationarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

impl cfn_resources::CfnResource for CfnConfiguration {
    fn type_string(&self) -> &'static str {
        "AWS::MSK::Configuration"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
