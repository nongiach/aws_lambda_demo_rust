# aws_lambda_demo_rust
Simple AWS lambda demo with rust

This is just a quick and dirty introduction to AWS lambda with rust
Two goals here:
* build and upload a lambda rust function => ./lambda_test
* invoke a lambda function from rust => ./rust_aws_lambda_client

## step 0: create AWS account
https://console.aws.amazon.com/

## step 1: create a role for the AWS lambda
Create a role that will be used by the aws lambda, this role defines the permissions of your lambda.
By default it requires a AWSLambdaBasicExecutionRole to send logs to cloudtrail.

### create a role using the bellow link
https://console.aws.amazon.com/iamv2/home?#/roles
AWS Service => lambda => next => search for AWSLambdaBasicExecutionRole => next.
Name it lambda-basic-execution and click create.

## step 2: statically compile the binary
We want to statically compile our binary for it to be compatible with the AWS lambda system.
```sh
cargo new lambda_test

sudo apt install musl-tools
rustup target add x86_64-unknown-linux-musl

cargo build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/bootstrap . && zip lambda.zip bootstrap && rm bootstrap
```

## step 3: create the lambda with the role and the rust compiled code
```sh
AWS_ACCOUNT_ID=`aws sts get-caller-identity --query Account --output text` && \
aws lambda create-function \
--function-name rust_lambda \
--runtime provided \
--role arn:aws:iam::$AWS_ACCOUNT_ID:role/lambda-basic-execution \
--zip-file fileb://lambda.zip \
--description "Simple Rust function" \
--timeout 5 \
--handler main
```

## step 4: invoke the function
```sh
aws lambda invoke \
--function-name=rust_lambda \
--invocation-type=RequestResponse \
--payload $(echo '{"fullName": "Martin Luther", "message": null}' | base64 ) \
output.json
```

## step 5: invoke the function from rust
```sh
cd aws_lambda_demo_rust\
cargo run -- -v --arn rust_lambda
```

More here about how to interact with a lambda function from rust
https://docs.rs/aws-sdk-lambda/0.3.0/aws_sdk_lambda/client/struct.Client.html#method.invoke

## Enjoy!

### other tips

#### list available commands
```sh
aws lambda list
```

#### list functions
```sh
aws lambda list-functions
```

#### delete a function
```sh
aws lambda delete-function --function-name rust_lambda
```

#### start a docker with the current function 
```sh
docker run --rm \
-e DOCKER_LAMBDA_STAY_OPEN=1 -p 9001:9001 \
-v "$PWD"/target/x86_64-unknown-linux-musl/release/bootstrap:/var/task/bootstrap:ro,delegated \
lambci/lambda:provided main
```

#### update function code 
```sh
aws lambda update-function-code --function-name rust_lambda --zip-file fileb://lambda.zip
```

#### compiling for amazon v1
```sh
sudo apt install musl-tools
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/bootstrap . && zip lambda.zip bootstrap && rm bootstrap
```

#### upload data to s3 bucket from rust
http://jamesmcm.github.io/blog/2020/04/19/data-engineering-with-rust-and-aws-lambda/


#### ressources
https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/
https://adevait.com/rust/deploying-rust-functions-on-aws-lambda
https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html

# TITLE:
