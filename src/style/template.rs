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
    pub contributor: Contributors,
    pub form: ContributorForm,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum ContributorForm {
    // REVIEW
    Long,
    Short,
}

// move this to another shared file
#[derive(Deserialize, Serialize, JsonSchema)]
pub enum Contributors {
    Author,
    Editor,
    Translator,
    Director,
    Recipient,
    Interviewer,
    Interviewee,
    Inventor,
    Counsel,
    Composer,
    WordsBy,
    Artist,
    Performer,
    Presenter,
    Commenter,
    Producer,
    CastMember,
    Sponsor,
    CitedAuthor,
    ContainerAuthor,
    OriginalAuthor,
    CollectionEditor,
    EditorialDirector,
    ReviewedAuthor,
    IssuingAuthority,
    Accessed,
    FictitiousAuthor,
    Cartographer,
    Compiler,
    Cosponsor,
    Scriptwriter,
}

// TODO align with TS
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct StyleTemplateDate {
    pub date: Dates,
    pub form: DateForm,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum Dates {
    Issued,
    Accessed,
    OriginalPublished,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum DateForm {
    Year,
    YearMonth,
    Full,
    MonthDay,
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
