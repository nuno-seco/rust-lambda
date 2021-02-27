# Description
This repo was used to deploy a Rust Binary with all dependencies statically linked, packaged as a Docker container, as AWS Lambda function.

The following links were used as references:
https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/
https://hub.docker.com/r/lambci/lambda
https://adevait.com/rust/deploying-rust-functions-on-aws-lambda
https://github.com/awslabs/aws-lambda-rust-runtime

## Testing the Binary Locally

### Launch local Lambda runtime:

```
docker run --rm -e DOCKER_LAMBDA_STAY_OPEN=1 -p 9001:9001 -v "$PWD"/x86_64-unknown-linux-musl/release/bootstrap:/var/task/bootstrap:ro,delegated lambci/lambda:provided main
```

### Send request Locally
```
aws lambda invoke --endpoint http://localhost:9001 --no-sign-request --function-name rust-lambda-test --payload $(echo '{"firstName": "Nuno"}' | base64) response.json && cat response.json
```
