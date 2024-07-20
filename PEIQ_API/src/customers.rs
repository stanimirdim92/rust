use std::collections::HashMap;

pub(crate) mod nordkurier_sm_ads;
pub(crate) mod schwaebische_rpa;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct UserDetails {
    pub(crate) token: String,
    offset: i32
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct UserConfig {
    event: String,
    url: String,
    pub(crate) users: HashMap<String, UserDetails>
}


#[allow(dead_code)]
fn users_test() -> HashMap<String, UserDetails> {
    let nordkurier_sm_ads: UserDetails = UserDetails {
        token: "02-83739cccd4ji0dxpyupra6hnbd66~v*zszp4bum9umeyshgr7s4r3p3e8dyktqsvl".to_string(),
        offset: 0
    };

    let schwaebische_rpa: UserDetails = UserDetails {
        token: "02kcbo8ly5s$1h6d*$lnd$-p-9jdv2mg71w$g*y3$c_9xkdltxv2j6j~2en6xcl33svl".parse().unwrap(),
        offset: 0
    };

    let transmatico_zvs_adapi: UserDetails = UserDetails {
        token: "02ycp7m4*t6m3yaw83rjhmjf$x69w3rjqry7kug1a-f-$qkb56s1tszmsaqs4nkr$svl".parse().unwrap(),
        offset: 0
    };

    HashMap::from([
        ("nordkurier_sm_ads".to_string(), nordkurier_sm_ads),
        ("schwaebische_rpa".to_string(), schwaebische_rpa),
        ("transmatico_zvs_adapi".to_string(), transmatico_zvs_adapi)
    ])
}

fn users_prod() -> HashMap<String, UserDetails> {
    let nordkurier_sm_ads: UserDetails = UserDetails {
        token: "02t9m3~y$7tm$h*-oat80chho$r**ro3z7nd8xve22vfuj-0azoefwwp6*_rurkprsvl".to_string(),
        offset: 0
    };

    let schwaebische_rpa: UserDetails = UserDetails {
        token: "028tykxhlfwolu7_zvlcpndjy*prtboyoab42-5zss$5kvqf7*e3lb9ex4e4bul0wsvl".to_string(),
        offset: 0
    };

    let transmatico_zvs_adapi: UserDetails = UserDetails {
        token: "02ycp7m4*t6m3yaw83rjhmjf$x69w3rjqry7kug1a-f-$qkb56s1tszmsaqs4nkr$svl".to_string(),
        offset: 0
    };

    HashMap::from([
        ("nordkurier_sm_ads".to_string(), nordkurier_sm_ads),
        ("schwaebische_rpa".to_string(), schwaebische_rpa),
        ("transmatico_zvs_adapi".to_string(), transmatico_zvs_adapi)
    ])
}


pub(crate) fn users() -> HashMap<String, UserConfig> {
    let mut users_config: HashMap<String, UserConfig> = HashMap::new();

    users_config.insert("TEST".to_string(), UserConfig {
        event: "changed".to_string(),
        url: "https://svg-create-test.peiq.cloud/adapi/v1/".to_string(),
        users: users_test()
    });
    users_config.insert("PROD".to_string(), UserConfig {
        event: "production".to_string(),
        url: "https://svg-create.peiq.cloud/adapi/v1/".to_string(),
        users: users_prod()
    });

    users_config
}

#[allow(dead_code)]
pub(crate) fn ad_is_ready_for_transfer(ads_info: &mut serde_json::Value) -> bool {

    if ads_info.get("online_publish").is_none() {
        println!("No online_publish found. Error: {}", ads_info.get("error").unwrap());
        return false
    }

    return match user() {
        "nordkurier_sm_ads" => nordkurier_sm_ads::ad_is_ready_for_transfer(ads_info),
        "schwaebische_rpa" => schwaebische_rpa::ad_is_ready_for_transfer(ads_info),
        // "transmatico_zvs_adapi" => transmatico_zvs_adapi_ad_is_ready_for_transfer(ads_info),
        other   => {
            println!("Unknown user: {}", other);
            false
        }
    };
}

//TODO return user from console argument
fn user() -> &'static str {
    return "nordkurier_sm_ads";
}