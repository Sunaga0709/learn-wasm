use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[derive(Debug, Deserialize)]
struct Input {
    message: String,
}

#[derive(Debug, Serialize)]
struct Output {
    echo: String,
}

impl From<Input> for Output {
    fn from(value: Input) -> Self {
        Self {
            echo: value.message,
        }
    }
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_echo(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let input = serde_json::from_slice::<Input>(req.body())?;
    let output = Output::from(input);
    let output = serde_json::to_string(&output)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/json")
        .body(output)
        .build())
}
