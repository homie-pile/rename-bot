use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::client::{Context, EventHandler};
use serenity::model::interactions::application_command::{
    ApplicationCommand, ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::model::prelude::Ready;
use tracing::info;

/// The struct that handles all discord events
pub struct Handler;

// The handler implementation
#[async_trait]
impl EventHandler for Handler {
    // When the bot is ready
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Register the rename command
        ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("rename")
                    .description("Rename another user")
                    .create_option(|option| {
                        option
                            .name("user")
                            .description("The user to rename")
                            .required(true)
                            .kind(ApplicationCommandOptionType::User)
                    })
                    .create_option(|option| {
                        option
                            .name("name")
                            .description("A new name to set")
                            .required(true)
                            .kind(ApplicationCommandOptionType::String)
                    })
            })
        })
        .await
        .unwrap();
    }

    /// Handle interaction events
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response: (String, Option<CreateEmbed>) = match command.data.name.as_str() {
                // Handle the rename command
                "rename" => {
                    // Ensure there is a user option
                    if let Some(option) = command.data.options.get(0) {
                        // Ensure the resolver worked correctly
                        if let Some(ApplicationCommandInteractionDataOptionValue::User(
                            user,
                            member,
                        )) = &option.resolved
                        {
                            // Ensure there is a name option
                            if let Some(option) = command.data.options.get(1) {
                                // Resolve the string
                                if let Some(ApplicationCommandInteractionDataOptionValue::String(
                                    name,
                                )) = &option.resolved
                                {
                                    // Ensure the name is a valid length for discord
                                    if name.len() <= 32 {
                                        info!("Renaming {} to {}", user.name, name);

                                        // Rename the user
                                        let original_name = member
                                            .as_ref()
                                            .map(|m| {
                                                m.nick
                                                    .as_ref()
                                                    .unwrap_or(&"unknown".to_string())
                                                    .clone()
                                            })
                                            .unwrap_or("unknown".to_string());
                                        if let Some(member) = member {
                                            let member = ctx
                                                .http
                                                .get_member(command.guild_id.unwrap().0, user.id.0)
                                                .await
                                                .unwrap();
                                            member.edit(&ctx, |m| m.nickname(name)).await.unwrap();
                                        }

                                        // Return a success message
                                        let embed = CreateEmbed::default()
                                            .title("Nickname Changed")
                                            .description(format!(
                                                "Changed <@{}>'s name from `{}` to `{}`",
                                                user.id.0, original_name, name
                                            ))
                                            .author(|a| {
                                                a.name(command.user.name.clone()).icon_url(
                                                    command.user.avatar_url().unwrap_or_default(),
                                                )
                                            })
                                            .to_owned();
                                        (String::new(), Some(embed))
                                    } else {
                                        ("That name is too long".to_string(), None)
                                    }
                                } else {
                                    (
                                        "Could not parse new name string from command".to_string(),
                                        None,
                                    )
                                }
                            } else {
                                ("Could not extract new name from command".to_string(), None)
                            }
                        } else {
                            ("Could not parse user id from command".to_string(), None)
                        }
                    } else {
                        ("Could not extract user from command".to_string(), None)
                    }
                }
                _ => ("Unknown command".to_string(), None),
            };

            // Send the response
            command
                .create_interaction_response(&ctx.http, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            let mut m = message.content(response.0);
                            if let Some(embed) = response.1 {
                                m = m.add_embed(embed);
                            }
                            m
                        })
                })
                .await
                .unwrap();
        }
    }
}
