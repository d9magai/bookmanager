use strum_macros::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    Development,
    Production,
}

pub fn which() -> Environment {
    #[cfg(debug_assertions)]
    let default_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;

    match std::env::var("ENV") {
        Ok(env) => match env.as_str() {
            "development" => Environment::Development,
            "production" => Environment::Production,
            _ => default_env, // Use default_env if not recognized
        },
        Err(_) => default_env, // Use default_env if ENV variable is not set
    }
}
