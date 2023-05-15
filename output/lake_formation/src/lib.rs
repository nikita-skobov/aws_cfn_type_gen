
pub mod cfn_data_cells_filter {

#[derive(serde::Serialize, Default)]
pub struct CfnDataCellsFilter {
    /// The name of the Database that the Table resides in.
    #[serde(rename = "DatabaseName")]
    pub database_name: NameString,
    /// The name of the Table to create a Data Cells Filter for.
    #[serde(rename = "TableName")]
    pub table_name: NameString,
    /// A list of columns to be included in this Data Cells Filter.
    #[serde(rename = "ColumnNames")]
    pub column_names: Option<ColumnNames>,
    /// The Catalog Id of the Table on which to create a Data Cells Filter.
    #[serde(rename = "TableCatalogId")]
    pub table_catalog_id: CatalogIdString,
    /// The desired name of the Data Cells Filter.
    #[serde(rename = "Name")]
    pub name: NameString,
    /// An object representing the Data Cells Filter's Row Filter. Either a Filter Expression or a Wildcard is required
    #[serde(rename = "RowFilter")]
    pub row_filter: Option<RowFilter>,
    /// An object representing the Data Cells Filter's Columns. Either Column Names or a Wildcard is required
    #[serde(rename = "ColumnWildcard")]
    pub column_wildcard: Option<ColumnWildcard>,

}

pub type NameString = String;
#[derive(serde::Serialize, Default)]
pub struct RowFilter {
    #[serde(rename = "FilterExpression")]
    pub filter_expression: Option<String>,
    #[serde(rename = "AllRowsWildcard")]
    pub all_rows_wildcard: Option<()>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnNames {

}

#[derive(serde::Serialize, Default)]
pub struct ColumnWildcard {
    #[serde(rename = "ExcludedColumnNames")]
    pub excluded_column_names: Option<ColumnNames>,

}
pub type CatalogIdString = String;

}

pub mod cfn_data_lake_settings {

#[derive(serde::Serialize, Default)]
pub struct CfnDataLakeSettings {
    /// No documentation provided by AWS
    #[serde(rename = "CreateTableDefaultPermissions")]
    pub create_table_default_permissions: Option<CreateTableDefaultPermissions>,
    /// No documentation provided by AWS
    #[serde(rename = "CreateDatabaseDefaultPermissions")]
    pub create_database_default_permissions: Option<CreateDatabaseDefaultPermissions>,
    /// No documentation provided by AWS
    #[serde(rename = "Parameters")]
    pub parameters: Option<()>,
    /// No documentation provided by AWS
    #[serde(rename = "TrustedResourceOwners")]
    pub trusted_resource_owners: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "AllowExternalDataFiltering")]
    pub allow_external_data_filtering: Option<bool>,
    /// No documentation provided by AWS
    #[serde(rename = "ExternalDataFilteringAllowList")]
    pub external_data_filtering_allow_list: Option<ExternalDataFilteringAllowList>,
    /// No documentation provided by AWS
    #[serde(rename = "Admins")]
    pub admins: Option<Admins>,
    /// No documentation provided by AWS
    #[serde(rename = "AuthorizedSessionTagValueList")]
    pub authorized_session_tag_value_list: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct CreateTableDefaultPermissions {

}

#[derive(serde::Serialize, Default)]
pub struct ExternalDataFilteringAllowList {

}

#[derive(serde::Serialize, Default)]
pub struct Admins {

}

#[derive(serde::Serialize, Default)]
pub struct CreateDatabaseDefaultPermissions {

}


}

