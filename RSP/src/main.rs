use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use rand::Rng;
use serde::Deserialize;
use std::cmp::Ordering;
use std::cmp::{Ord, PartialOrd};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct QueryParams {
    choice: String,
}

async fn play_game(event: Request) -> Result<impl IntoResponse, lambda_http::Error> {
    // Parse query parameters
    let query_params: QueryParams = match event.query_string_parameters() {
        Some(params) => serde_urlencoded::from_str(params)?,
        None => return Ok("Invalid choice. Choose rock, paper, or scissors.".into_response()),
    };

    let player_choice = match Choice::from_str(&query_params.choice) {
        Ok(choice) => choice,
        Err(_) => return Ok("Invalid choice. Choose rock, paper, or scissors.".into_response()),
    };

    let computer_choice = rand::thread_rng().gen_range(0..3);
    let computer_choice = match computer_choice {
        0 => Choice::Rock,
        1 => Choice::Paper,
        _ => Choice::Scissors,
    };

    let result = determine_winner(player_choice, computer_choice);
    Ok(result.into_response())
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    lambda::run(handler(function_handler)).await?;
    Ok(())
}
