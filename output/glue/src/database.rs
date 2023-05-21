

/// The AWS::Glue::Database resource specifies a logical grouping of tables       in AWS Glue. For more information, see Defining a Database in Your Data         Catalog and Database Structure in the AWS Glue Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CfnDatabase {


    /// 
    /// The AWS account ID for the account in which to create the catalog object.
    /// 
    /// Note To specify the account ID, you can use the Ref intrinsic function         with the AWS::AccountId pseudo parameter. For example: !Ref           AWS::AccountId
    /// 
    /// Required: Yes
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
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,


    /// 
    /// The metadata for the database.
    /// 
    /// Required: Yes
    ///
    /// Type: DatabaseInput
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseInput")]
    pub database_input: DatabaseInput,

}



impl cfn_resources::CfnResource for CfnDatabase {
    fn type_string() -> &'static str {
        "AWS::Glue::Database"
    }

    fn properties(self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize cloudformation resource properties")
    }
}


/// The AWS Lake Formation principal.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DataLakePrincipal {


    /// 
    /// An identifier for the AWS Lake Formation principal.
    /// 
    /// Required: Conditional
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 255
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLakePrincipalIdentifier")]
    pub data_lake_principal_identifier: Option<String>,

}




/// A structure that describes a target database for resource linking.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatabaseIdentifier {


    /// 
    /// The ID of the Data Catalog in which the database resides.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,


    /// 
    /// The name of the catalog database.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,

}




/// The structure used to create or update a database.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct DatabaseInput {


    /// 
    /// Creates a set of default permissions on the table for principals. Used by AWS Lake Formation. Not used in the normal course of AWS Glue operations.
    /// 
    /// Required: No
    ///
    /// Type: List of PrincipalPrivileges
    ///
    /// Update requires: No interruption
    #[serde(rename = "CreateTableDefaultPermissions")]
    pub create_table_default_permissions: Option<Vec<PrincipalPrivileges>>,


    /// 
    /// A description of the database.
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


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FederatedDatabase
    ///
    /// Update requires: No interruption
    #[serde(rename = "FederatedDatabase")]
    pub federated_database: Option<FederatedDatabase>,


    /// 
    /// The location of the database (for example, an HDFS path).
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 1024
    ///
    /// Pattern: [\u0020-\uD7FF\uE000-\uFFFD\uD800\uDC00-\uDBFF\uDFFF\r\n\t]*
    ///
    /// Update requires: No interruption
    #[serde(rename = "LocationUri")]
    pub location_uri: Option<String>,


    /// 
    /// The name of the database. For Hive compatibility, this is folded to lowercase when it is    stored.
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
    /// These key-value pairs define parameters and properties    of the database.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,


    /// 
    /// A DatabaseIdentifier structure that describes a target database for resource linking.
    /// 
    /// Required: No
    ///
    /// Type: DatabaseIdentifier
    ///
    /// Update requires: No interruption
    #[serde(rename = "TargetDatabase")]
    pub target_database: Option<DatabaseIdentifier>,

}




/// The FederatedDatabase property type specifies Property description not available. for an AWS::Glue::Database.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct FederatedDatabase {


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,


    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Identifier")]
    pub identifier: Option<String>,

}




/// the permissions granted to a principal
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PrincipalPrivileges {


    /// 
    /// The permissions that are granted to the principal.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<String>>,


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


