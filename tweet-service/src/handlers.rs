use crate::models::Tweet;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref TWEETS: Mutex<Vec<Tweet>> = Mutex::new(vec![]);
}

pub async fn post_tweet(new_tweet: web::Json<Tweet>) -> impl Responder {
    let mut tweets = TWEETS.lock().unwrap();
    tweets.push(new_tweet.into_inner());
    // In a complete implementation, publish a "NewTweet" event to a message queue.
    HttpResponse::Ok().body("Tweet posted")
}

pub async fn get_tweets() -> impl Responder {
    let tweets = TWEETS.lock().unwrap();
    HttpResponse::Ok().json(tweets.clone())
}
