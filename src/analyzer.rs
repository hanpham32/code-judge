use anyhow::Result;
use colored::*;
use std::path::Path;

pub struct CodeAnalysis {
    pub code: String,
    pub issues: Vec<CodeIssue>,
    pub ai_response: String,
}

pub struct CodeIssue {
    pub line_number: usize,
    pub issue_type: IssueType,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum IssueType {
    StyleIssue,
    PotentialBug,
    Performance,
    Todo,
}

pub struct Analyzer {
    pub code: String,
}

impl Analyzer {
    pub fn new(code: String) -> Self {
        Self { code }
    }

    pub fn analyze(&self) -> Result<Vec<CodeIssue>> {
        let mut issues = Vec::new();

        for (line_idx, line) in self.code.lines().enumerate() {
            if line.len() > 100 {
                issues.push(CodeIssue {
                    line_number: line_idx + 1,
                    issue_type: IssueType::StyleIssue,
                    message: "Line exceeds 100 characters".to_string(),
                });
            }

            if line.contains("TODO") {
                issues.push(CodeIssue {
                    line_number: line_idx + 1,
                    issue_type: IssueType::Todo,
                    message: "TODO comment found".to_string(),
                });
            }
        }
        Ok(issues)
    }
}
