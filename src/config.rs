/// URL de base de l'API Gateway (sans slash final).
/// À définir dans les variables d'env Amplify + Vercel à la build.
/// Exemple : "https://xxxxxxxxxx.execute-api.eu-west-3.amazonaws.com/prod"
pub const API_BASE: &str = match option_env!("API_GATEWAY_URL") {
    Some(v) => v,
    None    => "",
};
