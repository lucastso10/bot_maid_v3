use std::{env, error::Error, fmt::Debug, sync::Arc};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Event, Shard, ShardId, Config, EventTypeFlags};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;
use twilight_model::application::command::CommandType;
use twilight_util::builder::command::{BooleanBuilder, CommandBuilder, StringBuilder};
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};

#[tokio::main]
async fn main(){
    
    let token = env::var("DISCORD_TOKEN").unwrap();

    let event_types = EventTypeFlags::INTERACTION_CREATE;

    let config = Config::builder(token.clone(), Intents::GUILD_MESSAGES)
        .event_types(event_types)
        .build();

    let mut shard = Shard::with_config(ShardId::ONE, config);

    let http = Arc::new(HttpClient::new(token));

    let blep = CommandBuilder::new(
        "blep",
        "Send a random adorable animal photo",
        CommandType::ChatInput,
        )
        .option(
            StringBuilder::new("animal", "The type of animal")
                .required(true)
                .choices([
                    ("Dog", "animal_dog"),
                    ("Cat", "animal_cat"),
                    ("Penguin", "animal_penguin"),
                ]),
        )
        .option(BooleanBuilder::new(
            "only_smol",
            "Whether to show only baby animals",
        ))
        .build();

    let app = http.current_user_application().await.unwrap().model().await.unwrap();

    let interactions_client = http.interaction(app.id);

    let blep = [ blep ];


    for guild in http.current_user_guilds().await.unwrap().model().await.unwrap() {
        println!("I am in {} and their id is {}", guild.name, guild.id);
        if let Err(error) = interactions_client.set_guild_commands(guild.id, &blep).await {
            println!("failed to register commands: {}", error);
        }
    }

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                println!("error receiving event: {}", source);

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tokio::spawn(handle_event(event, Arc::clone(&http)));
    }

}

async fn handle_event(
    event: Event,
    http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!ping" => {
            http.create_message(msg.channel_id)
                .content("Pong!")?
                .await?;
        }
        // Other events here...
        _ => {}
    }

    Ok(())
}
