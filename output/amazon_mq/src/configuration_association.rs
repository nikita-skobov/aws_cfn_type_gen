

/// Use the AWS CloudFormation AWS::AmazonMQ::ConfigurationAssociation resource    to associate a configuration with a broker, or return information about the specified    ConfigurationAssociation. Only use one per broker, and don't use a configuration on the broker    resource if you have associated a configuration with that broker.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConfigurationAssociation {


    /// 
    /// The broker to associate with a configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Broker")]
    pub broker: String,


    /// 
    /// The configuration to associate with a broker.
    /// 
    /// Required: Yes
    ///
    /// Type: ConfigurationId
    ///
    /// Update requires: No interruption
    #[serde(rename = "Configuration")]
    pub configuration: ConfigurationId,

}



impl cfn_resources::CfnResource for CfnConfigurationAssociation {
    fn type_string() -> &'static str {
        "AWS::AmazonMQ::ConfigurationAssociation"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The ConfigurationId property type specifies a configuration Id and the    revision of a configuration.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConfigurationId {


    /// 
    /// The unique ID that Amazon MQ generates for the configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Id")]
    pub id: String,


    /// 
    /// The revision number of the configuration.
    /// 
    /// Required: Yes
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "Revision")]
    pub revision: i64,

}


