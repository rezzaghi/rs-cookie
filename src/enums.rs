pub enum Expires{
    Default,
    SetExpireDate(&'static str)
}

pub enum Path{
    Default,
    SetPath(&'static str)
}

#[derive(PartialEq)]
pub enum Domain{
    Default,
    SetDomain(&'static str)
}

#[derive(PartialEq)]
pub enum Secure {
    Secure,
    NonSecure,
}

#[derive(PartialEq)]
pub enum HttpOnly {
    HttpOnly,
    NonHttpOnly,
}
