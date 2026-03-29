pub const API_BASE: &str = match option_env!("API_GATEWAY_URL") {
    Some(v) => v,
    None    => "",
};
