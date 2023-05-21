

/// The AWS::Glue::Connection resource specifies an AWS Glue connection to a       data source. For more information, see Adding a Connection to Your Data Store       and Connection Structure in the AWS Glue Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnConnection {


    /// 
    /// The ID of the data catalog to create the catalog object in. Currently, this should be       the AWS account ID.
    /// 
    /// NoteTo specify the account ID, you can use the Ref intrinsic function         with the AWS::AccountId pseudo parameter. For example: !Ref           AWS::AccountId.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,


    /// 
    /// The connection that you want to create.
    /// 
    /// Required: Yes
    ///
    /// Type: ConnectionInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionInput")]
    pub connection_input: ConnectionInput,

}



impl cfn_resources::CfnResource for CfnConnection {
    fn type_string() -> &'static str {
        "AWS::Glue::Connection"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        self.connection_input.validate()?;

        Ok(())
    }
}

/// A structure that is used to specify a connection to create or update.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ConnectionInput {


    /// 
    /// These key-value pairs define parameters for the connection.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionProperties")]
    pub connection_properties: Option<serde_json::Value>,


    /// 
    /// The type of the connection. Currently, these types are supported:
    /// 
    /// JDBC - Designates a connection to a database through Java Database Connectivity (JDBC).                  JDBC Connections use the following ConnectionParameters.                                                         Required: All of (HOST, PORT, JDBC_ENGINE) or JDBC_CONNECTION_URL.                     Required: All of (USERNAME, PASSWORD) or SECRET_ID.                     Optional: JDBC_ENFORCE_SSL, CUSTOM_JDBC_CERT, CUSTOM_JDBC_CERT_STRING, SKIP_CUSTOM_JDBC_CERT_VALIDATION. These parameters are used to configure SSL with JDBC.                                  KAFKA - Designates a connection to an Apache Kafka streaming platform.                  KAFKA Connections use the following ConnectionParameters.                                                                                       Required: KAFKA_BOOTSTRAP_SERVERS.                     Optional: KAFKA_SSL_ENABLED, KAFKA_CUSTOM_CERT, KAFKA_SKIP_CUSTOM_CERT_VALIDATION. These parameters are used to configure SSL with KAFKA.                     Optional: KAFKA_CLIENT_KEYSTORE, KAFKA_CLIENT_KEYSTORE_PASSWORD, KAFKA_CLIENT_KEY_PASSWORD, ENCRYPTED_KAFKA_CLIENT_KEYSTORE_PASSWORD, ENCRYPTED_KAFKA_CLIENT_KEY_PASSWORD. These parameters are used to configure TLS client configuration with SSL in KAFKA.                     Optional: KAFKA_SASL_MECHANISM. Can be specified as SCRAM-SHA-512, GSSAPI, or AWS_MSK_IAM.                     Optional: KAFKA_SASL_SCRAM_USERNAME, KAFKA_SASL_SCRAM_PASSWORD, ENCRYPTED_KAFKA_SASL_SCRAM_PASSWORD. These parameters are used to configure SASL/SCRAM-SHA-512 authentication with KAFKA.                     Optional: KAFKA_SASL_GSSAPI_KEYTAB, KAFKA_SASL_GSSAPI_KRB5_CONF, KAFKA_SASL_GSSAPI_SERVICE, KAFKA_SASL_GSSAPI_PRINCIPAL. These parameters are used to configure SASL/GSSAPI authentication with KAFKA.                                  MONGODB - Designates a connection to a MongoDB document database.                  MONGODB Connections use the following ConnectionParameters.                                               Required: CONNECTION_URL.                     Required: All of (USERNAME, PASSWORD) or SECRET_ID.                                  NETWORK - Designates a network connection to a data source within an Amazon Virtual Private Cloud environment (Amazon VPC).                  NETWORK Connections do not require ConnectionParameters. Instead, provide a PhysicalConnectionRequirements.                        MARKETPLACE - Uses configuration settings contained in a connector purchased from AWS Marketplace to read from and write to data stores that are not natively supported by AWS Glue.                  MARKETPLACE Connections use the following ConnectionParameters.                                               Required: CONNECTOR_TYPE, CONNECTOR_URL, CONNECTOR_CLASS_NAME, CONNECTION_URL.                     Required for JDBC             CONNECTOR_TYPE connections: All of (USERNAME, PASSWORD) or SECRET_ID.                                  CUSTOM - Uses configuration settings contained in a custom connector to read from and write to data stores that are not natively supported by AWS Glue.
    /// 
    /// SFTP is not supported.
    /// 
    /// For more information about how optional ConnectionProperties are used to configure features in AWS Glue, consult AWS Glue connection properties.
    /// 
    /// For more information about how optional ConnectionProperties are used to configure features in AWS Glue Studio, consult Using connectors and connections.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Allowed values: CUSTOM | JDBC | KAFKA | MARKETPLACE | MONGODB | NETWORK | SFTP
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionType")]
    pub connection_type: ConnectionInputConnectionTypeEnum,


    /// 
    /// The description of the connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 2048
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Description")]
    pub description: Option<String>,


    /// 
    /// A list of criteria that can be used in selecting this connection.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 10
    ///
    /// Update requires: No interruption
    #[serde(rename = "MatchCriteria")]
    pub match_criteria: Option<Vec<String>>,


    /// 
    /// The name of the connection. Connection will not function as expected without a name.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// A map of physical connection requirements, such as virtual private cloud (VPC) and     SecurityGroup, that are needed to successfully make this connection.
    /// 
    /// Required: No
    ///
    /// Type: PhysicalConnectionRequirements
    ///
    /// Update requires: No interruption
    #[serde(rename = "PhysicalConnectionRequirements")]
    pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,

}


