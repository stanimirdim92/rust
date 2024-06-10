// $users = [
// 'TEST' => [
// 'event' => 'changed',
// 'url' => 'https://svg-create-test.peiq.cloud/adapi/v1/',
//
// 'users' => [
// 'nordkurier_sm_ads' => [
// "token" => '02-83739cccd4ji0dxpyupra6hnbd66~v*zszp4bum9umeyshgr7s4r3p3e8dyktqsvl',
// "offset" =>80282
// ],
// 'schwaebische_rpa' => [
// "token" =>  '02kcbo8ly5s$1h6d*$lnd$-p-9jdv2mg71w$g*y3$c_9xkdltxv2j6j~2en6xcl33svl',
// "offset" =>80282
// ],
// //SVZ not active atm
// 'transmatico_zvs_adapi' => [
// "token" => '02ycp7m4*t6m3yaw83rjhmjf$x69w3rjqry7kug1a-f-$qkb56s1tszmsaqs4nkr$svl',
// "offset" => 0
// ],
// ],
// ],
// 'PROD' => [
// 'event' => 'production',
// 'url' => 'https://svg-create.peiq.cloud/adapi/v1/',
// 'users' => [
// 'nordkurier_sm_ads' => [
// "token" => '02t9m3~y$7tm$h*-oat80chho$r**ro3z7nd8xve22vfuj-0azoefwwp6*_rurkprsvl',
// "offset" =>80282
// ],
// 'schwaebische_rpa' => [
// "token" => '028tykxhlfwolu7_zvlcpndjy*prtboyoab42-5zss$5kvqf7*e3lb9ex4e4bul0wsvl',
// "offset" =>80282
// ],
// //SVZ not active atm
// 'transmatico_zvs_adapi' => [
// "token" => '02ycp7m4*t6m3yaw83rjhmjf$x69w3rjqry7kug1a-f-$qkb56s1tszmsaqs4nkr$svl',
// "offset" => 0
// ],
// ],
// ],
// ];


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


