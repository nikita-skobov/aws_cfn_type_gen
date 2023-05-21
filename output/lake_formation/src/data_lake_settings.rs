

/// The AWS::LakeFormation::DataLakeSettings resource is an AWS Lake Formation resource type that manages the data lake settings for your account.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDataLakeSettings {


    /// 
    /// Whether to allow Amazon EMR clusters or other third-party query engines to access data managed by Lake Formation.
    /// 
    /// If set to true, you allow Amazon EMR clusters or other third-party engines to access data in Amazon S3 locations that are registered with Lake Formation.
    /// 
    /// If false or null, no third-party query engines will be able to access data in Amazon S3 locations that are registered with Lake Formation.
    /// 
    /// For more information, see External data filtering setting.
    /// 
    /// Required: No
    ///
    /// Type: Boolean
    ///
    /// Update requires: No interruption
    #[serde(rename = "AllowExternalDataFiltering")]
    pub allow_external_data_filtering: Option<bool>,


    /// 
    /// A list of the account IDs of AWS accounts with Amazon EMR clusters or third-party engines that are allwed to perform data filtering.
    /// 
    /// Required: No
    ///
    /// Type: ExternalDataFilteringAllowList
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExternalDataFilteringAllowList")]
    pub external_data_filtering_allow_list: Option<ExternalDataFilteringAllowList>,


    /// 
    /// A list of AWS Lake Formation principals.
    /// 
    /// Required: No
    ///
    /// Type: Admins
    ///
    /// Update requires: No interruption
    #[serde(rename = "Admins")]
    pub admins: Option<Admins>,


    /// 
    /// Lake Formation relies on a privileged process secured by Amazon EMR or the third party integrator to tag the user's role while assuming it. Lake Formation will publish the acceptable key-value pair, for example key = "LakeFormationTrustedCaller" and value = "TRUE" and the third party integrator must properly tag the temporary security credentials that will be used to call Lake Formation's administrative API operations.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "AuthorizedSessionTagValueList")]
    pub authorized_session_tag_value_list: Option<Vec<String>>,


    /// 
    /// A key-value map that provides an additional configuration on your data lake. CrossAccountVersion is the key you can configure in the Parameters field. Accepted values for the CrossAccountVersion key are 1, 2, and 3.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,


    /// 
    /// An array of UTF-8 strings.
    /// 
    /// A list of the resource-owning account IDs that the caller's account can use to share their user access details (user ARNs). The user ARNs can be logged in the resource owner's CloudTrail log. 	     	    You may want to specify this property when you are in a high-trust boundary, such as the same team or company.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "TrustedResourceOwners")]
    pub trusted_resource_owners: Option<Vec<String>>,


    /// 
    /// Specifies whether access control on a newly created database is managed by Lake Formation permissions or exclusively by IAM permissions.
    /// 
    /// A null value indicates that the access is controlled by Lake Formation permissions.       ALL permissions assigned to IAM_ALLOWED_PRINCIPALS group     indicates that the user's IAM permissions determine the access to the     database. This is referred to as the setting "Use only IAM access control," and is to     support backward compatibility with the AWS Glue permission model implemented by       IAM permissions.
    /// 
    /// The only permitted values are an empty array or an array that contains a single JSON object that grants ALL to IAM_ALLOWED_PRINCIPALS.
    /// 
    /// For more information, see Changing the default security settings for your data lake.
    /// 
    /// Required: No
    ///
    /// Type: CreateDatabaseDefaultPermissions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreateDatabaseDefaultPermissions")]
    pub create_database_default_permissions: Option<CreateDatabaseDefaultPermissions>,


    /// 
    /// Specifies whether access control on a newly created table is managed by Lake Formation permissions or exclusively by IAM permissions.
    /// 
    /// A null value indicates that the access is controlled by Lake Formation permissions.       ALL permissions assigned to IAM_ALLOWED_PRINCIPALS group     indicate that the user's IAM permissions determine the access to the     table. This is referred to as the setting "Use only IAM access control," and is to support     the backward compatibility with the AWS Glue permission model implemented by IAM     permissions.
    /// 
    /// The only permitted values are an empty array or an array that contains a single JSON object that grants ALL permissions to IAM_ALLOWED_PRINCIPALS.
    /// 
    /// For more information, see Changing the default security settings for your data lake.
    /// 
    /// Required: No
    ///
    /// Type: CreateTableDefaultPermissions
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreateTableDefaultPermissions")]
    pub create_table_default_permissions: Option<CreateTableDefaultPermissions>,

}

impl cfn_resources::CfnResource for CfnDataLakeSettings {
    fn type_string() -> &'static str {
        "AWS::LakeFormation::DataLakeSettings"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// A list of the account IDs of AWS accounts with Amazon EMR     clusters that are allowed to perform data filtering.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ExternalDataFilteringAllowList {

}


/// A list of AWS Lake Formation principals.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Admins {

}


/// Specifies whether access control on a newly created database is managed by Lake Formation permissions or exclusively by IAM permissions.
///
/// A null value indicates that the access is controlled by Lake Formation permissions. A value that assigns ALL to IAM_ALLOWED_PRINCIPALS indicates access control by IAM permissions. This is referred to as the setting "Use only IAM access control," and is for backward compatibility with the AWS Glue permission model implemented by IAM permissions.
///
/// The only permitted values are an empty array or an array that contains a single JSON object that grants ALL to IAM_ALLOWED_PRINCIPALS.
///
/// For more information, see Changing the default security settings for your data lake.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreateDatabaseDefaultPermissions {

}


/// Permissions granted to a principal.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Permissions {

}


/// The Lake Formation principal.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataLakePrincipal {


    /// 
    /// An identifier for the Lake Formation principal.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLakePrincipalIdentifier")]
    pub data_lake_principal_identifier: Option<String>,

}


/// Permissions granted to a principal.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrincipalPermissions {


    /// 
    /// The permissions that are granted to the principal.
    /// 
    /// Required: No
    ///
    /// Type: Permissions
    ///
    /// Update requires: No interruption
    #[serde(rename = "Permissions")]
    pub permissions: Option<Permissions>,


    /// 
    /// The principal who is granted permissions.
    /// 
    /// Required: No
    ///
    /// Type: DataLakePrincipal
    ///
    /// Update requires: No interruption
    #[serde(rename = "Principal")]
    pub principal: Option<DataLakePrincipal>,

}


/// Specifies whether access control on a newly created table is managed by Lake Formation permissions or exclusively by IAM permissions.
///
/// A null value indicates that the access is controlled by Lake Formation permissions. A value that assigns ALL to IAM_ALLOWED_PRINCIPALS indicates access control by IAM permissions.       This is referred to as the setting "Use only IAM access control," and is for backward compatibility with the AWS Glue permission model implemented by IAM permissions.
///
/// The only permitted values are an empty array or an array that contains a single JSON object that grants ALL to IAM_ALLOWED_PRINCIPALS.
///
/// For more information, see Changing the Default Security Settings for Your Data Lake.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreateTableDefaultPermissions {

}
