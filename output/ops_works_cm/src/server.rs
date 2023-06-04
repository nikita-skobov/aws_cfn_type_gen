/// The AWS::OpsWorksCM::Server resource creates an AWS OpsWorks for Chef Automate or OpsWorks for Puppet Enterprise       configuration management server. For more      information, see Create a Chef Automate Server in AWS CloudFormation or Create a        Puppet Enterprise Master in AWS CloudFormation in the AWS OpsWorks User          Guide, and CreateServer in the AWS OpsWorks CM API     Reference.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnServer {
    ///
    /// Associate a public IP address with a server that you are launching. Valid values are true or false. The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: Replacement
    #[serde(rename = "AssociatePublicIpAddress")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub associate_public_ip_address: Option<bool>,

    ///
    /// If you specify this field, AWS OpsWorks CM creates the server by using the backup represented by BackupId.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 79
    ///
    /// Pattern: [a-zA-Z][a-zA-Z0-9\-\.\:]*
    ///
    /// Update requires: Replacement
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub backup_id: Option<cfn_resources::StrVal>,

    ///
    /// The number of automated backups that you want to keep. Whenever a new backup is created, AWS OpsWorks CM deletes the oldest backups if this number is exceeded.     The default value is 1.
    ///
    /// Required: No
    ///
    /// Type: Integer
    ///
    /// Minimum: 1
    ///
    /// Update requires: No interruption
    #[serde(rename = "BackupRetentionCount")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub backup_retention_count: Option<i64>,

    ///
    /// Supported on servers running Chef Automate 2.0 only. A PEM-formatted HTTPS       certificate. The value can be be a single, self-signed certificate, or a certificate       chain. If you specify a custom certificate, you must also specify values for         CustomDomain and CustomPrivateKey. The following are       requirements for the CustomCertificate value:
    ///
    /// You can provide either a self-signed, custom certificate, or the full           certificate chain.               The certificate must be a valid X509 certificate, or a certificate chain in           PEM format.               The certificate must be valid at the time of upload. A certificate can't be           used before its validity period begins (the certificate's NotBefore           date), or after it expires (the certificate's NotAfter           date).               The certificate’s common name or subject alternative names (SANs), if present,           must match the value of CustomDomain.               The certificate must match the value of CustomPrivateKey.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 2097152
    ///
    /// Pattern: (?s)\s*-----BEGIN CERTIFICATE-----.+-----END CERTIFICATE-----\s*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomCertificate")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub custom_certificate: Option<cfn_resources::StrVal>,

    ///
    /// Supported on servers running Chef Automate 2.0 only. An optional public endpoint of a       server, such as https://aws.my-company.com. To access the server, create a       CNAME DNS record in your preferred DNS service that points the custom domain to the       endpoint that is generated when the server is created (the value of the CreateServer       Endpoint attribute). You cannot access the server by using the generated         Endpoint value if the server is using a custom domain. If you specify a       custom domain, you must also specify values for CustomCertificate and         CustomPrivateKey.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 253
    ///
    /// Pattern: ^(((?!-)[A-Za-z0-9-]{0,62}[A-Za-z0-9])\.)+((?!-)[A-Za-z0-9-]{1,62}[A-Za-z0-9])$
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomDomain")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub custom_domain: Option<cfn_resources::StrVal>,

    ///
    /// Supported on servers running Chef Automate 2.0 only. A private key in PEM format for       connecting to the server by using HTTPS. The private key must not be encrypted; it       cannot be protected by a password or passphrase. If you specify a custom private key,       you must also specify values for CustomDomain and         CustomCertificate.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 4096
    ///
    /// Pattern: (?ms)\s*^-----BEGIN (?-s:.*)PRIVATE KEY-----$.*?^-----END (?-s:.*)PRIVATE KEY-----$\s*
    ///
    /// Update requires: Replacement
    #[serde(rename = "CustomPrivateKey")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub custom_private_key: Option<cfn_resources::StrVal>,

    ///
    /// Enable or disable scheduled backups. Valid values are true or false. The default value is true.
    ///
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "DisableAutomatedBackup")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub disable_automated_backup: Option<bool>,

    ///
    /// The configuration management engine to use. Valid values include ChefAutomate and Puppet.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Pattern: (?s).*
    ///
    /// Update requires: Replacement
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub engine: Option<cfn_resources::StrVal>,

    ///
    /// Optional engine attributes on a specified server.
    ///
    /// Attributes accepted in a Chef createServer request:                                     CHEF_AUTOMATE_PIVOTAL_KEY: A          base64-encoded RSA public key. The corresponding private key is required to          access the Chef API. When no CHEF_AUTOMATE_PIVOTAL_KEY is set, a private key is          generated and returned in the response. When you are specifying the value of CHEF_AUTOMATE_PIVOTAL_KEY as a parameter in the AWS CloudFormation console,           you must add newline (\n) characters at the end of each line of the pivotal key value.                           CHEF_AUTOMATE_ADMIN_PASSWORD:           The password for the administrative user in the Chef Automate web-based dashboard. The           password length is a minimum of eight characters, and a maximum of 32. The           password can contain letters, numbers, and special characters           (!/@#$%^&+=_). The password must contain at least one lower case letter, one upper           case letter, one number, and one special character. When no CHEF_AUTOMATE_ADMIN_PASSWORD is set, one is           generated and returned in the response.
    ///
    /// Attributes accepted in a Puppet createServer request:                                            PUPPET_ADMIN_PASSWORD: To work with the Puppet Enterprise console, a password must use ASCII characters.                        PUPPET_R10K_REMOTE: The r10k remote is the URL of your control repository     (for example, ssh://git@your.git-repo.com:user/control-repo.git). Specifying an r10k remote opens TCP port 8170.                        PUPPET_R10K_PRIVATE_KEY: If you are using a private Git repository, add     PUPPET_R10K_PRIVATE_KEY to specify a PEM-encoded private SSH key.
    ///
    /// Required: No
    ///
    /// Type: List of EngineAttribute
    ///
    /// Update requires: No interruption
    #[serde(rename = "EngineAttributes")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub engine_attributes: Option<Vec<EngineAttribute>>,

    ///
    /// The engine model of the server. Valid values in this release include Monolithic for Puppet and Single for Chef.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Pattern: (?s).*
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineModel")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub engine_model: Option<cfn_resources::StrVal>,

    ///
    /// The major release version of the engine that you want to use. For a Chef server, the valid value for EngineVersion     is currently 2. For a Puppet server, valid values are 2019 or 2017.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Pattern: (?s).*
    ///
    /// Update requires: Replacement
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub engine_version: Option<cfn_resources::StrVal>,

    ///
    /// The ARN of the instance profile that your Amazon EC2 instances use.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Pattern: arn:aws:iam::[0-9]{12}:instance-profile/.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceProfileArn")]
    pub instance_profile_arn: cfn_resources::StrVal,

    ///
    /// The Amazon EC2 instance type to use. For example, m5.large.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Pattern: (?s).*
    ///
    /// Update requires: Replacement
    #[serde(rename = "InstanceType")]
    pub instance_type: cfn_resources::StrVal,

    ///
    /// The Amazon EC2 key pair to set for the instance. This parameter is optional; if desired, you may specify this parameter to connect to your instances by using SSH.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Pattern: .*
    ///
    /// Update requires: Replacement
    #[serde(rename = "KeyPair")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub key_pair: Option<cfn_resources::StrVal>,

    ///
    /// The start time for a one-hour period during which AWS OpsWorks CM backs up application-level data on your server    if automated backups are enabled. Valid values must be specified in one of the following formats:
    ///
    /// HH:MM for daily backups                        DDD:HH:MM for weekly backups
    ///
    /// MM must be specified as 00. The specified time is in coordinated universal time (UTC). The default value is a random, daily start time.
    ///
    /// Example:       08:00, which represents a daily start time of 08:00 UTC.
    ///
    /// Example:       Mon:08:00, which represents a start time of every Monday at 08:00 UTC. (8:00 a.m.)
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub preferred_backup_window: Option<cfn_resources::StrVal>,

    ///
    /// The start time for a one-hour period each week during which AWS OpsWorks CM performs maintenance on the instance.    Valid values must be specified in the following format: DDD:HH:MM. MM must be specified as 00. The specified time is in coordinated universal time (UTC).    The default value is a random one-hour period on Tuesday, Wednesday, or Friday. See TimeWindowDefinition for more information.
    ///
    /// Example:       Mon:08:00,    which represents a start time of every Monday at 08:00 UTC. (8:00 a.m.)
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub preferred_maintenance_window: Option<cfn_resources::StrVal>,

    ///
    /// A list of security group IDs to attach to the Amazon EC2 instance. If you add this parameter, the specified security groups    must be within the VPC that is specified by SubnetIds.
    ///
    /// If you do not specify this parameter, AWS OpsWorks CM creates one new security group that uses TCP ports 22 and 443, open to    0.0.0.0/0 (everyone).
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub security_group_ids: Option<Vec<String>>,

    ///
    /// The service role that the AWS OpsWorks CM service backend uses to work with your account. Although the AWS OpsWorks    management console typically creates    the service role for you, if you are using the AWS CLI or API commands,    run the service-role-creation.yaml AWS CloudFormation template, located at https://s3.amazonaws.com/opsworks-cm-us-east-1-prod-default-assets/misc/opsworks-cm-roles.yaml.    This template creates a CloudFormation stack that includes the service role and instance profile that you need.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Pattern: arn:aws:iam::[0-9]{12}:role/.*
    ///
    /// Update requires: Replacement
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: cfn_resources::StrVal,

    ///
    /// The IDs of subnets in which to launch the server EC2 instance.
    ///
    /// Amazon EC2-Classic customers: This field is required. All servers must run within a VPC. The VPC must have "Auto Assign Public IP" enabled.
    ///
    /// EC2-VPC customers: This field is optional. If you do not specify subnet IDs, your EC2 instances are created in a default subnet that    is selected by Amazon EC2. If you specify subnet IDs, the VPC must have "Auto Assign Public IP" enabled.
    ///
    /// For more information about supported Amazon EC2 platforms, see    Supported Platforms.
    ///
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub subnet_ids: Option<Vec<String>>,

    ///
    /// A map that contains tag keys and tag values to attach to an AWS OpsWorks for Chef Automate or OpsWorks for Puppet Enterprise server.
    ///
    /// The key cannot be empty.               The key can be a maximum of 127 characters, and can contain only Unicode letters, numbers, or separators, or the following special characters: + - = . _ : / @                       The value can be a maximum 255 characters, and contain only Unicode letters, numbers, or separators, or the following special characters: + - = . _ : / @                       Leading and trailing spaces are trimmed from both the key and value.               A maximum of 50 user-applied tags is allowed for any AWS OpsWorks CM server.
    ///
    /// Required: No
    ///
    /// Type: List of Tag
    ///
    /// Maximum: 200
    ///
    /// Update requires: No interruption
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub tags: Option<Vec<Tag>>,

    #[serde(skip_serializing)]
    pub att_arn: CfnServerarn,

    #[serde(skip_serializing)]
    pub att_endpoint: CfnServerendpoint,

    #[serde(skip_serializing)]
    pub att_server_name: CfnServerservername,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServerarn;
