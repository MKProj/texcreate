use crate::Config;
use thiserror::Error;
/// The TexCreate Errors that can occur
/// - Beamer Error: When Beamer template is chosen, but doc_class isn't beamer
/// - Invalid Template: Template doesn't exist
/// - Invalid Document Class: Document Class doesn't exist
/// - Empty Fields: A field in `config.toml` is left empty
/// - IO Error: Error caused by `std::io::Error`
/// - Invalid: Any other error
#[derive(Error, Debug)]
pub enum TexCreateError {
    #[error(
        "'{0}' document_class is illegal for Beamer Template!!!\nPlease set document_class to 'beamer'",
    )]
    BeamerError(String),
    #[error("'{0}' is an invalid Template, use texcreate list for list of available templates")]
    InvalidTemplate(String),
    #[error("'{0}' is an invalid Document Class!")]
    InvalidDocClass(String),
    #[error("The '{0}' field has an empty value!")]
    EmptyFields(String),
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("Invalid '{0}'")]
    Invalid(String),
    #[error("Cannot have both custom_template and template fields, please remove one of them!")]
    BothTemplate,
    #[error("Cannot find custom template directory, make sure to have files under '{0}'.")]
    CannotFindCustom(String),
    #[error("Cannot build project if no template field is present")]
    MissingTemplate
}

fn valid_templates() -> Vec<&'static str> {
    vec![
        "Basic", "Book", "Math", "Theatre", "Code", "Novel", "Beamer", "Lachaise", "Lachaise-Mod", "Dictionary", "News"
    ]
}
fn valid_classes() -> Vec<&'static str> {
    vec![
        "article", "IEEEtran", "proc", "minimal", "report", "book", "slides", "memoir", "letter",
    ]
}

/// Result type for TexCreate
pub type TexCreateResult<T> = std::result::Result<T, TexCreateError>;

/// Checks if config has a beamer error, if so returns `TexCreateError::BeamerError`
pub fn check_beamer_error(config: &Config) -> TexCreateResult<()> {
    if config.template.is_none(){
        return Ok(());
    }
    /*
    Beamer error occurs when the Document class is
    not set as beamer when the Beamer Template is chosen
     */
    if config.template.as_ref().unwrap() == "Beamer" && config.document_class != "beamer" {
        Err(TexCreateError::BeamerError(config.document_class.clone()))
    } else {
        Ok(())
    }
}
/// Explicit Missing Template Check
/// This occurs when both `Config.template == None && Config.custom_template == None`
pub fn check_missing_template(config: &Config) -> TexCreateResult<()>{
    if config.template.is_none() && config.custom_template.is_none() {
        return Err(TexCreateError::MissingTemplate);
    }
    Ok(())
}

/// Checks if config has an invalid template, if so returns `TexCreateError::InvalidTemplate`
pub fn check_invalid_template(config: &Config) -> TexCreateResult<()> {
    /*
    Invalid template error occurs when a user enters a template that
    does not exist, to do this we look at a vec and see if it matches
     */
    let template = match &config.template{
        Some(a) => a.to_string(),
        None => {
            return if config.custom_template == None {
                Err(TexCreateError::MissingTemplate)
            } else {
                Ok(())
            }
        }
    };
    if !valid_templates().contains(&template.as_str()) {
        Err(TexCreateError::InvalidTemplate(template))
    } else {
        Ok(())
    }
}
/// Checks if the config has an invalid document class, if so returns `TexCreateError::InvalidDocClass`
pub fn check_invalid_class(config: &Config) -> TexCreateResult<()> {
    /*
    Invalid class error occurs when a user enters a document
    class that does not exist, we look at a vec and see if it matches
     */
    if !valid_classes().contains(&config.document_class.as_str()) {
        Err(TexCreateError::InvalidDocClass(
            config.document_class.clone(),
        ))
    } else {
        Ok(())
    }
}
/// Checks if the config has any empty fields, if so returns `TexCreateError::EmptyFields`
pub fn check_empty_field(config: &Config) -> TexCreateResult<()> {
    check_missing_template(config)?;
    /*
    Checks each field if empty, ignores fields that are optional
     */
    if config.author.is_empty() {
        Err(TexCreateError::EmptyFields("Author".to_string()))
    } else if config.title.is_empty() {
        Err(TexCreateError::EmptyFields("Title".to_string()))
    } else if config.date.is_empty() {
        Err(TexCreateError::EmptyFields("Date".to_string()))
    } else if config.project_name.is_empty() {
        Err(TexCreateError::EmptyFields("Project Name".to_string()))
    } else if config.document_class.is_empty() {
        Err(TexCreateError::EmptyFields("Document Class".to_string()))
    } else if config.paper_size.is_empty() {
        Err(TexCreateError::EmptyFields("Paper Size".to_string()))
    } else if config.font_size.to_string().is_empty() {
        Err(TexCreateError::EmptyFields("Font Size".to_string()))
    } else {
        Ok(())
    }
}
/// Contains all checks helper functions into one function
pub fn check_errors(config: &Config) -> Result<(), String> {
    /*
    Checks all errors in one function
     */
    if check_beamer_error(config).is_err(){
        return match check_beamer_error(config){
            Err(e) => Err(e.to_string()),
            _ => Err("".to_string())
        }
    }
    if check_invalid_class(config).is_err(){
        return match check_invalid_class(config){
            Err(e) => Err(e.to_string()),
            _ => Err("".to_string())
        }
    }
    if check_missing_template(config).is_err(){
        return match check_missing_template(config){
            Err(e) => Err(e.to_string()),
            _ => Err("".to_string())
        }
    }
    if check_invalid_template(config).is_err(){
        return match check_invalid_template(config){
            Err(e) => Err(e.to_string()),
            _ => Err("".to_string())
        }
    }
    if check_empty_field(config).is_err(){
        return match check_empty_field(config){
            Err(e) => Err(e.to_string()),
            _ => Err("".to_string())
        }
    }
    Ok(())
}
