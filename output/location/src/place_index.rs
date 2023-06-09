/// Specifies a place index resource in your AWS account. Use a place index resource to       geocode addresses and other text queries by using the         SearchPlaceIndexForText operation, and reverse geocode coordinates by       using the SearchPlaceIndexForPosition operation, and enable autosuggestions       by using the SearchPlaceIndexForSuggestions operation.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CfnPlaceIndex {
    ///
    /// Specifies the geospatial data provider for the new place index.
    ///
    /// NoteThis field is case-sensitive. Enter the valid values as shown. For example,         entering HERE returns an error.
    ///
    /// Valid values include:
    ///
    /// Esri – For additional information about Esri's coverage in your region of interest, see Esri details on geocoding coverage.               Grab – Grab provides place index functionality for Southeast           Asia. For additional information about GrabMaps' coverage, see GrabMaps countries and areas covered.               Here – For additional information about HERE             Technologies' coverage in your region of interest, see HERE details on goecoding coverage.         ImportantIf you specify HERE Technologies (Here) as the data provider,             you may not store results for locations in Japan. For more information, see             the AWS Service               Terms for Amazon Location Service.
    ///
    /// For additional information , see Data         providers on the Amazon Location Service Developer Guide.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSource")]
    pub data_source: cfn_resources::StrVal,

    ///
    /// Specifies the data storage option requesting Places.
    ///
    /// Required: No
    ///
    /// Type: DataSourceConfiguration
    ///
    /// Update requires: Replacement
    #[serde(rename = "DataSourceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_configuration: Option<DataSourceConfiguration>,

    ///
    /// The optional description for the place index resource.
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Minimum: 0
    ///
    /// Maximum: 1000
    ///
    /// Update requires: Replacement
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<cfn_resources::StrVal>,

    ///
    /// The name of the place index resource.
    ///
    /// Requirements:
    ///
    /// Contain only alphanumeric characters (A–Z, a–z, 0–9), hyphens (-), periods           (.), and underscores (_).               Must be a unique place index resource name.               No spaces allowed. For example, ExamplePlaceIndex.
    ///
    /// Required: Yes
    ///
    /// Type: String
    ///
    /// Minimum: 1
    ///
    /// Maximum: 100
    ///
    /// Pattern: ^[-._\w]+$
    ///
    /// Update requires: Replacement
    #[serde(rename = "IndexName")]
    pub index_name: cfn_resources::StrVal,

    ///
    /// No longer used. If included, the only allowed value is       RequestBasedUsage.
    ///
    /// Allowed Values: RequestBasedUsage
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Update requires: Replacement
    #[serde(rename = "PricingPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<PlaceIndexPricingPlanEnum>,

    #[serde(skip_serializing)]
    pub att_arn: CfnPlaceIndexarn,

    #[serde(skip_serializing)]
    pub att_create_time: CfnPlaceIndexcreatetime,

    #[serde(skip_serializing)]
    pub att_index_arn: CfnPlaceIndexindexarn,

    #[serde(skip_serializing)]
    pub att_update_time: CfnPlaceIndexupdatetime,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum PlaceIndexPricingPlanEnum {
    /// RequestBasedUsage
    #[serde(rename = "RequestBasedUsage")]
    Requestbasedusage,
}

impl Default for PlaceIndexPricingPlanEnum {
    fn default() -> Self {
        PlaceIndexPricingPlanEnum::Requestbasedusage
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlaceIndexarn;
impl CfnPlaceIndexarn {
    pub fn att_name(&self) -> &'static str {
        r#"Arn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlaceIndexcreatetime;
impl CfnPlaceIndexcreatetime {
    pub fn att_name(&self) -> &'static str {
        r#"CreateTime"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlaceIndexindexarn;
impl CfnPlaceIndexindexarn {
    pub fn att_name(&self) -> &'static str {
        r#"IndexArn"#
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CfnPlaceIndexupdatetime;
impl CfnPlaceIndexupdatetime {
    pub fn att_name(&self) -> &'static str {
        r#"UpdateTime"#
    }
}

impl cfn_resources::CfnResource for CfnPlaceIndex {
    fn type_string(&self) -> &'static str {
        "AWS::Location::PlaceIndex"
    }

    fn properties(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Failed to serialize to value")
    }

    fn validate(&self) -> Result<(), String> {
        self.data_source_configuration
            .as_ref()
            .map_or(Ok(()), |val| val.validate())?;

        if let Some(the_val) = &self.description {
            if let cfn_resources::StrVal::String(s) = &the_val {
                if s.len() > 1000 as _ {
                    return Err(format!(
                        "Max validation failed on field 'description'. {} is greater than 1000",
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

        let the_val = &self.index_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() > 100 as _ {
                return Err(format!(
                    "Max validation failed on field 'index_name'. {} is greater than 100",
                    s.len()
                ));
            }
        }

        let the_val = &self.index_name;

        if let cfn_resources::StrVal::String(s) = &the_val {
            if s.len() < 1 as _ {
                return Err(format!(
                    "Min validation failed on field 'index_name'. {} is less than 1",
                    s.len()
                ));
            }
        }

        Ok(())
    }
}

/// Specifies the data storage option requesting Places.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DataSourceConfiguration {
    ///
    /// Specifies how the results of an operation will be stored by the caller.
    ///
    /// Valid values include:
    ///
    /// SingleUse specifies that the results won't be stored.                Storage specifies that the result can be cached or stored in a           database.
    ///
    /// Default value: SingleUse
    ///
    /// Required: No
    ///
    /// Type: String
    ///
    /// Allowed values: SingleUse | Storage
    ///
    /// Update requires: Replacement
    #[serde(rename = "IntendedUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_use: Option<DataSourceConfigurationIntendedUseEnum>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DataSourceConfigurationIntendedUseEnum {
    /// SingleUse
    #[serde(rename = "SingleUse")]
    Singleuse,

    /// Storage
    #[serde(rename = "Storage")]
    Storage,
}

impl Default for DataSourceConfigurationIntendedUseEnum {
    fn default() -> Self {
        DataSourceConfigurationIntendedUseEnum::Singleuse
    }
}

impl cfn_resources::CfnResource for DataSourceConfiguration {
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
