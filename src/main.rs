#[macro_use]
extern crate lambda_runtime;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use lambda_runtime::error::HandlerError;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() {
    lambda!(my_handler);
}

fn my_handler(e: CustomEvent, ctx: lambda_runtime::Context) -> Result<CustomOutput, HandlerError> {

    if e.first_name == "" {
        return Err(ctx.new_error("Missing first name!"));
    }

    println!("Hello {} from rust running on AWS Lambda.", e.first_name);

    Ok(CustomOutput{
        message: format!("Hello, {}!", e.first_name),
    })
}
