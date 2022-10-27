use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected snowflake option")
        .resolved
        .as_ref()
        .expect("Expected snowflake");

    if let CommandDataOptionValue::String(snowflake) = option {
        if !snowflake.parse::<i64>().is_ok() {
            return "That doesn't look like a snowflake. Snowflakes contain only numbers."
                .to_string();
        }
        let snowflake_parsed = snowflake.parse::<i64>().unwrap();

        if snowflake_parsed < 4194304 {
            return "That doesn't look like a snowflake. Snowflakes are much larger numbers."
                .to_string();
        }
        format!(
            "<t:{}>",
            (snowflake_parsed / 4194304 + 1420070400000) / 1000
        )
    } else {
        "Please provide a valid snowflake".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leet")
        .description("Get a timestamp for snowflake")
        .create_option(|option| {
            option
                .name("snowflake")
                .description("The snowflake (id) of the message")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
