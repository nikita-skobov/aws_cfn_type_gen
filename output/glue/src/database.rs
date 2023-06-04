/// The AWS::Glue::Database resource specifies a logical grouping of tables       in AWS Glue. For more information, see Defining a Database in Your Data         Catalog and Database Structure in the AWS Glue Developer       Guide.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub catalog_id: cfn_resources::StrVal,

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
    fn type_string(&self) -> &'static str {
        "AWS::Glue::Database"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        let the_val = &self.catalog_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 255 as _ {
                return Err(format!(
                    "Max validation failed on field 'catalog_id'. {} is greater than 255",
                    s.len()
                ));
            }
        }

        let the_val = &self.catalog_id;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'catalog_id'. {} is less than 1",
                    s.len()
                ));
            }
        }

        self.database_input.validate()?;

        Ok(())
    }
}

/// The AWS Lake Formation principal.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_principal_identifier: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DataLakePrincipal {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.data_lake_principal_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!("Max validation failed on field 'data_lake_principal_identifier'. {} is greater than 255", s.len()));
                }
            }
        }

        if let Some(the_val) = &self.data_lake_principal_identifier {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!("Min validation failed on field 'data_lake_principal_identifier'. {} is less than 1", s.len()));
                }
            }
        }

        Ok(())
    }
}

/// A structure that describes a target database for resource linking.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<cfn_resources::StrVal>,

    ///
    /// The name of the catalog database.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for DatabaseIdentifier {
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

/// The structure used to create or update a database.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: FederatedDatabase
    ///
    /// Update requires: No interruption
    #[serde(rename = "FederatedDatabase")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_uri: Option<cfn_resources::StrVal>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<cfn_resources::StrVal>,

    ///
    /// These key-value pairs define parameters and properties    of the database.
    ///
    /// Required: No
    ///
    /// Type: Json
    ///
    /// Update requires: No interruption
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_database: Option<DatabaseIdentifier>,
}

impl cfn_resources::CfnResource for DatabaseInput {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 2048 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 2048",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 0 as _ {
                    return Err(format!(
                        "Min validation failed on field 'description'. {} is less than 0",
                        s.len()
                    ));
                }
            }
        }

        self.federated_database
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.location_uri {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1024 as _ {
                    return Err(format!(
                        "Max validation failed on field 'location_uri'. {} is greater than 1024",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.location_uri {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'location_uri'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 255 as _ {
                    return Err(format!(
                        "Max validation failed on field 'name'. {} is greater than 255",
                        s.len()
                    ));
                }
            }
        }

        if let Some(the_val) = &self.name {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() < 1 as _ {
                    return Err(format!(
                        "Min validation failed on field 'name'. {} is less than 1",
                        s.len()
                    ));
                }
            }
        }

        self.target_database
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}

/// The FederatedDatabase property type specifies Property description not available. for an AWS::Glue::Database.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FederatedDatabase {
    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "ConnectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<cfn_resources::StrVal>,

    /// Property description not available.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: No interruption
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<cfn_resources::StrVal>,
}

impl cfn_resources::CfnResource for FederatedDatabase {
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

/// the permissions granted to a principal
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
}

impl cfn_resources::CfnResource for PrincipalPrivileges {
    fn type_string(&self) -> &'static str {
        "NOT_A_VALID_CFN_RESOURCE"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.principal
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        Ok(())
    }
}
