use std::collections::HashMap;

#[allow(dead_code)]
pub(crate) struct FtpUser<'a> {
    pub(crate) userid: &'a str,
    pub(crate) homedir:  &'a str,
    pub(crate) newspaper_id: i32,
    pub(crate) settings: Option<HashMap< &'a str, Option< &'a str>>>,
    pub(crate) newspaper_product_id: i32,
    pub(crate) xml:  &'a str
}

// TODO implement with DB
impl FtpUser<'_> {
    pub(crate) fn find_ftp_user(ftp_user: &String) -> FtpUser {
        FtpUser {
            userid: ftp_user.as_str(),
            homedir: "",
            newspaper_id: 0,
            settings: None,
            newspaper_product_id: 0,
            xml: "",
        }
    }
}