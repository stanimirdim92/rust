use std::collections::HashMap;

#[derive(Debug)]
struct UserDetail<'a> {
    token: &'a str,
    offset: i32,
}

#[derive(Debug)]
struct UserConfig<'a> {
    event: &'a str,
    url: &'a str,
    users: HashMap<&'a str, UserDetail<'a>>,
}

fn main() {
    let mut users: HashMap<&str, UserConfig> = HashMap::new();

    users.insert("TEST", UserConfig {
        event: "changed",
        url: "https://test.example.com/api/v1/",
        users: HashMap::from([
            ("client1", UserDetail {
                token: "1",
                offset: 0,
            }),
            ("client2", UserDetail {
                token: "2",
                offset: 0,
            }),
            //SVZ not active atm
            ("client3", UserDetail {
                token: "3",
                offset: 0,
            }),
        ]),
    });

    println!("{:#?}", users.get("TEST").unwrap().users.get("client1").unwrap().token);
}


