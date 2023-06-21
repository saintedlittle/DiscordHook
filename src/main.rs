mod service;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use discord_rich_presence::activity::{Assets, Button, Party};
use crate::service::start_as_hidden_service;

/*
    ====================
    VALUES FOR CHANGE!!!
    ====================

    @url: https://discord.com/developers/applications
 */

/*
    SERVICE INFORMATION
 */

#[cfg(windows)]
const SERVICE_FOLDER: &str = "APPDATA";
#[cfg(windows)]
const PATH_TO_SERVICE : &str = "Microsoft\\Windows\\Start Menu\\Programs\\Startup";

const CLIENT_ID: &str = "1086700430882504725";
const DETAILS: &str = "Анастасия Пилипчук, 24 годика.";
const STATE: &str = "Ищу себе дед-инсайдика,\nчтобы пиздил сковородкой.";

fn set_assets() -> Assets<'static> {
    let mut _assets = Assets::new();

    return _assets.clone().
        large_image("default")
        .large_text("Є таке місце на нашій землі")
        .small_image("ukraine")
        .small_text("Воно стане цвинтарем для всієї русні");

}

fn set_buttons() -> Vec<Button<'static>> {
    let mut _buttons : Vec<Button> = Vec::<Button>::new();

    _buttons.push(
        Button::new("Website", "http://saintedlittle.nl")
    );
    _buttons.push(
        Button::new("Telegram", "https://t.me/norfitpine")
    );

    return _buttons;
}

fn set_party() -> Party<'static> {
    let mut _party = Party::new();

    return _party.clone().size([2, 2]);
}

/*
    ====================
            END
    ====================
 */

fn discord_loop(mut client: DiscordIpcClient) {

    let _activity = activity::Activity::new();

    client.connect().ok().expect("Panic message!");
    client.set_activity(activity::Activity::new()
        .state(STATE)
        .details(DETAILS)
        .assets(set_assets())
        .party(set_party())
        .buttons(set_buttons())
    ).expect("Panic message!");

    println!("Connected!");
    loop {}
}

fn main() {
    let client = DiscordIpcClient::new(CLIENT_ID).expect("Panic message!");

    #[cfg(windows)]
    start_as_hidden_service(String::from(SERVICE_FOLDER), String::from(PATH_TO_SERVICE)).expect("panic message!");

    discord_loop(client);
}

