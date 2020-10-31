use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready},
  prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
      if msg.content == "clara, ping" {
        if let Err(why) = msg.channel_id.say(&ctx.http, "pong?").await {
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
