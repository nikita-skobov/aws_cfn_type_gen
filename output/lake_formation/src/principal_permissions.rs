

/// The AWS::LakeFormation::PrincipalPermissions resource represents the permissions that a principal has on a Data Catalog resource (such as AWS Glue databases or AWS Glue tables).     When you create a PrincipalPermissions resource, the permissions are granted via the AWS Lake Formation GrantPermissions API operation. When you delete a PrincipalPermissions resource, the permissions on principal-resource pair are revoked via the AWS Lake Formation RevokePermissions API operation.
#[derive(Default, serde::Serialize)]
pub struct CfnPrincipalPermissions {


    /// 
    /// The principal to be granted a permission.
    /// 
    /// Required: Yes
    ///
    /// Type: DataLakePrincipal
    ///
    /// Update requires: Replacement
    #[serde(rename = "Principal")]
    pub principal: DataLakePrincipal,


    /// 
    /// Indicates the ability to grant permissions (as a subset of permissions granted).
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PermissionsWithGrantOption")]
    pub permissions_with_grant_option: Vec<String>,


    /// 
    /// The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store.     It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Catalog")]
    pub catalog: Option<String>,


    /// 
    /// The permissions granted or revoked.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Permissions")]
    pub permissions: Vec<String>,


    /// 
    /// The resource to be granted or revoked permissions.
    /// 
    /// Required: Yes
    ///
    /// Type: Resource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Resource")]
    pub resource: Resource,

}


/// The AWS Lake Formation principal.
#[derive(Default, serde::Serialize)]
pub struct DataLakePrincipal {


    /// 
    /// An identifier for the AWS Lake Formation principal.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataLakePrincipalIdentifier")]
    pub data_lake_principal_identifier: Option<String>,

}


/// A structure for a data location object where permissions are granted or revoked.
#[derive(Default, serde::Serialize)]
pub struct DataLocationResource {


    /// 
    /// The identifier for the Data Catalog where the location is registered with AWS Lake Formation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,


    /// 
    /// The Amazon Resource Name (ARN) that uniquely identifies the data location resource.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,

}


/// A structure for the table object. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal.
#[derive(Default, serde::Serialize)]
pub struct TableResource {


    /// 
    /// A wildcard object representing every table under a database.
    /// 
    /// At least one of TableResource$Name or TableResource$TableWildcard is required.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableWildcard")]
    pub table_wildcard: Option<serde_json::Value>,


    /// 
    /// The name of the database for the table. Unique to a Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// Property description not available.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,


    /// 
    /// The name of the table.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: Option<String>,

}


/// A structure that describes certain columns on certain rows.
#[derive(Default, serde::Serialize)]
pub struct DataCellsFilterResource {


    /// 
    /// The name of the table.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableName")]
    pub table_name: String,


    /// 
    /// The ID of the catalog to which the table belongs.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableCatalogId")]
    pub table_catalog_id: String,


    /// 
    /// A database in the Data Catalog.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// 
    /// The name given by the user to the data filter cell.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}


/// A structure for a table with columns object. This object is only used when granting a SELECT permission.
///
/// This object must take a value for at least one of ColumnsNames, ColumnsIndexes, or ColumnsWildcard.
#[derive(Default, serde::Serialize)]
pub struct TableWithColumnsResource {


    /// 
    /// A wildcard specified by a ColumnWildcard object. At least one of ColumnNames or ColumnWildcard is required.
    /// 
    /// Required: No
    ///
    /// Type: ColumnWildcard
    ///
    /// Update requires: Replacement
    #[serde(rename = "ColumnWildcard")]
    pub column_wildcard: Option<ColumnWildcard>,


    /// 
    /// The name of the table resource. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,


    /// 
    /// The list of column names for the table. At least one of ColumnNames or ColumnWildcard is required.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ColumnNames")]
    pub column_names: Option<Vec<String>>,


    /// 
    /// The name of the database for the table with columns resource. Unique to the Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database privileges to a principal.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DatabaseName")]
    pub database_name: String,


