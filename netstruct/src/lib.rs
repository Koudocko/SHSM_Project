use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Account{
    mode: String,
    username: String,
    password: String
}

#[derive(Clone)]
pub enum Page{
    Certifications, 
    ShsmSelection,
    Events,
    Login,
    Home
}
