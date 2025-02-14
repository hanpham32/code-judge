use anyhow::Result;
use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, process};
use tracing::info;

mod analyzer;
mod assertions;
mod character;
mod model;

use analyzer::{Analyzer, IssueType};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeResponse {
    content: Vec<ContentItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContentItem {
    text: String,
    #[serde(rename = "type")]
    content_type: String,
}

struct Judgement {
    score: f64,
    message: String,
}

fn setup_logging() {
    tracing_subscriber::fmt::init();
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

fn judge_score(code: &str, assertions: [&str; 18]) -> Result<Judgement> {
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

fn analyze_code(
    code: String,
    assertions: [&str; 18],
) -> Result<(Judgement, Vec<analyzer::CodeIssue>)> {
    let judgement = judge_score(&code, assertions)?;

    let analyzer = Analyzer::new(code.to_string());
    let issues = analyzer.analyze()?;

    Ok((judgement, issues))
}

fn main() -> Result<()> {
    setup_logging();
    let args = Args::parse();

    info!("Starting code-judge");
    println!("{}", "Code Judge is running!".green());

    if let Some(path) = args.path {
        println!("Reading path: {}", path.blue());

        let code = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", path, e);
                process::exit(1);
            }
        };

        let (judgement, issues) = analyze_code(code, assertions::ASSERTIONS)?;

        println!(
            "\n========= AI Evaluation =======\nMessage: {}\n\nScore: {}{}{}\n",
            judgement.message,
            if judgement.score < 2.0 { RED } else { GREEN },
            judgement.score,
            RESET
        );

        if !issues.is_empty() {
            println!("\n========= Static Analysis =======");
            for issue in issues {
                let color = match issue.issue_type {
                    IssueType::Todo => "blue",
                    IssueType::StyleIssue => "orange",
                    IssueType::PotentialBug => "red",
                    IssueType::Performance => "pink",
                };
                println!(
                    "Line {}: {}",
                    issue.line_number.to_string().cyan(),
                    issue.message.color(color)
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependencies_load() {
        assert!(true);
    }

    #[test]
    fn test_analyze_code() -> Result<()> {
        let code = "fn main() {\n    // TODO: Implement this very long line that should trigger a length warning in our analysis\n}".to_string();
        let analyzer = Analyzer::new(code);
        let issues = analyzer.analyze()?;

        assert_eq!(issues.len(), 1);
        Ok(())
    }
}