#[derive(Clone, Debug, serde::Serialize)]
pub enum ConnectionInputConnectionTypeEnum {

    /// CUSTOM
    #[serde(rename = "CUSTOM")]
    Custom,

    /// JDBC
    #[serde(rename = "JDBC")]
    Jdbc,

    /// KAFKA
    #[serde(rename = "KAFKA")]
    Kafka,

    /// MARKETPLACE
    #[serde(rename = "MARKETPLACE")]
    Marketplace,

    /// MONGODB
    #[serde(rename = "MONGODB")]
    Mongodb,

    /// NETWORK
    #[serde(rename = "NETWORK")]
    Network,

    /// SFTP
    #[serde(rename = "SFTP")]
    Sftp,

}

impl Default for ConnectionInputConnectionTypeEnum {
    fn default() -> Self {
        ConnectionInputConnectionTypeEnum::Custom
    }
}


impl cfn_resources::CfnResource for ConnectionInput {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.description {

        if the_val.len() > 2048 as _ {
            return Err(format!("Max validation failed on field 'description'. {} is greater than 2048", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.description {

        if the_val.len() < 0 as _ {
            return Err(format!("Min validation failed on field 'description'. {} is less than 0", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.match_criteria {

        if the_val.len() > 10 as _ {
            return Err(format!("Max validation failed on field 'match_criteria'. {} is greater than 10", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'name'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.name {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'name'. {} is less than 1", the_val.len()));
        }

        }
        
        self.physical_connection_requirements.as_ref().map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Specifies the physical requirements for a connection.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PhysicalConnectionRequirements {


    /// 
    /// The connection's Availability Zone. This field is redundant because the specified subnet    implies the Availability Zone to be used. Currently the field must be populated, but it will    be deprecated in the future.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: Option<String>,


    /// 
    /// The security group ID list used by the connection.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Maximum: 50
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecurityGroupIdList")]
    pub security_group_id_list: Option<Vec<String>>,


    /// 
    /// The subnet ID used by the connection.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "SubnetId")]
    pub subnet_id: Option<String>,

}



impl cfn_resources::CfnResource for PhysicalConnectionRequirements {
    fn type_string() -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }

    fn validate(&self) -> Result<(), String> {

        if let Some(the_val) = &self.availability_zone {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'availability_zone'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.availability_zone {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'availability_zone'. {} is less than 1", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.security_group_id_list {

        if the_val.len() > 50 as _ {
            return Err(format!("Max validation failed on field 'security_group_id_list'. {} is greater than 50", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.subnet_id {

        if the_val.len() > 255 as _ {
            return Err(format!("Max validation failed on field 'subnet_id'. {} is greater than 255", the_val.len()));
        }

        }
        
        if let Some(the_val) = &self.subnet_id {

        if the_val.len() < 1 as _ {
            return Err(format!("Min validation failed on field 'subnet_id'. {} is less than 1", the_val.len()));
        }

        }
        
        Ok(())
    }
}