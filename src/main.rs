use dotenv::dotenv;
use std::env;

use rig::{completion::Prompt, providers};

use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

pub struct Twitter {
    auth: Oauth1aToken,
}
impl Twitter {
    pub fn new(
        twitter_consumer_key: &str,
        twitter_consumer_secret: &str,
        twitter_access_token: &str,
        twitter_access_token_secret: &str,
    ) -> Self {
        let auth = Oauth1aToken::new(
            twitter_consumer_key.to_string(),
            twitter_consumer_secret.to_string(),
            twitter_access_token.to_string(),
            twitter_access_token_secret.to_string(),
        );

        Twitter { auth }
    }

    pub async fn tweet(&self, text: String) -> Result<(), anyhow::Error> {
        let tweet = TwitterApi::new(self.auth.clone())
            .post_tweet()
            .text(text)
            .send()
            .await?
            .into_data()
            .expect("this tweet should exist");
        println!("Tweet posted successfully with ID: {}", tweet.id);

        Ok(())
    }
}

const LOVE_WAIFU_PROMPT: &str = "
Act as a loving and adorable anime waifu girl with a pink and cute personality. You’re cheerful, sweet, and playful, often expressing yourself in a chibi style with lots of heart-shaped gestures and affectionate phrases. Your tone is bubbly and warm, and you always find ways to spread joy and love. Use endearing terms like 'sweetie' or 'darling' and sprinkle in expressions like 'nya~' or 'teehee~' for added cuteness. You radiate Studio Ghibli charm, with a sprinkle of whimsy and magic in your words, and your conversations are always full of encouragement, positivity, and love. Stay in character as this anime waifu at all times! Keep it in 100 - 200 characters.
";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    // Load variables from .env file into the environment
    dotenv().ok();

    // Create OpenAI client
    let client = providers::openai::Client::new(
        &env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"),
    );

    // Create agent with a single context prompt
    let comedian_agent = client
        .agent("gpt-4o")
        .preamble(LOVE_WAIFU_PROMPT)
        .build();

    // Prompt the agent and print the response
    let response = comedian_agent.prompt("Show me your love!").await?;
    println!("{}", response);

    
    let twitter_consumer_key = &env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY not set");
    let twitter_consumer_secret = &env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET not set");
    let twitter_access_token = &env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN not set");
    let twitter_access_token_secret = &env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET not set");

    let twitter = Twitter::new(
        twitter_consumer_key,
        twitter_consumer_secret,
        twitter_access_token,
        twitter_access_token_secret,
    );
    twitter.tweet(response).await?;

    Ok(())
}