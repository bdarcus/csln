use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum StyleTemplate {
    Contributor(StyleTemplateContributor),
    Date(StyleTemplateDate),
    Title(StyleTemplateTitle),
}

// TODO align with TS
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateContributor {
    pub contributor: String,
    pub form: ContributorForm,
}

// TODO align with TS
#[derive(Deserialize, Serialize, JsonSchema)]
pub enum ContributorForm {
    Long,
    Short,
}

// TODO align with TS
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateDate {
    pub date: String,
    pub form: DateForm,
}

// TODO align with TS
#[derive(Deserialize, Serialize, JsonSchema)]
pub enum DateForm {
    Text,
    Numeric,
    Roman,
    Ordinal,
    Short,
    Long,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateTitle {
    pub title: String,
    pub form: TitleForm,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum TitleForm {
    Short,
    Long,
}
