use super::getenv;
use crate::config::Config;
use crate::rbx::Message;

pub struct EventParams {
    pub branch_name: Option<String>,
    pub topic: Option<String>,
    pub message: Option<String>,
    pub api_key: Option<String>,
}

pub async fn event(params: &EventParams) -> anyhow::Result<Option<String>> {
    let api_key = getenv(params.api_key.clone(), "OPENCLOUD_KEY".to_string());
    let branch = match params.branch_name.clone() {
        Some(v) => v,
        None => "main".to_string(),
    };
    let config = Config::new(branch);
    let universe_id = config.get_universe_id().unwrap();

    let message = Message::new(&api_key, universe_id);
    message
        .publish(
            &params.topic.clone().unwrap(),
            &params.message.clone().unwrap(),
        )
        .await;
    Ok(None)
}
