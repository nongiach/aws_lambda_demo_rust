use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LambdaRequest {
    full_name: String,
    message: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LambdaResponse {
    lambda_request: LambdaRequest,
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(lambda_handler);
    Ok(())
}

fn lambda_handler(e: LambdaRequest, _c: Context) -> Result<LambdaResponse, HandlerError> {
    let mut e = e.clone();
    e.full_name = format!("Hello {name}!", name = e.full_name);
    let msg = match e.message {
        Some(msg) => format!("Your message is '{msg}'.", msg = msg),
        None => format!("You have no message."),
    };
    e.message = Some(msg);
    Ok(LambdaResponse { lambda_request: e })
}

#[cfg(test)]
mod tests {
    use lambda_runtime::Context;
    use super::{LambdaRequest, LambdaResponse};

    #[test]
    fn test_lambda_handler() {
        let expected_response = LambdaResponse {
            lambda_request: LambdaRequest {
                full_name: "Hello Chuma Umenze!".to_string(),
                message: Some("Your message is 'It is a simple test'.".to_string())
            }
        };

        let lambda_context = Context {
            aws_request_id: "0123456789".to_string(),
            function_name: "test_function_name".to_string(),
            memory_limit_in_mb: 128,
            function_version: "$LATEST".to_string(),
            invoked_function_arn: "arn:aws:lambda".to_string(),
            xray_trace_id: Some("0987654321".to_string()),
            client_context: Option::default(),
            identity: Option::default(),
            log_stream_name: "logStreamName".to_string(),
            log_group_name: "logGroupName".to_string(),
            deadline: 0,
        };

        let lambda_request = LambdaRequest {
            full_name: "Chuma Umenze".to_string(),
            message: Some("It is a simple test".to_string())
        };

        // Check the result is ok
        let result = super::lambda_handler(lambda_request, lambda_context);
        assert_eq!(result.is_err(), false, "Error: {}", result.err().unwrap());

        // Confirm the expected values in result
        let value = result.ok().unwrap();
        assert_eq!(value.lambda_request.full_name, expected_response.lambda_request.full_name);
        assert_eq!(value.lambda_request.message, expected_response.lambda_request.message);
    }
}
