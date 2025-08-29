use rocket::form::Form;

#[derive(FromForm)]
pub struct UserSearch<'a>{
    pub search_text: & 'a str,
}