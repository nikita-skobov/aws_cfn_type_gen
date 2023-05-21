

/// The AWS::SecretsManager::SecretTargetAttachment resource completes the final    link between a Secrets Manager secret and the associated database by adding the database    connection information to the secret JSON. If you want to turn on automatic rotation    for a database credential secret, the secret must contain the database connection information.    For more information, see JSON structure     of Secrets Manager database credential secrets.
///
/// For Amazon RDS master user credentials, see AWS::RDS::DBCluster MasterUserSecret.
#[derive(Default, serde::Serialize)]
pub struct CfnSecretTargetAttachment {


    /// 
    /// A string that defines the type of service or database associated with the secret. This    value instructs Secrets Manager how to update the secret with the details of the service or    database. This value must be one of the following:
    /// 
    /// AWS::RDS::DBInstance        AWS::RDS::DBCluster        AWS::Redshift::Cluster        AWS::DocDB::DBInstance        AWS::DocDB::DBCluster
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetType")]
    pub target_type: String,


    /// 
    /// The ID of the database or cluster.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetId")]
    pub target_id: String,


    /// 
    /// The ARN or name of the secret. To reference a secret also created in this template, use    the see Ref    function with the secret's logical ID.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SecretId")]
    pub secret_id: String,

}
