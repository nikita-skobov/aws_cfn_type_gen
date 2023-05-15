
pub mod cfn_application {

#[derive(serde::Serialize, Default)]
pub struct CfnApplication {
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    /// The description of the application.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// The name of the application.
    #[serde(rename = "Name")]
    pub name: String,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_attribute_group {

#[derive(serde::Serialize, Default)]
pub struct CfnAttributeGroup {
    /// The name of the attribute group.
    #[serde(rename = "Name")]
    pub name: String,
    /// The description of the attribute group.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// No documentation provided by AWS
    #[serde(rename = "Attributes")]
    pub attributes: (),
    /// No documentation provided by AWS
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,

}


#[derive(serde::Serialize, Default)]
pub struct Tags {

}


}

pub mod cfn_attribute_group_association {

#[derive(serde::Serialize, Default)]
pub struct CfnAttributeGroupAssociation {
    /// The name or the Id of the Application.
    #[serde(rename = "Application")]
    pub application: String,
    /// The name or the Id of the AttributeGroup.
    #[serde(rename = "AttributeGroup")]
    pub attribute_group: String,

}



}

pub mod cfn_resource_association {

#[derive(serde::Serialize, Default)]
pub struct CfnResourceAssociation {
    /// The name or the Id of the Resource.
    #[serde(rename = "Resource")]
    pub resource: String,
    /// The name or the Id of the Application.
    #[serde(rename = "Application")]
    pub application: String,
    /// The type of the CFN Resource for now it's enum CFN_STACK.
    #[serde(rename = "ResourceType")]
    pub resource_type: String,

}



}
