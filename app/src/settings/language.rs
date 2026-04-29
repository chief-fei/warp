//! Settings for UI display language / internationalization.

use serde::{Deserialize, Serialize};
use settings::{macros::define_settings_group, RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};
use warpui::{AppContext, Entity, SingletonEntity};

use crate::i18n;

/// The display language chosen by the user.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(description = "The language used for the Warp UI.", rename_all = "snake_case")]
pub enum DisplayLanguage {
    /// English (default)
    #[default]
    English,
    /// Simplified Chinese (简体中文)
    ChineseSimplified,
}

impl DisplayLanguage {
    /// Returns the human-readable display name of this language option.
    pub fn display_name(self) -> &'static str {
        match self {
            DisplayLanguage::English => "English",
            DisplayLanguage::ChineseSimplified => "简体中文 (Simplified Chinese)",
        }
    }

    /// Converts this setting value into the i18n `Language` enum.
    pub fn to_i18n_language(self) -> i18n::Language {
        match self {
            DisplayLanguage::English => i18n::Language::English,
            DisplayLanguage::ChineseSimplified => i18n::Language::ChineseSimplified,
        }
    }
}

settings::macros::implement_setting_for_enum!(
    DisplayLanguage,
    LanguageSettings,
    SupportedPlatforms::ALL,
    SyncToCloud::Globally(RespectUserSyncSetting::Yes),
    private: false,
    toml_path: "language.display_language",
    description: "The language used for Warp's user interface.",
);

define_settings_group!(LanguageSettings, settings: [
    display_language: DisplayLanguage,
]);

/// Applies the stored `DisplayLanguage` setting to the global i18n language.
///
/// Call this once at startup (after settings are registered) so that all
/// subsequent calls to [`crate::i18n::t`] use the user's preferred language.
pub fn apply_language_setting(ctx: &AppContext) {
    let lang = LanguageSettings::as_ref(ctx)
        .display_language
        .value()
        .to_i18n_language();
    i18n::set_language(lang);
}
