use super::welcome_screen_channel::WelcomeScreenChannel;

#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomeScreen {
    description: Option<String>,
    welcome_channels: Option<Vec<WelcomeScreenChannel>>,
}
