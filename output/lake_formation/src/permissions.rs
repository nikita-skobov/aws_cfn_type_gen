

/// The AWS::LakeFormation::Permissions resource represents the permissions that a principal has on an AWS Glue Data Catalog resource (such as AWS Glue database or AWS Glue tables). When you upload a permissions stack, the permissions are granted to the principal and when you remove the stack, the permissions are revoked from the principal. If you remove a stack, and the principal does not have the permissions referenced in the stack then AWS Lake Formation will throw an error because you can’t call revoke on non-existing permissions. To successfully remove the stack, you’ll need to regrant those permissions and then remove the stack.
#[derive(Default, serde::Serialize)]
pub struct CfnPermissions {


    /// 
    /// Indicates the ability to grant permissions (as a subset of permissions granted).
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "PermissionsWithGrantOption")]
    pub permissions_with_grant_option: Option<Vec<String>>,


    /// 
    /// A structure for the resource.
    /// 
    /// Required: Yes
    ///
    /// Type: Resource
    ///
    /// Update requires: No interruption
    #[serde(rename = "Resource")]
    pub resource: Resource,


    /// 
    /// The AWS Lake Formation principal.
    /// 
    /// Required: Yes
    ///
    /// Type: DataLakePrincipal
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLakePrincipal")]
    pub data_lake_principal: DataLakePrincipal,


    /// 
    /// The permissions granted or revoked.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<String>>,

}


/// A structure for the database object.
#[derive(Default, serde::Serialize)]
pub struct DatabaseResource {


    /// 
    /// The name of the database resource. Unique to the Data Catalog.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,


    /// 
    /// The identifier for the Data Catalog. By default, it is the account ID of the caller.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,

}


/// A structure for a data location object where permissions are granted or revoked.
#[derive(Default, serde::Serialize)]
pub struct DataLocationResource {


    /// 
    /// The identifier for the Data Catalog. By default, it is the account ID of the caller.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,


    /// 
    /// The Amazon Resource Name (ARN) that uniquely identifies the data location resource.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "S3Resource")]
    pub s3_resource: Option<String>,

}


/// A wildcard object, consisting of an optional list of excluded column names or indexes.
#[derive(Default, serde::Serialize)]
pub struct ColumnWildcard {


    /// 
    /// Excludes column names. Any column with this name will be excluded.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ExcludedColumnNames")]
    pub excluded_column_names: Option<Vec<String>>,

}


/// A structure for the resource.
#[derive(Default, serde::Serialize)]
pub struct Resource {


    /// 
    /// A structure for the table object. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal.
    /// 
    /// Required: No
    ///
    /// Type: TableResource
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableResource")]
    pub table_resource: Option<TableResource>,


    /// 
    /// A structure for a table with columns object. This object is only used when granting a SELECT permission.
    /// 
    /// Required: No
    ///
    /// Type: TableWithColumnsResource
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableWithColumnsResource")]
    pub table_with_columns_resource: Option<TableWithColumnsResource>,


    /// 
    /// A structure for the database object.
    /// 
    /// Required: No
    ///
    /// Type: DatabaseResource
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseResource")]
    pub database_resource: Option<DatabaseResource>,


    /// 
    /// A structure for a data location object where permissions are granted or revoked.
    /// 
    /// Required: No
    ///
    /// Type: DataLocationResource
    ///
    /// Update requires: No interruption
    #[serde(rename = "DataLocationResource")]
    pub data_location_resource: Option<DataLocationResource>,

}


/// A wildcard object representing every table under a database.
#[derive(Default, serde::Serialize)]
pub struct TableWildcard {

}


/// A structure for a table with columns object. This object is only used when granting a SELECT permission.
///
/// This object must take a value for at least one of ColumnsNames, ColumnsIndexes, or ColumnsWildcard.
#[derive(Default, serde::Serialize)]
pub struct TableWithColumnsResource {


    /// 
    /// The name of the database for the table with columns resource. Unique to the Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,


    /// 
    /// The identifier for the Data Catalog. By default, it is the account ID of the caller.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,


    /// 
    /// A wildcard specified by a ColumnWildcard object. At least one of ColumnNames or ColumnWildcard is required.
    /// 
    /// Required: No
    ///
    /// Type: ColumnWildcard
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnWildcard")]
    pub column_wildcard: Option<ColumnWildcard>,


    /// 
    /// The list of column names for the table. At least one of ColumnNames or ColumnWildcard is required.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ColumnNames")]
    pub column_names: Option<Vec<String>>,


    /// 
    /// The name of the table resource. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// A structure for the table object. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal.
#[derive(Default, serde::Serialize)]
pub struct TableResource {


    /// 
    /// An empty object representing all tables under a database. If this field is specified instead of the Name field, all tables under DatabaseName will have permission changes applied.
    /// 
    /// Required: No
    ///
    /// Type: TableWildcard
    ///
    /// Update requires: No interruption
    #[serde(rename = "TableWildcard")]
    pub table_wildcard: Option<TableWildcard>,


    /// 
    /// The identifier for the Data Catalog. By default, it is the account ID of the caller.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,


    /// 
    /// The name of the database for the table. Unique to a Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,


    /// 
    /// The name of the table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// The Lake Formation principal.
#[derive(Default, serde::Serialize)]
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
