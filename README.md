# Description

In this Repo you can find an implementation of a guessing game inpired from chapter 2 of the [Rust Book](https://doc.rust-lang.org/book/).
There are several other aspects explored in this repo that are not necessarily related to rust. 
In particular, other than rust programming per se, the project also explores:
- packaging a statically linked rust binary as a Docker image and deploying it to [AWS Lambda](https://aws.amazon.com/lambda/)
- uses an [API Gateway](https://aws.amazon.com/api-gateway/) instance in a "Lambda Integration" (not a Lambda Proxy) which by its nature implies use of VTL templates to bridge the Lambda events into REST like methods
- use the AWS CDK to provision and update the AWS stack with updates
- running and testing a lammbda function locally by running it in a docker container
- with the new billing unit for [AWS Lambda at 1ms](https://aws.amazon.com/blogs/aws/new-for-aws-lambda-1ms-billing-granularity-adds-cost-savings/) it begs one to consider lowering the latency as much as possible when dealing with high volume systems. How far can Rust take us? And what is the tradeoff with developer productivity?

## Caveats
The [Rust Lambda Runtime](https://github.com/awslabs/aws-lambda-rust-runtime) used in the project is not the most recent as that version caused the lambda function to hang in certain situations. The version used, and as far as I could understand, has no good way of transforming custom `Error` types to the types the library requires which lead to some ugly code in the main function. 

The state of the lambda is kept in a in-memory HashMap, hence when the lambda function is disposed so is all the state of your ongoing games. Likewise, if several Lambdas are spawned you won't be able to control where requests are routed and you'll experience "split brain" like scenario. Phantom reads are also quite likely if there is high concurrency on the same game. Depending on your view point this could actually make the game more exciting.... A following iteration will include persistent storage most likely using [DynamoDB](https://aws.amazon.com/dynamodb/). 

Tests are scarce and require furher knowledge of the existing Rust mocking frameworks and best practices.

In this project it is assumed that the rust binary produced is statically linked to [Musl](https://www.musl-libc.org/) hence producing a standalone binary. After adding the `x86_64-unknown-linux-musl` target to the Rust toolchain `Cargo` failed to find  the linker to the `Musl` target in question. The solution to make it work was to create a symbolic link using `sudo ln -s musl-gcc x86_64-linux-musl-gcc` in the `usr/bin` dir (note that this was on a Ubuntu Linux machine with [musl](https://packages.ubuntu.com/search?keywords=musl))

## Setup

### Requirements
- Install the latest version of Rust. See the [Rust Site](https://www.rust-lang.org/) for instructions. (Make sure the Cargo is also installed)
- Install the AWS CDK. In this repo the [Python version](https://docs.aws.amazon.com/cdk/latest/guide/work-with-cdk-python.html) was used. 
- Install [Docker](https://docs.docker.com/get-started/)
- Make sure you have an AWS account and that you have setup the [AWS cli](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-quickstart.html) and can "talk" to your AWS account.  

### Building 
The repo contains a `Makefile` that can be used to build, test and deploy the lambda function.
If you can run the command `make cargo_build` then you have your Rust environment setup correctly.
That command should produce a statically linked binary that is placed in the `docker` folder.

## Running the Lambda Function Locally
After building in the previous step you can locally launch the lambda function by issuing the command `make lambda_local_run`. 
You should notice a docker container start up and with it that has loaded the compiled binary in a Lambda like enviroment. 
If the container has successfully started then you can run the following command to create a new session of the guessing game:
```
aws lambda invoke --endpoint http://localhost:9001 --no-sign-request --function-name dummy --payload '{"kind": "gameRequested"}' response.json && cat response.json
```
If everything is running as expected you should see a response similar to:
```
{
    "StatusCode": 200,
    "ExecutedVersion": "$LATEST"
}
{"kind":"GameCreated","id":"4ce7c7b8-02bc-40a4-823b-6c7c02b028d3","guesses":[null,null,null],"random_number":7,"status":"ongoing"}

```

Basically, what has been done in the previous command was to initiate a new guessing game session. The response indicates that the game has been created with the id `4ce7c7b8-02bc-40a4-823b-6c7c02b028d3`, the random_number that is to be guessed by the user is `7`, the status of the game is `ongoing` and no guesses `[null,null,null]` have been made yet.
> NOTE: In the output above the number to guess is being shown. However, when the game is exposed via the REST like API configured in the API gateway the number is never exposed.

To make a guess you can issue the following command:
```
 aws lambda invoke --endpoint http://localhost:9001 --no-sign-request --function-name dummy --payload '{"kind": "guessSubmitted", "guess": 5, "id": "4ce7c7b8-02bc-40a4-823b-6c7c02b028d3"}' response.json && cat response.json
```
Which will result in the following output:
```
{
    "StatusCode": 200,
    "ExecutedVersion": "$LATEST"
}
{"kind":"GuessEvaluated","id":"4ce7c7b8-02bc-40a4-823b-6c7c02b028d3","guesses":[5,null,null],"random_number":7,"status":"ongoing"}
```
The response indicates that the guess (in this case `5`) has been evaluted, the list of guesses has been updated and all other fields remain the same.

Let's now provide the correct number, in this case `7`:
```
aws lambda invoke --endpoint http://localhost:9001 --no-sign-request --function-name dummy --payload '{"kind": "guessSubmitted", "guess": 7, "id": "4ce7c7b8-02bc-40a4-823b-6c7c02b028d3"}' response.json && cat response.json
```
The response now indicates that the game has been `won`:
```
{
    "StatusCode": 200,
    "ExecutedVersion": "$LATEST"
}
{"kind":"GameWon","id":"4ce7c7b8-02bc-40a4-823b-6c7c02b028d3","guesses":[5,7,null],"random_number":7,"status":"won"}
```
Finally, an existing event exists that allows one to just retrive the state of the current game:
```
aws lambda invoke --endpoint http://localhost:9001 --no-sign-request --function-name dummy --payload '{"kind": "gameInfoRequested", "id": "4ce7c7b8-02bc-40a4-823b-6c7c02b028d3"}' response.json && cat response.json
```
which will return the current state of the game:
```
{
    "StatusCode": 200,
    "ExecutedVersion": "$LATEST"
}
{"kind":"GameInfoProvided","id":"4ce7c7b8-02bc-40a4-823b-6c7c02b028d3","guesses":[5,7,null],"random_number":7,"status":"won"}
```

## Deploying to AWS
The `Makefile` also includes a target that will deploy to your AWS account, in this case use `make cdk_deploy`.
If your CDK is configured correctly you will be prompted to proceed with creating a Cloudformation Stack called `InfraStack`. 
The stack will comprise an [API Gateway](https://aws.amazon.com/api-gateway/) instance, an [AWS ECR](https://aws.amazon.com/ecr/) where the docker image will be uploaded to and a [Lambda function](https://aws.amazon.com/lambda/) that will instantiate the docker image.

Everytime you change something in the code or with the AWS infrastructure you can run `make cdk_deploy` and your stack will be updated with the new version.

### Test the REST like API
Assuming the deployment was successful you should now be able to shoot `curl` requests at the API.
The output from the CDK command above should have included the URL of the configured API Gateway instance; something like:
```
https://05bad69fcc.execute-api.eu-north-1.amazonaws.com/prod
```

With the endpoint we can now make requests to it. Following the pattern above let's first create a new game:
```
curl -H "Content-Type: application/json" -X POST  https://05bad69fcc.execute-api.eu-north-1.amazonaws.com/prod/games/
```

Just like above we get an `id` for the new game along with some more information indicating what endpoints are available:
```
{
  "game":
   {
     "id" : "abe63716-f7c0-4d70-9383-db6ed4dad15d",
     "guesses" : [null,null,null],
     "status" : "ongoing"
   },
  "links":
   {                                                                            
     "status": 
      {
        "method": "GET",
        "link": "/prod/games/abe63716-f7c0-4d70-9383-db6ed4dad15d"
      },
      "guess":
       {
         "method": "POST",
         "link": "/prod/games/abe63716-f7c0-4d70-9383-db6ed4dad15d",
         "body": 
          {
            "type": "object",
            "properties": 
             {
                "guess": 
                 {
                   "type": "integer",
                   "minimum": 0,
                   "maximum": 10
                 }
             }   
          }    
        }
    }
}
```
> NOTE: The extra information in the reponse is generated using VTL templates in the API gateway. The lambda per se is decoupled from the HTTP interface and just generates intgeration agnostic events. 

Knowing the `id` of the game we can now submit a guess:
```
curl -H "Content-Type: application/json" -X POST -d '{"guess": 5}'  https://05bad69fcc.execute-api.eu-north-1.amazonaws.com/prod/games/abe63716-f7c0-4d70-9383-db6ed4dad15d
```
and the response will be:
```
{
  "game":
   {
     "id" : "abe63716-f7c0-4d70-9383-db6ed4dad15d",
     "guesses" : [5,null,null],
     "status" : "ongoing"
   },
  "links":
   {                                                                            
     "status": 
      {
        "method": "GET",
        "link": "/prod/games/abe63716-f7c0-4d70-9383-db6ed4dad15d"
      },
      "guess":
       {
         "method": "POST",
         "link": "/prod/games/abe63716-f7c0-4d70-9383-db6ed4dad15d",
         "body": 
          {
            "type": "object",
            "properties": 
             {
                "guess": 
                 {
                   "type": "integer",
                   "minimum": 0,
                   "maximum": 10
                 }
             }   
          }    
        }
    }
}
```
As can be seen the guess has been added to the list of guesses and since we did not guess the right number the game is still `ongoing`.

Finally to just get the state of the game one can issue the following request:
```
curl -H "Content-Type: application/json" -X GET  https://05bad69fcc.execute-api.eu-north-1.amazonaws.com/prod/games/abe63716-f7c0-4d70-9383-db6ed4dad15d
```
the response will be exactly the same as the previous response.

After submitting the last guess you can see that the status of the game has changed to `lost`:
```
{
  "game":
   {
     "id" : "abe63716-f7c0-4d70-9383-db6ed4dad15d",
     "guesses" : [5,3,9],
     "status" : "lost"
   },
  "links":
   {                                                                            
     "status": 
      {
        "method": "GET",
        "link": "/prod/games/abe63716-f7c0-4d70-9383-db6ed4dad15d"
      },
      "guess":
       {
         "method": "POST",
         "link": "/prod/games/abe63716-f7c0-4d70-9383-db6ed4dad15d",
         "body": 
          {
            "type": "object",
            "properties": 
             {
                "guess": 
                 {
                   "type": "integer",
                   "minimum": 0,
                   "maximum": 10
                 }
             }   
          }    
        }
    }
}
```

The are also error responses for requests using `id`s that are unknown or submitting guesses to games that have already finished. 
For example submitting a new guess to a game that is finished (either `won` or `lost`) will result in the following response:
```
{"errorMessage":"UnknownError: UnknownError: Game Already Finished","errorType":"UnknownError","stackTrace":null}
```
> NOTE: The error message is pretty screwed up with `UnknownError` being shown multiple times. I believe this is  a bug in the version of the lambda runtime being used. The newer version handles errors differently and uses different crates hopefully making error handling easier.  


## Disclaimer
Nothing in this repo should be considered a reference as it is just one persons attempt in exploring tech using happy path programming.   


# Other References
- https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/
- https://hub.docker.com/r/lambci/lambda
- https://adevait.com/rust/deploying-rust-functions-on-aws-lambda
- https://github.com/awslabs/aws-lambda-rust-runtime
- https://docs.aws.amazon.com/cdk/latest/guide/work-with-cdk-python.html
