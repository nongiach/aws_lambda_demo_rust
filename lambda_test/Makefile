update:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap . && zip lambda.zip bootstrap && rm bootstrap
	aws lambda update-function-code --function-name rust_lambda --zip-file fileb://lambda.zip
