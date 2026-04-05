use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAction {
    Deny,
    RequireApproval,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub name: String,
    pub action: PolicyAction,
    pub contains: String,
}

impl PolicyRule {
    pub fn deny_contains(name: impl Into<String>, needle: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            action: PolicyAction::Deny,
            contains: needle.into(),
        }
    }

    pub fn require_approval_contains(name: impl Into<String>, needle: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            action: PolicyAction::RequireApproval,
            contains: needle.into(),
        }
    }

    fn matches(&self, input: &str) -> bool {
        let haystack = input.to_ascii_lowercase();
        let needle = self.contains.to_ascii_lowercase();
        !needle.is_empty() && haystack.contains(&needle)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
    Allow,
    RequireApproval { rule: String, reason: String },
    Deny { rule: String, reason: String },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyEngine {
    pub rules: Vec<PolicyRule>,
}

impl PolicyEngine {
    pub fn with_rule(mut self, rule: PolicyRule) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn evaluate(&self, input: &str) -> PolicyDecision {
        for rule in &self.rules {
            if !rule.matches(input) {
                continue;
            }

            let reason = format!("rule '{}' matched '{}'", rule.name, rule.contains);
            return match rule.action {
                PolicyAction::Deny => PolicyDecision::Deny {
                    rule: rule.name.clone(),
                    reason,
                },
                PolicyAction::RequireApproval => PolicyDecision::RequireApproval {
                    rule: rule.name.clone(),
                    reason,
                },
            };
        }

        PolicyDecision::Allow
    }
}
