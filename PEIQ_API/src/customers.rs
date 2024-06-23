use std::collections::HashMap;

pub(crate) mod nordkurier_sm_ads;
pub(crate) mod schwaebische_rpa;

#[derive(Debug)]
pub(crate) struct UserDetails<'a> {
    token: &'a str,
    offset: i32
}

#[derive(Debug)]
pub(crate) struct UserConfig<'a> {
    event: &'a str,
    url: &'a str,
    users: HashMap<&'a str, UserDetails<'a>>
}


fn users_test() -> HashMap<&'static str, UserDetails<'static>> {
    let nordkurier_sm_ads: UserDetails = UserDetails {
        token: "02-83739cccd4ji0dxpyupra6hnbd66~v*zszp4bum9umeyshgr7s4r3p3e8dyktqsvl",
        offset: 0
    };

    let schwaebische_rpa: UserDetails = UserDetails {
        token: "02kcbo8ly5s$1h6d*$lnd$-p-9jdv2mg71w$g*y3$c_9xkdltxv2j6j~2en6xcl33svl",
        offset: 0
    };

    let transmatico_zvs_adapi: UserDetails = UserDetails {
        token: "02ycp7m4*t6m3yaw83rjhmjf$x69w3rjqry7kug1a-f-$qkb56s1tszmsaqs4nkr$svl",
        offset: 0
    };

    HashMap::from([
        ("nordkurier_sm_ads", nordkurier_sm_ads),
        ("schwaebische_rpa", schwaebische_rpa),
        ("transmatico_zvs_adapi", transmatico_zvs_adapi)
    ])
}

fn users_prod() -> HashMap<&'static str, UserDetails<'static>> {
    let nordkurier_sm_ads: UserDetails = UserDetails {
        token: "02t9m3~y$7tm$h*-oat80chho$r**ro3z7nd8xve22vfuj-0azoefwwp6*_rurkprsvl",
        offset: 0
    };

    let schwaebische_rpa: UserDetails = UserDetails {
        token: "028tykxhlfwolu7_zvlcpndjy*prtboyoab42-5zss$5kvqf7*e3lb9ex4e4bul0wsvl",
        offset: 0
    };

    let transmatico_zvs_adapi: UserDetails = UserDetails {
        token: "02ycp7m4*t6m3yaw83rjhmjf$x69w3rjqry7kug1a-f-$qkb56s1tszmsaqs4nkr$svl",
        offset: 0
    };

    HashMap::from([
        ("nordkurier_sm_ads", nordkurier_sm_ads),
        ("schwaebische_rpa", schwaebische_rpa),
        ("transmatico_zvs_adapi", transmatico_zvs_adapi)
    ])
}

pub(crate) fn users() -> HashMap<&'static str, UserConfig<'static>> {
    let mut users_config: HashMap<&str, UserConfig> = HashMap::new();

    users_config.insert("TEST", UserConfig {
        event: "changed",
        url: "https://svg-create-test.peiq.cloud/adapi/v1/",
        users: users_test()
    });
    users_config.insert("PROD", UserConfig {
        event: "production",
        url: "https://svg-create.peiq.cloud/adapi/v1/",
        users: users_prod()
    });

    users_config
}

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