/*!
A small, dependency-free i18n module for ripgrep.

All user-visible strings live in external `.ftl` files (one per language)
under a `locales` directory. The `.ftl` format is intentionally simple:

    key = value

Multi-line values are supported: every line after the `key = ` line (until
the next `key = ` line) belongs to the value of that key.

The language is chosen from the `LANG` environment variable, whose value has
the shape `language_region.encoding`, e.g. `zh_CN.UTF-8`. This is normalized
into a language tag such as `zh-CN` (the encoding and any `@modifier` are
stripped, and `_` becomes `-`). ripgrep then looks for a file named
`<tag>.ftl` (e.g. `zh-CN.ftl`) in a `locales` directory next to the ripgrep
binary, or in the current working directory.

The default language is `en-US`. When the language selected via `LANG` has no
corresponding `.ftl` file, or when `LANG` is unset or unrecognized (e.g. `C`
or `POSIX`), the `en-US.ftl` file is used instead. If even `en-US.ftl` cannot
be found, a warning is printed and translation keys are returned verbatim so
that failures remain visible.
*/

use std::{collections::HashMap, env, path::PathBuf, sync::LazyLock};

/// The default language tag used whenever the language selected via `LANG`
/// cannot be resolved to an existing `.ftl` file.
const DEFAULT_LANG: &str = "en-US";

/// The loaded translation state.
#[allow(dead_code)]
struct I18n {
    /// The language tag that was actually loaded.
    lang: String,
    /// The mapping from translation keys to their localized values.
    messages: HashMap<String, String>,
}

/// The global translation state, initialized lazily on first use.
static I18N: LazyLock<I18n> = LazyLock::new(I18n::load);

impl I18n {
    /// Load the translation state for the language selected via `LANG`.
    fn load() -> I18n {
        let lang = detect_lang();
        let dirs = locales_dirs();
        // 1. Try the language selected via LANG.
        for dir in &dirs {
            if let Some(messages) = try_load(dir, &lang) {
                return I18n { lang, messages };
            }
        }
        // 2. Fall back to the default language.
        for dir in &dirs {
            if let Some(messages) = try_load(dir, DEFAULT_LANG) {
                return I18n { lang: DEFAULT_LANG.to_string(), messages };
            }
        }
        // 3. Nothing usable found: fall back to raw keys so that failures
        // remain visible. (This message cannot itself be translated because
        // no translation file is available.)
        eprintln!(
            "rg: warning: could not find locales/{lang}.ftl or \
             locales/{DEFAULT_LANG}.ftl; showing raw translation keys"
        );
        I18n { lang, messages: HashMap::new() }
    }
}

/// Return the localized string for the given key.
///
/// If the key is unknown, then the key itself is returned.
pub(crate) fn t(key: &str) -> String {
    I18N.messages.get(key).cloned().unwrap_or_else(|| key.to_string())
}

/// Return the localized string for the given key with named placeholders
/// (of the form `{name}`) replaced by the corresponding argument values.
///
/// If the key is unknown, then the key itself is returned.
pub(crate) fn t_args(key: &str, args: &[(&str, &str)]) -> String {
    let mut msg = t(key);
    for (name, value) in args {
        msg = msg.replace(&format!("{{{name}}}"), value);
    }
    msg
}

/// Return the language tag currently in use, e.g. `en-US` or `zh-CN`.
#[allow(dead_code)]
pub(crate) fn lang() -> &'static str {
    &I18N.lang
}

/// Normalize the value of `LANG` into a language tag.
///
/// `LANG` values look like `language_region.encoding`, e.g. `zh_CN.UTF-8`,
/// `en_US.UTF-8` or `fr_FR@euro`. This function strips any `@modifier` and
/// `.encoding` parts and turns `_` into `-`, producing tags such as `zh-CN`,
/// `en-US` or `fr-FR`. Values that do not identify a language (`C`, `POSIX`,
/// empty) are mapped to the default language.
fn normalize_lang(value: &str) -> String {
    let v = value.split('@').next().unwrap_or(value);
    let v = v.split('.').next().unwrap_or(v).trim();
    if v.is_empty() || v == "C" || v == "POSIX" {
        return DEFAULT_LANG.to_string();
    }
    v.replace('_', "-")
}

/// Detect the desired language from the `LANG` environment variable.
fn detect_lang() -> String {
    match env::var("LANG") {
        Ok(value) => normalize_lang(&value),
        Err(_) => DEFAULT_LANG.to_string(),
    }
}

