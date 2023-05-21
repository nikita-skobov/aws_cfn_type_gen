

/// The AWS::LakeFormation::TagAssociation resource represents an assignment of an LF-tag to a Data Catalog resource (database, table, or column).     During a stack operation, CloudFormation calls AWS Lake Formation AddLFTagsToResource API to create a TagAssociation resource and calls the RemoveLFTagsToResource API to delete it.
#[derive(Default, serde::Serialize)]
pub struct CfnTagAssociation {


    /// 
    /// A structure containing an LF-tag key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: List of LFTagPair
    ///
    /// Update requires: Replacement
    #[serde(rename = "LFTags")]
    pub lftags: Vec<LFTagPair>,


    /// 
    /// UTF-8 string (valid values: DATABASE | TABLE).
    /// 
    /// The resource for which the LF-tag policy applies.
    /// 
    /// Required: Yes
    ///
    /// Type: Resource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Resource")]
    pub resource: Resource,

}


/// A structure for the table object. A table is a metadata definition that represents your data. You can Grant and Revoke table privileges to a principal.
#[derive(Default, serde::Serialize)]
pub struct TableResource {


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
    /// A wildcard object representing every table under a database.This is an object with no properties that effectively behaves as a true or false depending on whether not it is passed as a parameter.     The valid inputs for a property with this type in either yaml or json is null or {}.
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

}


/// A structure for the database object.
#[derive(Default, serde::Serialize)]
pub struct DatabaseResource {


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


    /// 
    /// The identifier for the Data Catalog. By default, it should be the account ID of the caller.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,

}


/// A structure for a table with columns object. This object is only used when granting a SELECT permission.
///
/// This object must take a value for at least one of ColumnsNames, ColumnsIndexes, or ColumnsWildcard.
#[derive(Default, serde::Serialize)]
pub struct TableWithColumnsResource {


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
    /// The list of column names for the table. At least one of ColumnNames or ColumnWildcard is required.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "ColumnNames")]
    pub column_names: Vec<String>,


    /// 
    /// A wildcard object representing every table under a database.
    /// 
    /// At least one of TableResource$Name or TableResource$TableWildcard is required.
    /// 
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "CatalogId")]
    pub catalog_id: String,


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

}


/// A structure for the resource.
#[derive(Default, serde::Serialize)]
pub struct Resource {


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
    /// The database for the resource. Unique to the Data Catalog. A database is a set of associated table definitions organized into a logical group. You can Grant and Revoke database permissions to a principal.
    /// 
    /// Required: No
    ///
    /// Type: DatabaseResource
    ///
    /// Update requires: Replacement
    #[serde(rename = "Database")]
    pub database: Option<DatabaseResource>,


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

}


/// A structure containing the catalog ID, tag key, and tag values of an LF-tag key-value pair.
#[derive(Default, serde::Serialize)]
pub struct LFTagPair {


    /// 
    /// A list of possible values of the corresponding TagKey of an LF-tag key-value pair.
    /// 
    /// Required: Yes
    ///
    /// Type: List of String
    ///
    /// Update requires: Replacement
    #[serde(rename = "TagValues")]
    pub tag_values: Vec<String>,


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

}