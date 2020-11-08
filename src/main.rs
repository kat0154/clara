use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready},
  prelude::*,
};
use serenity::model::prelude::Activity;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        _ctx.set_activity(Activity::listening("my master")).await;
        _ctx.dnd().await;
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
      let mut m = msg.content.splitn(2, ' ');
      let command = m.next();
      let args = m.next();
      if msg.content == "c~ping" {
        if let Err(why) = msg.channel_id.say(&ctx.http, "pong?").await {
          println!("Error sending message: {:?}", why);
        }
      }
      if msg.content == "c~avatar" {
	  let avatar_url = msg.author.face();
          if let Err(why) = msg.channel_id.send_message(&ctx.http, |cm| cm.embed(|ce| 
                ce.title(format!("Here's your avatar {}", msg.author.name))
                .color(0x00d6ff)
		.image(&avatar_url)
            )).await {
            println!("Error sending message: {:?}", why);
          }
      }
      if let (Some("c~say"), Some(me)) = (command, args) {
        if let Err(y) = msg.delete(&ctx.http).await {
          println!("Error sending message: {:?}", y);
        }
        if let Err(why) = msg.channel_id.say(&ctx.http, me).await {
          println!("Error sending message: {:?}", why);
        }
      }
    }
}

#[tokio::main]
  async fn main() {
    let token: &str = "botto-token";

    let mut client = Client::builder(&token)
      .event_handler(Handler)
      .await
      .expect("Err creating client");

      if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
      }
  }
