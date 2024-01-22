use crate::utils::minecraft::get_player_uuid;
use crate::{Context, Error};

use tracing::error;


#[poise::command(slash_command, prefix_command)]
pub async fn uuid(
    ctx: Context<'_>,
    #[description = "Minecraft username"] name: String
) -> Result<(), Error> {
    let uuid = match get_player_uuid::uuid(&name).await {
        Ok(uuid) => uuid,
        Err(e) => {
            error!("Cannot get player {}'s uuid: {}", name, e);
            return Err(e);
        }
    };

    ctx.reply(format!("{}'s uuid is: {:?}", name, uuid)).await?;

    Ok(())
}