

/// The AWS::ElastiCache::SecurityGroupIngress type authorizes ingress to a cache security group from hosts in specified Amazon EC2 security groups. For more information about ElastiCache security group ingress,     go to AuthorizeCacheSecurityGroupIngress in the Amazon ElastiCache API Reference Guide.
#[derive(Default, serde::Serialize)]
pub struct CfnSecurityGroupIngress {


    /// 
    /// Name of the EC2 Security Group to include in the authorization.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EC2SecurityGroupName")]
    pub ec2_security_group_name: String,


    /// 
    /// The name of the Cache Security Group to authorize.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CacheSecurityGroupName")]
    pub cache_security_group_name: String,


    /// 
    /// Specifies the Amazon Account ID of the owner of the EC2 security group specified in the EC2SecurityGroupName property. The Amazon access key ID is not an acceptable value.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: Option<String>,

}