/// Sets the rotation schedule and Lambda rotation function for    a secret. For more information, see How rotation works.
///
/// For Amazon RDS master user credentials, see AWS::RDS::DBCluster MasterUserSecret.
///
/// For the rotation function, you have two options:
///
/// For database secrets, if you define    both the secret and the database or service in the AWS CloudFormation template, then    you need to define the AWS::SecretsManager::SecretTargetAttachment resource to populate the secret with    the connection details of the database or service before you attempt to configure    rotation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnRotationSchedule {
    ///
    /// Creates a new Lambda rotation    function based on one of the     Secrets Manager rotation function templates. To use a rotation function that already   exists, specify RotationLambdaARN instead.
    ///
    /// For Amazon RDS master user credentials, see AWS::RDS::DBCluster MasterUserSecret.
    ///
    /// Required: No
    ///
    /// Type: HostedRotationLambda
    ///
    /// Update requires: No interruption
    #[serde(rename = "HostedRotationLambda")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_rotation_lambda: Option<HostedRotationLambda>,

    ///
    /// Specifies whether to rotate the secret immediately or wait until the next scheduled    rotation window. The rotation schedule is defined in RotationRules.
    ///
    /// If you don't immediately rotate the secret, Secrets Manager tests the rotation    configuration by running the testSecret step of the Lambda rotation function. The test creates an     AWSPENDING version of the secret and then removes it.
    ///
    /// If you don't specify this value, then by default, Secrets Manager rotates the secret    immediately.
    ///
    /// Rotation is an asynchronous process. For more information, see How rotation    works.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotateImmediatelyOnUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_immediately_on_update: Option<bool>,

    ///
    /// The ARN of an existing Lambda rotation function. To specify a rotation function    that is also defined in this template, use the Ref    function.
    ///
    /// For Amazon RDS master user credentials, see AWS::RDS::DBCluster MasterUserSecret.
    ///
    /// To create a new rotation function based on one of the    Secrets Manager rotation function templates, specify HostedRotationLambda    instead.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotationLambdaARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<cfn_resources::StrVal>,

    ///
    /// A structure that defines the rotation configuration for this secret.
    ///
    /// Required: No
    ///
    /// Type: RotationRules
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRules>,

    ///
    /// The ARN or name of the secret to rotate.
    ///
    /// To reference a secret also created in this template, use the Ref    function with the secret's logical ID.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecretId")]
    pub secret_id: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for CfnRotationSchedule {
    fn type_string(&self) -> &'static str {
        "AWS::SecretsManager::RotationSchedule"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.hosted_rotation_lambda
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        self.rotation_rules
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// Creates a new Lambda rotation    function based on one of the     Secrets Manager rotation function templates.
///
/// You must specify Transform:     AWS::SecretsManager-2020-07-23 at the beginning of the CloudFormation    template.
///
/// For Amazon RDS master user credentials, see AWS::RDS::DBCluster MasterUserSecret.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HostedRotationLambda {
    ///
    /// A string of the characters that you don't want in the password.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludeCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_characters: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the KMS key that Secrets Manager uses to encrypt the secret. If you don't    specify this value, then Secrets Manager uses the key aws/secretsmanager. If     aws/secretsmanager doesn't yet exist, then Secrets Manager creates it for you    automatically the first time it encrypts the secret value.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the secret that contains superuser credentials, if you use the    Alternating users rotation strategy. CloudFormation grants the execution role for the Lambda rotation function GetSecretValue permission to the secret in this property. For more information, see Lambda rotation function execution role permissions for Secrets Manager.
    ///
    /// You must create the superuser secret before you can set this property.
    ///
    /// You must also include the superuser secret ARN as a key in the JSON of the rotating secret so that the Lambda rotation function can find it. CloudFormation does not hardcode secret ARNs in the Lambda rotation function, so you can use the function to rotate multiple secrets. For more information, see JSON structure of Secrets Manager secrets.
    ///
    /// You can specify MasterSecretArn or SuperuserSecretArn but not both. They represent the same superuser secret.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterSecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_secret_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the KMS key that Secrets Manager used to encrypt the superuser secret, if    you use the alternating users strategy and the superuser secret is encrypted with a customer managed key. You don't need to specify this property if the superuser secret is encrypted using the key aws/secretsmanager. CloudFormation grants the execution role for the Lambda rotation function Decrypt, DescribeKey, and GenerateDataKey permission to the key in this property. For more information, see Lambda rotation function execution role permissions for Secrets Manager.
    ///
    /// You can specify MasterSecretKmsKeyArn or SuperuserSecretKmsKeyArn but not both. They represent the same superuser secret KMS key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "MasterSecretKmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_secret_kms_key_arn: Option<cfn_resources::StrVal>,

    ///
    /// The name of the Lambda rotation function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotationLambdaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_name: Option<cfn_resources::StrVal>,

    ///
    /// The rotation template to base the rotation function on, one of the following:
    ///
    /// MySQLSingleUser to use the template SecretsManagerRDSMySQLRotationSingleUser.        MySQLMultiUser to use the template SecretsManagerRDSMySQLRotationMultiUser.         PostgreSQLSingleUser to use the template SecretsManagerRDSPostgreSQLRotationSingleUser        PostgreSQLMultiUser to use the template SecretsManagerRDSPostgreSQLRotationMultiUser.        OracleSingleUser to use the template SecretsManagerRDSOracleRotationSingleUser.        OracleMultiUser to use the template SecretsManagerRDSOracleRotationMultiUser.        MariaDBSingleUser to use the template SecretsManagerRDSMariaDBRotationSingleUser.        MariaDBMultiUser to use the template SecretsManagerRDSMariaDBRotationMultiUser.        SQLServerSingleUser to use the template SecretsManagerRDSSQLServerRotationSingleUser.        SQLServerMultiUser to use the template SecretsManagerRDSSQLServerRotationMultiUser.        RedshiftSingleUser to use the template SecretsManagerRedshiftRotationSingleUsr.        RedshiftMultiUser to use the template SecretsManagerRedshiftRotationMultiUser.        MongoDBSingleUser to use the template SecretsManagerMongoDBRotationSingleUser.        MongoDBMultiUser to use the template SecretsManagerMongoDBRotationMultiUser.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "RotationType")]
    pub rotation_type: cfn_resources::StrVal,

    ///
    /// The Python runtime version associated with the Lambda function.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the secret that contains superuser credentials, if you use the    Alternating users rotation strategy. CloudFormation grants the execution role for the Lambda rotation function GetSecretValue permission to the secret in this property. For more information, see Lambda rotation function execution role permissions for Secrets Manager.
    ///
    /// You must create the superuser secret before you can set this property.
    ///
    /// You must also include the superuser secret ARN as a key in the JSON of the rotating secret so that the Lambda rotation function can find it. CloudFormation does not hardcode secret ARNs in the Lambda rotation function, so you can use the function to rotate multiple secrets. For more information, see JSON structure of Secrets Manager secrets.
    ///
    /// You can specify MasterSecretArn or SuperuserSecretArn but not both. They represent the same superuser secret.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuperuserSecretArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superuser_secret_arn: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the KMS key that Secrets Manager used to encrypt the superuser secret, if    you use the alternating users strategy and the superuser secret is encrypted with a customer managed key. You don't need to specify this property if the superuser secret is encrypted using the key aws/secretsmanager. CloudFormation grants the execution role for the Lambda rotation function Decrypt, DescribeKey, and GenerateDataKey permission to the key in this property. For more information, see Lambda rotation function execution role permissions for Secrets Manager.
    ///
    /// You can specify MasterSecretKmsKeyArn or SuperuserSecretKmsKeyArn but not both. They represent the same superuser secret KMS key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "SuperuserSecretKmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superuser_secret_kms_key_arn: Option<cfn_resources::StrVal>,

    ///
    /// A comma-separated list of security group IDs applied to the target database.
    ///
    /// The template applies the same security groups as on the Lambda rotation function that is    created as part of this stack.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<cfn_resources::StrVal>,

    ///
    /// A comma separated list of VPC subnet IDs of the target database network. The Lambda    rotation function is in the same subnet group.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "VpcSubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnet_ids: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for HostedRotationLambda {
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

/// The rotation schedule and window. We recommend you use ScheduleExpression to       set a cron or rate expression for the schedule and Duration to set the length of       the rotation window.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RotationRules {
    ///
    /// The number of days between automatic scheduled rotations of the secret. You can use this    value to check that your secret meets your compliance guidelines for how often secrets must    be rotated.
    ///
    /// In DescribeSecret and ListSecrets, this value is calculated from    the rotation schedule after every successful rotation. In RotateSecret, you can    set the rotation schedule in RotationRules with AutomaticallyAfterDays    or ScheduleExpression, but not both.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Update requires: No interruption
    #[serde(rename = "AutomaticallyAfterDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_after_days: Option<i64>,

    ///
    /// The length of the rotation window in hours, for example 3h for a three    hour window. Secrets Manager rotates your secret at any time during this window. The window must not    extend into the next rotation window or the next UTC day. The window starts according to the ScheduleExpression. If you don't specify a Duration,    for a ScheduleExpression in hours, the window automatically closes after one    hour. For a ScheduleExpression in days, the window automatically closes at the    end of the UTC day. For    more information, including examples, see Schedule expressions    in Secrets Manager rotation in the Secrets Manager Users Guide.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<cfn_resources::StrVal>,

    ///
    /// A cron() or rate() expression that defines the schedule for    rotating your secret. Secrets Manager rotation schedules use UTC time zone. Secrets Manager rotates your secret any time during a rotation window.
    ///
    /// Secrets Manager rate() expressions represent the interval in hours or days that you    want to rotate your secret, for example rate(12 hours) or    rate(10 days). You can rotate a secret as often as every four hours. If you    use a rate() expression, the rotation    window starts at midnight. For a rate in hours, the default rotation window closes after one    hour. For a rate in days, the default rotation window closes at the end of the day. You can    set the Duration to change the rotation window. The rotation window must not    extend into the next UTC day or into the next rotation window.
    ///
    /// You can use a cron() expression to create a rotation schedule that is    more detailed than a rotation interval. For more information, including examples, see    Schedule expressions in    Secrets Manager rotation in the Secrets Manager Users Guide. For a cron expression    that represents a schedule in hours, the default rotation window closes after one hour. For    a cron expression that represents a schedule in days, the default rotation window closes at    the end of the day. You can set the Duration to change the rotation window. The    rotation window must not extend into the next UTC day or into the next rotation window.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for RotationRules {
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
