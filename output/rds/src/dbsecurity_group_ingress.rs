

/// The AWS::RDS::DBSecurityGroupIngress resource enables ingress to a DB       security group using one of two forms of authorization. First, you can add EC2 or VPC       security groups to the DB security group if the application using the database is       running on EC2 or VPC instances. Second, IP ranges are available if the application       accessing your database is running on the Internet.
///
/// This type supports updates. For more information about updating stacks, see AWS         CloudFormation Stacks Updates.
///
/// For details about the settings for DB security group ingress, see AuthorizeDBSecurityGroupIngress.
#[derive(Default, serde::Serialize)]
pub struct CfnDBSecurityGroupIngress {


    /// 
    /// AWS account number of the owner of the EC2 security group     specified in the EC2SecurityGroupName parameter.     The AWS access key ID isn't an acceptable value.     For VPC DB security groups, EC2SecurityGroupId must be provided.     Otherwise, EC2SecurityGroupOwnerId and either EC2SecurityGroupName or EC2SecurityGroupId must be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: Option<String>,


    /// 
    /// The name of the DB security group to add authorization to.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DBSecurityGroupName")]
    pub dbsecurity_group_name: String,


    /// 
    /// Name of the EC2 security group to authorize.     For VPC DB security groups, EC2SecurityGroupId must be provided.     Otherwise, EC2SecurityGroupOwnerId and either EC2SecurityGroupName    or EC2SecurityGroupId must be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EC2SecurityGroupName")]
    pub ec2_security_group_name: Option<String>,


    /// 
    /// Id of the EC2 security group to authorize.     For VPC DB security groups, EC2SecurityGroupId must be provided.     Otherwise, EC2SecurityGroupOwnerId and either EC2SecurityGroupName or EC2SecurityGroupId must be provided.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "EC2SecurityGroupId")]
    pub ec2_security_group_id: Option<String>,


    /// 
    /// The IP range to authorize.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CIDRIP")]
    pub cidrip: Option<String>,

}
