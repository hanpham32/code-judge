use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, fs, process};

#[derive(Debug, Serialize, Deserialize)]
struct ContentItem {
    text: String,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeResponse {
    content: Vec<ContentItem>,
}

fn get_claude_response(prompt: &str) -> Result<String> {
    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("failed to retrieve API Key");
    let model = "claude-3-5-haiku-latest";

    let mut response: ClaudeResponse = ureq::post("https://api.anthropic.com/v1/messages")
        .set("x-api-key", &api_key)
        .set("anthropic-version", "2023-06-01")
        .set("content-type", "application/json")
        .send_json(json!({
            "model": model,
            "temperature": 0.0,
            "messages": [{
                "role": "user",
                "content": prompt
            }],
            "max_tokens": 1024
        }))?
        .into_json()?;

    Ok(response.content.remove(0).text)
}

struct Judgement {
    score: f64,
    message: String,
}

fn judge_score(code: &String, assertions: Vec<&str>) -> Result<Judgement> {
    let mut fenced_code = String::from("```");
    fenced_code.push_str(code);
    fenced_code.push_str("```");

    let formatted_assertions = assertions
        .iter()
        .map(|a| format!("- {}", a))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = include_str!("../prompts/judge.md")
        .replace("<code>", &fenced_code)
        .replace("<assertions>", &formatted_assertions);

    let response = get_claude_response(&prompt)?;

    let (message, score_text) = response
        .rsplit_once('\n')
        .ok_or(anyhow::anyhow!("Failed to parse score"))?;

    let score = score_text.parse::<f64>()?;

    Ok(Judgement {
        score,
        message: message.trim().into(),
    })
}

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <code_file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // let code = include_str!("../data/code.cs");
    let code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    let assertions = vec![
        "Each class should have only one reason to change, meaning it should have only one job or responsibility.",
        "Classes should be open for extension but closed for modification. New functionality should be added by extending the class, not by modifying existing code.",
        "Objects of a superclass should be replaceable with objects of a subclass without affecting the correctness of the program.",
        "Clients should not be forced to depend on interfaces they do not use. Interfaces should be specific to the needs of the client.",
        "High-level modules should not depend on low-level modules. Both should depend on abstractions. Abstractions should not depend on details. Details should depend on abstractions.",
        "Code should be readable and self-documenting, with meaningful names and clear structure, reducing the need for excessive comments.",
    ];


    let result = judge_score(&code, assertions);
    match result {
        Ok(judgement) => println!(
            "========= Result =======\nMessage: {}\n\nScore: {}{}{}\n",
            judgement.message,
            if judgement.score < 2.0 { RED } else { GREEN },
            judgement.score,
            RESET
        ),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
