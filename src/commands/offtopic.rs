use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    println!("{:?}", options);
    String::from("Not implemented")
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("offtopic")
        .description("'Manage offtopic channels")
        .create_option(|option| {
            option
                .name("create")
                .description("Create a offtopic channel")
                .kind(CommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name("archive")
                .description("Archive a offtopic channel")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option| {
                    sub_option
                        .kind(CommandOptionType::Channel)
                        .name("channel")
                        .required(true)
                        .description("Channel to archive")
                })
        })
        .create_option(|option| {
            option
                .name("unarchive")
                .description("Unarchive a offtopic channel")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option| {
                    sub_option
                        .kind(CommandOptionType::Channel)
                        .name("channel")
                        .required(true)
                        .description("Channel to unarchive")
                })
        })
        .create_option(|option| {
            option
                .name("edit")
                .description("Edit a offtopic channel")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|sub_option| {
                    sub_option
                        .kind(CommandOptionType::Channel)
                        .name("channel")
                        .required(true)
                        .description("Channel to edit")
                })
        })
        .create_option(|option| {
            option
                .name("order")
                .description("Order offtopic channels alphabetically")
                .kind(CommandOptionType::SubCommand)
        })
}
