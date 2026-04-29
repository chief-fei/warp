//! Internationalization (i18n) support for Warp.
//!
//! This module provides a simple translation mechanism for UI strings.
//! The current language is stored globally and applied to all translations.
//!
//! # Usage
//! ```ignore
//! // Set the language once at startup:
//! i18n::set_language(Language::Chinese);
//!
//! // Translate a string:
//! let label = i18n::t("API Key Settings");
//! ```

use std::sync::atomic::{AtomicU8, Ordering};

/// Supported display languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Language {
    /// English (default)
    #[default]
    English = 0,
    /// Simplified Chinese (简体中文)
    ChineseSimplified = 1,
}

impl Language {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Language::ChineseSimplified,
            _ => Language::English,
        }
    }

    /// Human-readable display name of the language.
    pub fn display_name(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::ChineseSimplified => "简体中文",
        }
    }
}

/// The globally active UI language, stored as an atomic u8 for lock-free reads.
static CURRENT_LANGUAGE: AtomicU8 = AtomicU8::new(0);

/// Sets the active UI language. Call this once at startup after reading the user's preference.
pub fn set_language(lang: Language) {
    CURRENT_LANGUAGE.store(lang as u8, Ordering::Relaxed);
}

/// Returns the currently active UI language.
pub fn current_language() -> Language {
    Language::from_u8(CURRENT_LANGUAGE.load(Ordering::Relaxed))
}

/// Translates a UI string into the currently active language.
///
/// For English, the input string is returned as-is (the English string IS the key).
/// For other languages, a lookup is performed and the translation is returned.
/// If no translation is found, the original English string is returned as a fallback.
pub fn t(english: &'static str) -> &'static str {
    match current_language() {
        Language::English => english,
        Language::ChineseSimplified => zh_cn(english),
    }
}

/// Returns the Simplified Chinese translation for a given English string,
/// or falls back to the original English string if no translation exists.
fn zh_cn(english: &'static str) -> &'static str {
    match english {
        // ── AI Settings page ──────────────────────────────────────────────────
        "AI" => "AI 设置",
        "API Key Settings" => "API 密钥设置",
        "Bring Your Own API Key" => "使用自己的 API 密钥",
        "Use your own API keys from model providers for the Warp Agent to use. API keys are stored locally and never synced to the cloud. Using auto models or models from providers you have not provided API keys for will consume Warp credits."
            => "使用您自己的模型提供商 API 密钥，供 Warp Agent 使用。API 密钥仅保存在本地，不会同步到云端。使用自动模型或未提供 API 密钥的提供商模型将消耗 Warp 积分。",
        "OpenAI API Key" => "OpenAI API 密钥",
        "Anthropic API Key" => "Anthropic API 密钥",
        "Google API Key" => "Google API 密钥",
        "DeepSeek API Key" => "DeepSeek API 密钥",
        // ── Model selection ───────────────────────────────────────────────────
        "Agent Mode" => "代理模式",
        "Coding" => "编码",
        "Model" => "模型",
        "Select Model" => "选择模型",
        "Default" => "默认",
        // ── General AI settings ───────────────────────────────────────────────
        "Enable AI" => "启用 AI",
        "AI is disabled" => "AI 已禁用",
        "AI Features" => "AI 功能",
        "Natural Language" => "自然语言",
        "Autosuggestions" => "自动建议",
        "Voice Input" => "语音输入",
        // ── Settings navigation ───────────────────────────────────────────────
        "Settings" => "设置",
        "Appearance" => "外观",
        "Privacy" => "隐私",
        "Features" => "功能",
        "Keybindings" => "快捷键",
        // ── Actions / buttons ─────────────────────────────────────────────────
        "Save" => "保存",
        "Cancel" => "取消",
        "Close" => "关闭",
        "Edit" => "编辑",
        "Delete" => "删除",
        "Add" => "添加",
        "Remove" => "移除",
        "Enable" => "启用",
        "Disable" => "禁用",
        "Update" => "更新",
        "Upgrade" => "升级",
        // ── Language setting ──────────────────────────────────────────────────
        "Display Language" => "显示语言",
        "Language" => "语言",
        // ── AWS Bedrock ───────────────────────────────────────────────────────
        "AWS Bedrock" => "AWS Bedrock",
        "Enable AWS Bedrock" => "启用 AWS Bedrock",
        // ── Fallback ──────────────────────────────────────────────────────────
        other => other,
    }
}