pub mod cfn_permissions {

#[derive(serde::Serialize, Default)]
pub struct CfnPermissions {
    /// No documentation provided by AWS
    #[serde(rename = "Resource")]
    pub resource: Resource,
    /// No documentation provided by AWS
    #[serde(rename = "Permissions")]
    pub permissions: Option<Vec<String>>,
    /// No documentation provided by AWS
    #[serde(rename = "DataLakePrincipal")]
    pub data_lake_principal: DataLakePrincipal,
    /// No documentation provided by AWS
    #[serde(rename = "PermissionsWithGrantOption")]
    pub permissions_with_grant_option: Option<Vec<String>>,

}


#[derive(serde::Serialize, Default)]
pub struct DataLakePrincipal {
    #[serde(rename = "DataLakePrincipalIdentifier")]
    pub data_lake_principal_identifier: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct DatabaseResource {
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TableWithColumnsResource {
    #[serde(rename = "ColumnNames")]
    pub column_names: Option<Vec<String>>,
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ColumnWildcard")]
    pub column_wildcard: Option<ColumnWildcard>,
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct Resource {
    #[serde(rename = "DatabaseResource")]
    pub database_resource: Option<DatabaseResource>,
    #[serde(rename = "DataLocationResource")]
    pub data_location_resource: Option<DataLocationResource>,
    #[serde(rename = "TableWithColumnsResource")]
    pub table_with_columns_resource: Option<TableWithColumnsResource>,
    #[serde(rename = "TableResource")]
    pub table_resource: Option<TableResource>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnWildcard {
    #[serde(rename = "ExcludedColumnNames")]
    pub excluded_column_names: Option<Vec<String>>,

}

#[derive(serde::Serialize, Default)]
pub struct DataLocationResource {
    #[serde(rename = "S3Resource")]
    pub s3_resource: Option<String>,
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct TableWildcard {

}

#[derive(serde::Serialize, Default)]
pub struct TableResource {
    #[serde(rename = "DatabaseName")]
    pub database_name: Option<String>,
    #[serde(rename = "TableWildcard")]
    pub table_wildcard: Option<TableWildcard>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<String>,

}


}

pub mod cfn_principal_permissions {

#[derive(serde::Serialize, Default)]
pub struct CfnPrincipalPermissions {
    /// No documentation provided by AWS
    #[serde(rename = "Catalog")]
    pub catalog: Option<CatalogIdString>,
    /// No documentation provided by AWS
    #[serde(rename = "Principal")]
    pub principal: DataLakePrincipal,
    /// No documentation provided by AWS
    #[serde(rename = "Resource")]
    pub resource: Resource,
    /// No documentation provided by AWS
    #[serde(rename = "PermissionsWithGrantOption")]
    pub permissions_with_grant_option: PermissionList,
    /// No documentation provided by AWS
    #[serde(rename = "Permissions")]
    pub permissions: PermissionList,

}

pub type CatalogIdString = String;
#[derive(serde::Serialize, Default)]
pub struct LFTagPair {
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<CatalogIdString>,
    #[serde(rename = "TagKey")]
    pub tag_key: LFTagKey,
    #[serde(rename = "TagValues")]
    pub tag_values: TagValueList,

}

#[derive(serde::Serialize, Default)]
pub struct DatabaseResource {
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,
    #[serde(rename = "Name")]
    pub name: NameString,

}
pub type PathString = String;
#[derive(serde::Serialize, Default)]
pub struct LFTagsList {

}

#[derive(serde::Serialize, Default)]
pub struct DataLakePrincipal {
    #[serde(rename = "DataLakePrincipalIdentifier")]
    pub data_lake_principal_identifier: Option<DataLakePrincipalString>,

}

#[derive(serde::Serialize, Default)]
pub struct DataCellsFilterResource {
    #[serde(rename = "TableName")]
    pub table_name: NameString,
    #[serde(rename = "TableCatalogId")]
    pub table_catalog_id: CatalogIdString,
    #[serde(rename = "DatabaseName")]
    pub database_name: NameString,
    #[serde(rename = "Name")]
    pub name: NameString,

}
pub type LFTagValue = String;
#[derive(serde::Serialize, Default)]
pub struct DataLakePrincipalList {

}

#[derive(serde::Serialize, Default)]
pub struct TagValueList {

}

#[derive(serde::Serialize, Default)]
pub struct TableWithColumnsResource {
    #[serde(rename = "DatabaseName")]
    pub database_name: NameString,
    #[serde(rename = "ColumnWildcard")]
    pub column_wildcard: Option<ColumnWildcard>,
    #[serde(rename = "ColumnNames")]
    pub column_names: Option<ColumnNames>,
    #[serde(rename = "Name")]
    pub name: NameString,
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,

}

#[derive(serde::Serialize, Default)]
pub struct LFTag {
    #[serde(rename = "TagValues")]
    pub tag_values: Option<TagValueList>,
    #[serde(rename = "TagKey")]
    pub tag_key: Option<LFTagKey>,

}

#[derive(serde::Serialize, Default)]
pub struct LFTagKeyResource {
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,
    #[serde(rename = "TagKey")]
    pub tag_key: NameString,
    #[serde(rename = "TagValues")]
    pub tag_values: TagValueList,

}

#[derive(serde::Serialize, Default)]
pub struct Expression {

}

#[derive(serde::Serialize, Default)]
pub struct TableResource {
    #[serde(rename = "DatabaseName")]
    pub database_name: NameString,
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,
    #[serde(rename = "TableWildcard")]
    pub table_wildcard: Option<TableWildcard>,
    #[serde(rename = "Name")]
    pub name: Option<NameString>,

}

#[derive(serde::Serialize, Default)]
pub struct Resource {
    #[serde(rename = "LFTagPolicy")]
    pub lftag_policy: Option<LFTagPolicyResource>,
    #[serde(rename = "Catalog")]
    pub catalog: Option<CatalogResource>,
    #[serde(rename = "TableWithColumns")]
    pub table_with_columns: Option<TableWithColumnsResource>,
    #[serde(rename = "DataLocation")]
    pub data_location: Option<DataLocationResource>,
    #[serde(rename = "Table")]
    pub table: Option<TableResource>,
    #[serde(rename = "DataCellsFilter")]
    pub data_cells_filter: Option<DataCellsFilterResource>,
    #[serde(rename = "LFTag")]
    pub lftag: Option<LFTagKeyResource>,
    #[serde(rename = "Database")]
    pub database: Option<DatabaseResource>,

}

#[derive(serde::Serialize, Default)]
pub struct NullableBoolean {

}
pub type DataLakePrincipalString = String;
#[derive(serde::Serialize, Default)]
pub struct PrincipalPermissionsList {

}
pub type ResourceArnString = String;pub type NameString = String;
#[derive(serde::Serialize, Default)]
pub struct PermissionList {

}

#[derive(serde::Serialize, Default)]
pub struct CatalogResource {

}
pub type IAMRoleArn = String;
#[derive(serde::Serialize, Default)]
pub struct ColumnWildcard {
    #[serde(rename = "ExcludedColumnNames")]
    pub excluded_column_names: Option<ColumnNames>,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnNames {

}

#[derive(serde::Serialize, Default)]
pub struct TableWildcard {

}

#[derive(serde::Serialize, Default)]
pub struct LFTagPolicyResource {
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,
    #[serde(rename = "ResourceType")]
    pub resource_type: ResourceType,
    #[serde(rename = "Expression")]
    pub expression: Expression,

}
pub type LFTagKey = String;pub type ResourceType = String;
#[derive(serde::Serialize, Default)]
pub struct DataLocationResource {
    #[serde(rename = "ResourceArn")]
    pub resource_arn: ResourceArnString,
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,

}

#[derive(serde::Serialize, Default)]
pub struct PrincipalResourcePair {
    #[serde(rename = "Resource")]
    pub resource: Resource,
    #[serde(rename = "Principal")]
    pub principal: DataLakePrincipal,

}

#[derive(serde::Serialize, Default)]
pub struct PrincipalPermissions {
    #[serde(rename = "DataLakePrincipal")]
    pub data_lake_principal: Option<DataLakePrincipal>,
    #[serde(rename = "PermissionList")]
    pub permission_list: Option<PermissionList>,

}
pub type Permission = String;

}

pub mod cfn_resource {

#[derive(serde::Serialize, Default)]
pub struct CfnResource {
    /// No documentation provided by AWS
    #[serde(rename = "RoleArn")]
    pub role_arn: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// No documentation provided by AWS
    #[serde(rename = "UseServiceLinkedRole")]
    pub use_service_linked_role: bool,
    /// No documentation provided by AWS
    #[serde(rename = "WithFederation")]
    pub with_federation: Option<bool>,

}



}

pub mod cfn_tag {

#[derive(serde::Serialize, Default)]
pub struct CfnTag {
    /// The identifier for the Data Catalog. By default, the account ID. The Data Catalog is the persistent metadata store. It contains database definitions, table definitions, and other control information to manage your Lake Formation environment.
    #[serde(rename = "CatalogId")]
    pub catalog_id: Option<CatalogIdString>,
    /// A list of possible values an attribute can take.
    #[serde(rename = "TagValues")]
    pub tag_values: TagValueList,
    /// The key-name for the LF-tag.
    #[serde(rename = "TagKey")]
    pub tag_key: LFTagKey,

}

pub type CatalogIdString = String;pub type LFTagValue = String;
#[derive(serde::Serialize, Default)]
pub struct TagValueList {

}
pub type LFTagKey = String;

}

pub mod cfn_tag_association {

#[derive(serde::Serialize, Default)]
pub struct CfnTagAssociation {
    /// Resource to tag with the Lake Formation Tags
    #[serde(rename = "Resource")]
    pub resource: Resource,
    /// List of Lake Formation Tags to associate with the Lake Formation Resource
    #[serde(rename = "LFTags")]
    pub lftags: LFTagsList,

}


#[derive(serde::Serialize, Default)]
pub struct CatalogResource {

}

#[derive(serde::Serialize, Default)]
pub struct TableWildcard {

}

#[derive(serde::Serialize, Default)]
pub struct TableResource {
    #[serde(rename = "DatabaseName")]
    pub database_name: NameString,
    #[serde(rename = "TableWildcard")]
    pub table_wildcard: Option<TableWildcard>,
    #[serde(rename = "Name")]
    pub name: Option<NameString>,
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,

}

#[derive(serde::Serialize, Default)]
pub struct ColumnNames {

}

#[derive(serde::Serialize, Default)]
pub struct TagValueList {

}
pub type CatalogIdString = String;pub type DataLakePrincipalString = String;pub type LFTagKey = String;
#[derive(serde::Serialize, Default)]
pub struct DatabaseResource {
    #[serde(rename = "Name")]
    pub name: NameString,
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,

}

#[derive(serde::Serialize, Default)]
pub struct Resource {
    #[serde(rename = "Database")]
    pub database: Option<DatabaseResource>,
    #[serde(rename = "Catalog")]
    pub catalog: Option<CatalogResource>,
    #[serde(rename = "TableWithColumns")]
    pub table_with_columns: Option<TableWithColumnsResource>,
    #[serde(rename = "Table")]
    pub table: Option<TableResource>,

}

#[derive(serde::Serialize, Default)]
pub struct LFTagsList {

}

#[derive(serde::Serialize, Default)]
pub struct TableWithColumnsResource {
    #[serde(rename = "DatabaseName")]
    pub database_name: NameString,
    #[serde(rename = "Name")]
    pub name: NameString,
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,
    #[serde(rename = "ColumnNames")]
    pub column_names: ColumnNames,

}
pub type LFTagValue = String;pub type NameString = String;
#[derive(serde::Serialize, Default)]
pub struct DataLakePrincipal {
    #[serde(rename = "DataLakePrincipalIdentifier")]
    pub data_lake_principal_identifier: Option<DataLakePrincipalString>,

}
pub type ResourceType = String;
#[derive(serde::Serialize, Default)]
pub struct LFTagPair {
    #[serde(rename = "CatalogId")]
    pub catalog_id: CatalogIdString,
    #[serde(rename = "TagValues")]
    pub tag_values: TagValueList,
    #[serde(rename = "TagKey")]
    pub tag_key: LFTagKey,

}


}
