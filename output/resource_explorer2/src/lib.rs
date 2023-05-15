
pub mod cfn_default_view_association {

#[derive(serde::Serialize, Default)]
pub struct CfnDefaultViewAssociation {
    /// No documentation provided by AWS
    #[serde(rename = "ViewArn")]
    pub view_arn: String,

}



}

pub mod cfn_index {

#[derive(serde::Serialize, Default)]
pub struct CfnIndex {
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<TagMap>,
    /// No documentation provided by AWS
    #[serde(rename = "Type")]
    pub ty: IndexType,

}


#[derive(serde::Serialize, Default)]
pub struct TagMap {

}
pub type IndexType = String;pub type IndexState = String;

}

pub mod cfn_view {

#[derive(serde::Serialize, Default)]
pub struct CfnView {
    /// List of IncludedProperty
    #[serde(rename = "IncludedProperties")]
    pub included_properties: Option<Vec<IncludedProperty>>,
    /// No documentation provided by AWS
    #[serde(rename = "ViewName")]
    pub view_name: String,
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<TagMap>,
    /// No documentation provided by AWS
    #[serde(rename = "Filters")]
    pub filters: Option<Filters>,

}


#[derive(serde::Serialize, Default)]
pub struct Filters {
    #[serde(rename = "FilterString")]
    pub filter_string: String,

}

#[derive(serde::Serialize, Default)]
pub struct IncludedProperty {
    #[serde(rename = "Name")]
    pub name: String,

}

#[derive(serde::Serialize, Default)]
pub struct TagMap {

}


}
