use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use style::template::{
    Contributors, DateForm, Dates, StyleTemplateContributor, StyleTemplateDate,
    StyleTemplateList, StyleTemplateTitle, Titles,
};
//use edtf::DateComplete;

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonSchema, PartialEq)]
pub struct InputReference {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<Vec<String>>,
    pub editor: Option<Vec<String>>,
    pub translator: Option<Vec<String>>,
    pub issued: Option<String>,
    pub publisher: Option<Vec<String>>,
    pub url: Option<String>,
    pub accessed: Option<String>,
    pub note: Option<String>,
}

impl InputReference {
    pub fn render_names(names: Vec<String>) -> String {
        let mut name_string = String::new();
        if names.len() == 1 {
            name_string = names[0].to_string();
        } else if names.len() == 2 {
            name_string = names.join(" and ");
        } else if names.len() > 2 {
            let last_author = names.last().unwrap();
            let other_authors = &names[..names.len() - 1];
            name_string = other_authors.join(", ");
            name_string.push_str(", and ");
            name_string.push_str(last_author);
        }
        name_string
    }

    pub fn render_contributors(
        &self,
        template_component: &StyleTemplateContributor,
    ) -> String {
        match template_component.contributor {
            Contributors::Author => {
                let authors = self
                    .author
                    .as_ref()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|name| name.to_string())
                    .collect::<Vec<String>>();
                InputReference::render_names(authors)
            }
            Contributors::Editor => {
                let editors = self
                    .editor
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|editor| editor.to_string())
                    .collect::<Vec<String>>();
                InputReference::render_names(editors)
            }
            Contributors::Translator => {
                let translators = self
                    .translator
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|translator| translator.to_string())
                    .collect::<Vec<String>>();
                InputReference::render_names(translators)
            }
            Contributors::Director => todo!(),
            Contributors::Recipient => todo!(),
            Contributors::Interviewer => todo!(),
            Contributors::Interviewee => todo!(),
            Contributors::Inventor => todo!(),
            Contributors::Counsel => todo!(),
            Contributors::Composer => todo!(),
            Contributors::Publisher => todo!(),
        }
    }
    pub fn render_date(&self, template_component: &StyleTemplateDate) -> String {
        let date_string: &str = match template_component.date {
            Dates::Issued => self.issued.as_ref().unwrap(),
            Dates::Accessed => todo!(),
            Dates::OriginalPublished => todo!(),
        };

        let format_string: &str = match template_component.form {
            DateForm::Year => "%Y",
            DateForm::YearMonth => "%Y-%m",
            DateForm::Full => "%Y-%m-%d",
            DateForm::MonthDay => "%m-%d",
        };

        // use EDTF instead?
        let date: NaiveDate = NaiveDate::parse_from_str(date_string, "%Y-%m-%d").unwrap();
        let formatted_date: String = date.format(format_string).to_string();
        formatted_date
    }

    pub fn render_title(&self, template_component: &StyleTemplateTitle) -> String {
        let title: &str = match template_component.title {
            Titles::Title => self.title.as_ref().unwrap(),
            Titles::ContainerTitle => todo!(),
        };
        title.to_string()
    }

    pub fn render_list(&self, _template_component: &StyleTemplateList) -> String {
        "".to_string() // TODO
    }
}
