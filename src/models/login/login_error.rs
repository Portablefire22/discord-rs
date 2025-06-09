use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginError {
    message: String,
    code: isize,
    errors: Vec<ErrorType>,
}


#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorType {
    Login(Vec<ErrorStruct>),
    Password(Vec<ErrorStruct>)
}

#[derive(Deserialize)]
pub struct ErrorStruct {
    #[serde(rename = "_errors")]
    errors: Vec<ErrorInfo>,
}

#[derive(Deserialize)]
pub struct ErrorInfo {
    code: String, 
    message: String,
}
