
pub mod cfn_allow_list {

#[derive(serde::Serialize, Default)]
pub struct CfnAllowList {
    /// AllowList criteria.
    #[serde(rename = "Criteria")]
    pub criteria: Criteria,
    /// Description of AllowList.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Name of AllowList.
    #[serde(rename = "Name")]
    pub name: String,
    /// A collection of tags associated with a resource
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<Tag>>,

}

pub type Regex = String;
#[derive(serde::Serialize, Default)]
pub struct S3WordsList {
    #[serde(rename = "ObjectKey")]
    pub object_key: String,
    #[serde(rename = "BucketName")]
    pub bucket_name: String,

}
pub type Status = String;
#[derive(serde::Serialize, Default)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,

}

#[derive(serde::Serialize, Default)]
pub struct Criteria {

}


}

pub mod cfn_custom_data_identifier {

#[derive(serde::Serialize, Default)]
pub struct CfnCustomDataIdentifier {
    /// Regular expression for custom data identifier.
    #[serde(rename = "Regex")]
    pub regex: String,
    /// Keywords to be matched against.
    #[serde(rename = "Keywords")]
    pub keywords: Option<Vec<String>>,
    /// Name of custom data identifier.
    #[serde(rename = "Name")]
    pub name: String,
    /// Description of custom data identifier.
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Words to be ignored.
    #[serde(rename = "IgnoreWords")]
    pub ignore_words: Option<Vec<String>>,
    /// Maximum match distance.
    #[serde(rename = "MaximumMatchDistance")]
    pub maximum_match_distance: Option<usize>,

}



}

pub mod cfn_findings_filter {

#[derive(serde::Serialize, Default)]
pub struct CfnFindingsFilter {
    /// Findings filter name
    #[serde(rename = "Name")]
    pub name: String,
    /// Findings filter criteria.
    #[serde(rename = "FindingCriteria")]
    pub finding_criteria: FindingCriteria,
    /// Findings filter action.
    #[serde(rename = "Action")]
    pub action: Option<FindingFilterAction>,
    /// Findings filter description
    #[serde(rename = "Description")]
    pub description: Option<String>,
    /// Findings filter position.
    #[serde(rename = "Position")]
    pub position: Option<usize>,

}


#[derive(serde::Serialize, Default)]
pub struct FindingCriteria {
    #[serde(rename = "Criterion")]
    pub criterion: Option<Criterion>,

}
pub type FindingFilterAction = String;
#[derive(serde::Serialize, Default)]
pub struct Criterion {

}

#[derive(serde::Serialize, Default)]
pub struct FindingsFilterListItem {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,

}

#[derive(serde::Serialize, Default)]
pub struct CriterionAdditionalProperties {
    #[serde(rename = "gte")]
    pub gte: Option<usize>,
    #[serde(rename = "lte")]
    pub lte: Option<usize>,
    #[serde(rename = "eq")]
    pub eq: Option<Vec<String>>,
    #[serde(rename = "gt")]
    pub gt: Option<usize>,
    #[serde(rename = "lt")]
    pub lt: Option<usize>,
    #[serde(rename = "neq")]
    pub neq: Option<Vec<String>>,

}


}

pub mod cfn_session {

#[derive(serde::Serialize, Default)]
pub struct CfnSession {
    /// A enumeration value that specifies how frequently finding updates are published.
    #[serde(rename = "FindingPublishingFrequency")]
    pub finding_publishing_frequency: Option<String>,
    /// A enumeration value that specifies the status of the Macie Session.
    #[serde(rename = "Status")]
    pub status: Option<String>,

}



}