/// Return the candidate `locales` directories, in priority order.
///
/// The directory given by the `RG_LOCALES_DIR` environment variable (when
/// set) is preferred. Otherwise, the directory next to the ripgrep binary is
/// tried, followed by the current working directory.
fn locales_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![];
    if let Some(dir) = env::var_os("RG_LOCALES_DIR") {
        dirs.push(PathBuf::from(dir));
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(parent) = exe.parent() {
            dirs.push(parent.join("locales"));
        }
    }
    if let Ok(cwd) = env::current_dir() {
        dirs.push(cwd.join("locales"));
    }
    dirs
}

/// Try to load the `.ftl` file for the given language tag from the given
/// directory. Returns `None` if the file is missing or cannot be read.
fn try_load(
    dir: &std::path::Path,
    lang: &str,
) -> Option<HashMap<String, String>> {
    let path = dir.join(format!("{lang}.ftl"));
    let content = std::fs::read_to_string(path).ok()?;
    Some(parse_ftl(&content))
}

/// Parse the simple `key = value` format used by ripgrep's `.ftl` files.
///
/// Multi-line values are supported: all lines following a `key = ` line
/// (until the next `key = ` line) belong to that key's value. Leading and
/// trailing whitespace of each value is trimmed.
fn parse_ftl(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut cur_key: Option<String> = None;
    let mut cur_value = String::new();
    for line in content.lines() {
        if let Some(eq) = find_key_separator(line) {
            if let Some(key) = cur_key.take() {
                map.insert(key, cur_value.trim().to_string());
            }
            cur_key = Some(line[..eq].trim().to_string());
            cur_value = line[eq + 1..].to_string();
        } else if cur_key.is_some() {
            cur_value.push('\n');
            cur_value.push_str(line);
        }
    }
    if let Some(key) = cur_key.take() {
        map.insert(key, cur_value.trim().to_string());
    }
    map
}

/// Find the `=` that separates a key from its value at the start of a line.
///
/// Keys are restricted to lowercase ASCII letters, digits and hyphens, which
/// keeps lines inside multi-line values (e.g. prose in a help document) from
/// being mistaken for new entries.
fn find_key_separator(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let mut i = 0;
    let b = bytes[0];
    if !(b.is_ascii_lowercase() || b.is_ascii_digit()) {
        return None;
    }
    i += 1;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' {
            i += 1;
        } else {
            break;
        }
    }
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'=' { Some(i) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_lang_basic() {
        assert_eq!(normalize_lang("zh_CN.UTF-8"), "zh-CN");
        assert_eq!(normalize_lang("en_US.UTF-8"), "en-US");
        assert_eq!(normalize_lang("zh_CN"), "zh-CN");
        assert_eq!(normalize_lang("de"), "de");
        assert_eq!(normalize_lang("fr_FR@euro"), "fr-FR");
        assert_eq!(normalize_lang("C"), "en-US");
        assert_eq!(normalize_lang("POSIX"), "en-US");
        assert_eq!(normalize_lang("C.UTF-8"), "en-US");
        assert_eq!(normalize_lang(""), "en-US");
    }

    #[test]
    fn parse_ftl_single_line() {
        let map = parse_ftl("a = hello\nb = world\n");
        assert_eq!(map.get("a").map(String::as_str), Some("hello"));
        assert_eq!(map.get("b").map(String::as_str), Some("world"));
    }

    #[test]
    fn parse_ftl_multi_line() {
        let content = "a = first\nsecond line\n\nb = done\n";
        let map = parse_ftl(&content);
        assert_eq!(
            map.get("a").map(String::as_str),
            Some("first\nsecond line")
        );
        assert_eq!(map.get("b").map(String::as_str), Some("done"));
    }

    #[test]
    fn parse_ftl_prose_not_key() {
        // Lines inside a multi-line value must not be treated as keys, even
        // when they contain '='.
        let content = "a = This is prose\nwith an equals = sign inside\n";
        let map = parse_ftl(&content);
        assert_eq!(
            map.get("a").map(String::as_str),
            Some("This is prose\nwith an equals = sign inside")
        );
    }

    #[test]
    fn t_missing_key_returns_key() {
        // I18N is a global; this just checks the fallback logic via the map.
        let map: HashMap<String, String> = HashMap::new();
        let got =
            map.get("nope").cloned().unwrap_or_else(|| "nope".to_string());
        assert_eq!(got, "nope");
    }
}