    /// 
    /// The identifier for the Data Catalog where the location is registered with AWS Lake Formation.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,

}


/// A list of LF-tag conditions that define a resource's LF-tag policy.
///
/// A structure that allows an admin to grant user permissions on certain conditions. For example, granting a role access to all columns that do not have the LF-tag 'PII' in tables that have the LF-tag 'Prod'.
#[derive(Default, serde::Serialize)]
pub struct LFTagPolicyResource {


    /// 
    /// A list of LF-tag conditions that apply to the resource's LF-tag policy.
    /// 
    /// Required: Yes
    ///
    /// Type: List of LFTag
    ///
    /// Update requires: Replacement
    #[serde(rename = "Expression")]
    pub expression: Vec<LFTag>,


    /// 
    /// The identifier for the Data Catalog. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,


    /// 
    /// The resource type for which the LF-tag policy applies.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ResourceType")]
    pub resource_type: String,

}


/// A structure containing an LF-tag key and values for a resource.
#[derive(Default, serde::Serialize)]
pub struct LFTagKeyResource {


    /// 
    /// A list of possible values for the corresponding TagKey of an LF-tag key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagValues")]
    pub tag_values: Vec<String>,


    /// 
    /// The identifier for the Data Catalog where the location is registered with Data Catalog.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,


    /// 
    /// The key-name for the LF-tag.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagKey")]
    pub tag_key: String,

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
    /// Update requires: Replacement
    #[serde(rename = "ExcludedColumnNames")]
    pub excluded_column_names: Option<Vec<String>>,

}


/// A structure for the resource.
#[derive(Default, serde::Serialize)]
pub struct Resource {


    /// 
    /// The location of an Amazon S3 path where permissions are granted or revoked.
    /// 
    /// Required: No
    ///
    /// Type: DataLocationResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataLocation")]
    pub data_location: Option<DataLocationResource>,


    /// 
    /// The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your AWS Lake Formation environment.
    /// 
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: Replacement
    #[serde(rename = "Catalog")]
    pub catalog: Option<serde_json::Value>,


    /// 
    /// A list of LF-tag conditions that define a resource's LF-tag policy.
    /// 
    /// Required: No
    ///
    /// Type: LFTagPolicyResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "LFTagPolicy")]
    pub lftag_policy: Option<LFTagPolicyResource>,


    /// 
    /// The LF-tag key and values attached to a resource.
    /// 
    /// Required: No
    ///
    /// Type: LFTagKeyResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "LFTag")]
    pub lftag: Option<LFTagKeyResource>,


    /// 
    /// The table with columns for the resource. A principal with permissions to this resource can select metadata from the columns of a table in the Data Catalog and the underlying data in Amazon S3.
    /// 
    /// Required: No
    ///
    /// Type: TableWithColumnsResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "TableWithColumns")]
    pub table_with_columns: Option<TableWithColumnsResource>,


    /// 
    /// The table for the resource. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal.
    /// 
    /// Required: No
    ///
    /// Type: TableResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Table")]
    pub table: Option<TableResource>,


    /// 
    /// A data cell filter.
    /// 
    /// Required: No
    ///
    /// Type: DataCellsFilterResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataCellsFilter")]
    pub data_cells_filter: Option<DataCellsFilterResource>,


    /// 
    /// The database for the resource. Unique to the Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database permissions to a principal.
    /// 
    /// Required: No
    ///
    /// Type: DatabaseResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Database")]
    pub database: Option<DatabaseResource>,

}


/// The LF-tag key and values attached to a resource.
#[derive(Default, serde::Serialize)]
pub struct LFTag {


    /// 
    /// A list of possible values of the corresponding TagKey of an LF-tag key-value pair.
    /// 
    /// Required: No
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagValues")]
    pub tag_values: Option<Vec<String>>,


    /// 
    /// The key-name for the LF-tag.
    /// 
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagKey")]
    pub tag_key: Option<String>,

}


/// A structure for the database object.
#[derive(Default, serde::Serialize)]
pub struct DatabaseResource {


    /// 
    /// The identifier for the Data Catalog. By default, it is the account ID of the caller.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,


    /// 
    /// The name of the database resource. Unique to the Data Catalog.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "Name")]
    pub name: String,

}
