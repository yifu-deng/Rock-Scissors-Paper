# Rock, Paper, Scissors Serverless Game

[![Build Status](https://travis-ci.org/awslabs/aws-lambda-rust-runtime.svg?branch=master)](https://travis-ci.org/awslabs/aws-lambda-rust-runtime)

This is a simple implementation of a Rock-Paper-Scissors game using Rust and AWS Lambda. It uses the lambda_http crate for handling HTTP requests and responses, the actix_web crate for routing, and the rand crate for generating random computer choices.

## Prerequisites

- Rust installed on your machine
- AWS account with proper IAM roles and policies set up
- AWS CLI installed and configured

## How to Run

- Clone the repository to your local machine.
- Navigate to the project directory and run cargo build --release to build the binary.
- Create a new Lambda function in AWS and upload the binary in the .aws-sam/build directory.
- Set the runtime to custom and the handler to lambda_handler.
- Set up an API Gateway to route requests to the Lambda function.
- Test the function using your API Gateway endpoint.

## How to Play

- The game can be played by making a GET request to the API Gateway endpoint with a query parameter choice set to rock, paper, or scissors. The Lambda function will randomly generate a choice for the computer and determine the winner based on the rules of Rock-Paper-Scissors.

## Code Explanation

- QueryParams: This is a struct that is deserialized from the query string parameters of the HTTP request. It contains a single field choice, which represents the player's choice.
- play_game: This is the main function that handles the game logic. It parses the player's choice from the query string, generates a random choice for the computer, determines the winner, and returns the result.
- main: This is the entry point of the program. It uses the lambda::run function to start the Lambda handler and bind it to a specific function.

## References

- [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
