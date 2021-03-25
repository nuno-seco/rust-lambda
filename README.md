# Description
This repo was used to deploy a Rust Binary with all dependencies statically linked, packaged as a Docker container, as AWS Lambda function.

The following links were used as references:
https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/
https://hub.docker.com/r/lambci/lambda
https://adevait.com/rust/deploying-rust-functions-on-aws-lambda
https://github.com/awslabs/aws-lambda-rust-runtime

`sudo ln -s musl-gcc x86_64-linux-musl-gcc`

## Testing the Binary Locally

### Launch local Lambda runtime:

```
sudo docker run --rm -e DOCKER_LAMBDA_STAY_OPEN=1 -p 9001:9001 -v "$PWD"/target/x86_64-unknown-linux-musl/release/bootstrap:/var/task/bootstrap:ro,delegated -v "$PWD"/tmp/cenas:/cenas  lambci/lambda:provided main
```

### Send request Locally
```
aws lambda invoke --endpoint http://localhost:9001 --no-sign-request --function-name dummy --payload '{"firstName": "Nuno"}' response.json && cat response.json
```


We need the CDK in Rust
Laterst version of lambda runtime does not work!!

export API_SERVER=

POST Guess
curl -H "Content-Type: application/json" -X POST -d '{"guess": 11}' https://"$API_SERVER".execute-api.eu-north-1.amazonaws.com/prod/games/58bdd4bd-bcde-4b95-86f5-6b0f637fd480

curl -H "Accept: application/json" https://"$API_SERVER".execute-api.eu-north-1.amazonaws.com/prod/games/58bdd4bd-bcde-4b95-86f5-6b0f637fd480

curl -H "Content-Type: application/json" -X POST https://"$API_SERVER".execute-api.eu-north-1.amazonaws.com/prod/games/