impl CfnServerarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServerendpoint;
impl CfnServerendpoint {
    pub fn att_name(&self) -> &'static str {
        r#"Endpoint"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnServerservername;
impl CfnServerservername {
    pub fn att_name(&self) -> &'static str {
        r#"ServerName"#
    }
}

impl cfn_resources::CfnResource for CfnServer {
    fn type_string(&self) -> &'static str {
        "AWS::OpsWorksCM::Server"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.backup_id {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 79 as _ {
                    return Err(format!(
                        "Max validation failed on field 'backup_id'. {} is greater than 79",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.backup_retention_count {
            if *the_val < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'backup_retention_count'. {} is less than 1",
                    the_val
                ));
            }
        }

        if let Some(the_val) = &self.custom_certificate {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2097152 as _ {
                    return Err(format!("Max validation failed on field 'custom_certificate'. {} is greater than 2097152", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.custom_domain {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 253 as _ {
                    return Err(format!(
                        "Max validation failed on field 'custom_domain'. {} is greater than 253",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.custom_private_key {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 4096 as _ {
                    return Err(format!("Max validation failed on field 'custom_private_key'. {} is greater than 4096", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.engine {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'engine'. {} is greater than 10000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.engine_model {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'engine_model'. {} is greater than 10000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.engine_version {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'engine_version'. {} is greater than 10000",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.instance_profile_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 10000 as _ {
                return Err(format!("Max validation failed on field 'instance_profile_arn'. {} is greater than 10000", s.len()));
            }
        }

        let the_val = &self.instance_type;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 10000 as _ {
                return Err(format!(
                    "Max validation failed on field 'instance_type'. {} is greater than 10000",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.key_pair {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'key_pair'. {} is greater than 10000",
                        s.len()
                    ));
                }
            }
        }

        let the_val = &self.service_role_arn;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 10000 as _ {
                return Err(format!(
                    "Max validation failed on field 'service_role_arn'. {} is greater than 10000",
                    s.len()
                ));
            }
        }

        if let Some(the_val) = &self.tags {
            if the_val.len() > 200 as _ {
                return Err(format!(
                    "Max validation failed on field 'tags'. {} is greater than 200",
                    the_val.len()
                ));
            }
        }

        Ok(())
    }
}

/// The EngineAttribute property type specifies administrator credentials for      an AWS OpsWorks for Chef Automate or OpsWorks for Puppet Enterprise server.     EngineAttribute is a property of the AWS::OpsWorksCM::Server    resource type.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EngineAttribute {
    ///
    /// The name of the engine attribute.
    ///
    /// Attribute name for Chef Automate servers:
    ///
    /// CHEF_AUTOMATE_ADMIN_PASSWORD
    ///
    /// Attribute names for Puppet Enterprise servers:
    ///
    /// PUPPET_ADMIN_PASSWORD                          PUPPET_R10K_REMOTE                          PUPPET_R10K_PRIVATE_KEY
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Pattern: (?s).*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// The value of the engine attribute.
    ///
    /// Attribute value for Chef Automate servers:
    ///
    /// CHEF_AUTOMATE_PIVOTAL_KEY: A base64-encoded RSA public key. The corresponding      private key is required to access the Chef API. You can generate this key by running the      following OpenSSL command on Linux-based      computers.               openssl genrsa -out pivotal_key_file_name.pem      2048             On Windows-based computers, you can use the PuTTYgen utility to generate a      base64-encoded RSA private key. For more information, see PuTTYgen - Key Generator for PuTTY       on Windows on SSH.com.
    ///
    /// Attribute values for Puppet Enterprise servers:
    ///
    /// PUPPET_ADMIN_PASSWORD: An administrator password that you can use to      sign in to the Puppet Enterprise console webpage after the server is online. The password      must use between 8 and 32 ASCII characters.                    PUPPET_R10K_REMOTE: The r10k remote is the URL of your control      repository (for example, ssh://git@your.git-repo.com:user/control-repo.git). Specifying an      r10k remote opens TCP port 8170.                    PUPPET_R10K_PRIVATE_KEY: If you are using a private Git repository, add       PUPPET_R10K_PRIVATE_KEY to specify a PEM-encoded private SSH key.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Maximum: 10000
    ///
    /// Pattern: (?s).*
    ///
    /// Update requires: No interruption
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "cfn_resources::wants_serialization")]
    pub value: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for EngineAttribute {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 10000",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.value {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 10000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'value'. {} is greater than 10000",
                        s.len()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// You can use the Resource Tags property to apply tags to resources, which can help you    identify and categorize those resources. You can tag only resources for which AWS CloudFormation supports    tagging. For information about which resources you can tag with CloudFormation, see the individual    resources in AWS resource and property types reference.
///
/// In addition to any tags you define, CloudFormation automatically creates the following    stack-level tags with the prefix aws::
///
/// The aws: prefix is reserved for AWS use. This prefix is case-insensitive. If    you use this prefix in the Key or Value property, you can't update    or delete the tag. Tags with this prefix don't count toward the number of tags per    resource.
///
/// Propagation of stack-level tags to resources, including automatically created tags, can vary by resource. For example, tags aren't propagated to Amazon EBS volumes that are created from block device mappings.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Tag {
    ///
    /// The key name of the tag. You can specify a value that's 1 to 128 Unicode          characters in length and can't be prefixed with aws:. You can use any          of the following characters: the set of Unicode letters, digits, whitespace,           _, ., /, =, +,          and -.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Key")]
    pub key: cfn_resources::StrVal,

    ///
    /// The value for the tag. You can specify a value that's 1 to 256 characters in          length.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    #[serde(rename = "Value")]
    pub value: cfn_resources::StrVal,
}

impl cfn_resources::CfnResource for Tag {
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
