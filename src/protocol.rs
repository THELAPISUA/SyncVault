use crate::{
    config::Config,
    state::User,
    utils::{get_file_data, get_keys},
};

fn login(apikey: &str, config: &Config) -> bool {
    let keys = get_keys(&config.keys);
    keys.contains(&apikey.to_string())
}

pub fn protocol_checker(buffer: &[u8], user: &mut User, config: Config) -> String {
    let data_string = std::str::from_utf8(buffer);
    match data_string {
        Ok(data) => data,
        Err(_e) => "INPUT_ERROR",
    };

    let argv: Vec<&str> = data_string.unwrap().split(" ").collect();

    let mut respond = String::new();
    match argv[0].trim() {
        "LOGIN" => {
            if argv.len() > 1 {
                let login = login(argv[1].trim(), &config);
                user.is_logged_in = login;
                if login {
                    respond.push_str(format!("Successful login to {}", &config.name).as_str());
                } else {
                    respond.push_str(format!("Failed login to {}", &config.name).as_str());
                }
            }
        }
        "CHECK" if user.is_logged_in => {
            respond.push_str(&config.version);
        }
        "GETUPDATE" if user.is_logged_in => {
            respond.push_str(get_file_data(&config.data).as_str());
        }

        "INPUT_ERROR" => {
            respond.push_str(get_file_data("INPUT_ERROR").as_str());
        }

        _ => {}
    }
    respond
}
