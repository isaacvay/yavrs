//! Internationalization (i18n) catalog for the Yav.rs official website.
//!
//! Defaults to English (`"en"`) for server-side rendering and static export, and embeds
//! a complete synchronized dictionary (`en`, `fr`, `es`, `de`) for instant zero-roundtrip
//! client-side language switching via [`get_i18n_json`].

mod arch;
mod bench;
mod comp;
mod docs;
mod home;
mod pages;
mod shell;

use std::collections::BTreeMap;
use std::sync::OnceLock;
use yav::prelude::I18n;

pub use shell::Entry;

pub const DEFAULT_LOCALE: &str = "en";
pub const SUPPORTED_LOCALES: &[&str] = &["en", "fr", "es", "de"];

static I18N_JSON_CACHE: OnceLock<String> = OnceLock::new();

/// Returns all modular translation tables composing the site dictionary.
pub fn all_sections() -> &'static [&'static [Entry]] {
    &[
        shell::ENTRIES,
        home::ENTRIES,
        arch::ENTRIES,
        bench::ENTRIES,
        pages::ENTRIES,
        comp::ENTRIES,
        docs::ENTRIES,
    ]
}

/// Builds the complete [`I18n`] catalog populated across all supported locales (`en`, `fr`, `es`, `de`).
pub fn build_i18n() -> I18n {
    let mut i18n = I18n::new(DEFAULT_LOCALE).supported_locales(SUPPORTED_LOCALES.iter().copied());

    let total: usize = all_sections().iter().map(|s| s.len()).sum();
    let mut en = Vec::with_capacity(total);
    let mut fr = Vec::with_capacity(total);
    let mut es = Vec::with_capacity(total);
    let mut de = Vec::with_capacity(total);

    for section in all_sections() {
        for &(key, v_en, v_fr, v_es, v_de) in *section {
            en.push((key, v_en));
            fr.push((key, v_fr));
            es.push((key, v_es));
            de.push((key, v_de));
        }
    }

    i18n = i18n.add_translations("en", en);
    i18n = i18n.add_translations("fr", fr);
    i18n = i18n.add_translations("es", es);
    i18n = i18n.add_translations("de", de);
    i18n
}

/// Returns a lazily cached, deterministic JSON payload containing every translation key
/// for `en`, `fr`, `es`, and `de`, safe for direct embedding inside `<script type="application/json">`.
pub fn get_i18n_json() -> &'static str {
    I18N_JSON_CACHE.get_or_init(|| {
        let mut root: BTreeMap<&'static str, BTreeMap<&'static str, &'static str>> =
            BTreeMap::new();
        let mut en = BTreeMap::new();
        let mut fr = BTreeMap::new();
        let mut es = BTreeMap::new();
        let mut de = BTreeMap::new();

        for section in all_sections() {
            for &(key, v_en, v_fr, v_es, v_de) in *section {
                en.insert(key, v_en);
                fr.insert(key, v_fr);
                es.insert(key, v_es);
                de.insert(key, v_de);
            }
        }

        root.insert("en", en);
        root.insert("fr", fr);
        root.insert("es", es);
        root.insert("de", de);

        // Escape '<' as '\u003c' so embedded JSON can never contain '</script' or '<!--'.
        serde_json::to_string(&root)
            .expect("static i18n catalog must serialize to JSON")
            .replace('<', "\\u003c")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeSet, HashMap, HashSet};
    use std::fs;
    use std::path::{Path, PathBuf};

    fn collect_source_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let entries = fs::read_dir(dir).expect("read src dir");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().and_then(|n| n.to_str()) == Some("i18n") {
                    continue;
                }
                collect_source_files(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }

    fn catalog_en_map() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        for section in all_sections() {
            for &(key, v_en, _, _, _) in *section {
                map.insert(key, v_en);
            }
        }
        map
    }

    fn decode_rsx_string(raw: &str) -> String {
        // First unescape Rust string literal escapes inside RSX `"..."` literals.
        let mut out = String::with_capacity(raw.len());
        let mut chars = raw.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some('n') => out.push('\n'),
                    Some('r') => out.push('\r'),
                    Some('t') => out.push('\t'),
                    Some('0') => out.push('\0'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(c);
            }
        }
        // Then decode common HTML named entities used in RSX literals.
        out.replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
            .replace("&quot;", "\"")
    }

    fn decode_js_string(raw: &str) -> String {
        let mut out = String::with_capacity(raw.len());
        let mut chars = raw.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('\'') => out.push('\''),
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some('n') => out.push('\n'),
                    Some('r') => out.push('\r'),
                    Some('t') => out.push('\t'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    /// Scans forward from the start of an opening HTML/RSX tag (`<tag ...>`),
    /// respecting double-quoted attribute values so `=>` inside `onclick="..."` does
    /// not prematurely end the tag. Returns the byte index of closing `>`.
    fn find_tag_end(s: &str, start: usize) -> Option<usize> {
        let bytes = s.as_bytes();
        let mut i = start;
        let mut in_quote = false;
        while i < bytes.len() {
            let b = bytes[i];
            if in_quote {
                if b == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if b == b'"' {
                    in_quote = false;
                }
            } else {
                if b == b'"' {
                    in_quote = true;
                } else if b == b'>' {
                    return Some(i);
                }
            }
            i += 1;
        }
        None
    }

    fn is_ident_char(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }

    fn extract_attr_value(tag: &str, attr_name: &str) -> Option<String> {
        let needle = format!("{attr_name}=\"");
        let bytes = tag.as_bytes();
        let mut search_from = 0;
        while let Some(pos) = tag[search_from..].find(&needle) {
            let abs = search_from + pos;
            if abs > 0 && !bytes[abs - 1].is_ascii_whitespace() {
                search_from = abs + needle.len();
                continue;
            }
            let val_start = abs + needle.len();
            let mut i = val_start;
            while i < bytes.len() {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    return Some(decode_rsx_string(&tag[val_start..i]));
                }
                i += 1;
            }
            break;
        }
        None
    }

    fn parse_quoted_after(s: &str, start: usize, quote: u8) -> Option<(String, usize)> {
        let bytes = s.as_bytes();
        let mut i = start;
        while i < bytes.len() {
            if bytes[i] == b'\\' && i + 1 < bytes.len() {
                i += 2;
                continue;
            }
            if bytes[i] == quote {
                return Some((s[start..i].to_string(), i + 1));
            }
            i += 1;
        }
        None
    }

    #[test]
    fn no_duplicate_keys_and_all_locales_non_empty() {
        let mut seen = HashSet::new();
        let mut total = 0usize;

        for section in all_sections() {
            for &(key, v_en, v_fr, v_es, v_de) in *section {
                assert!(!key.is_empty(), "empty translation key found");
                assert_eq!(
                    key,
                    key.trim(),
                    "translation key '{key}' has leading/trailing whitespace"
                );
                assert!(
                    seen.insert(key),
                    "duplicate translation key detected: '{key}'"
                );
                assert!(!v_en.is_empty(), "empty EN translation for '{key}'");
                assert!(!v_fr.is_empty(), "empty FR translation for '{key}'");
                assert!(!v_es.is_empty(), "empty ES translation for '{key}'");
                assert!(!v_de.is_empty(), "empty DE translation for '{key}'");
                total += 1;
            }
        }

        assert_eq!(seen.len(), total);
        assert!(total >= 1640, "expected >= 1640 i18n keys, got {total}");
    }

    #[test]
    fn json_payload_is_valid_complete_and_script_safe() {
        let json = get_i18n_json();
        assert!(
            !json.contains('<'),
            "i18n JSON must escape '<' as \\u003c for safe <script> embedding"
        );
        assert!(
            !json.to_ascii_lowercase().contains("</script"),
            "i18n JSON must not contain </script"
        );

        let parsed: serde_json::Value =
            serde_json::from_str(json).expect("get_i18n_json must produce valid JSON");
        let root = parsed.as_object().expect("top-level JSON must be an object");
        let top_keys: BTreeSet<&str> = root.keys().map(String::as_str).collect();
        let expected_locales: BTreeSet<&str> = SUPPORTED_LOCALES.iter().copied().collect();
        assert_eq!(top_keys, expected_locales);

        let expected_keys: BTreeSet<&str> = all_sections()
            .iter()
            .flat_map(|s| s.iter().map(|e| e.0))
            .collect();

        for loc in SUPPORTED_LOCALES {
            let loc_map = root[*loc]
                .as_object()
                .unwrap_or_else(|| panic!("locale '{loc}' must be a JSON object"));
            let loc_keys: BTreeSet<&str> = loc_map.keys().map(String::as_str).collect();
            assert_eq!(
                loc_keys, expected_keys,
                "locale '{loc}' key set drifted from catalog"
            );
        }
    }

    #[test]
    fn build_i18n_contains_all_catalog_entries() {
        let i18n = build_i18n();
        for section in all_sections() {
            for &(key, v_en, v_fr, v_es, v_de) in *section {
                assert_eq!(i18n.t("en", key), v_en, "EN mismatch in build_i18n for {key}");
                assert_eq!(i18n.t("fr", key), v_fr, "FR mismatch in build_i18n for {key}");
                assert_eq!(i18n.t("es", key), v_es, "ES mismatch in build_i18n for {key}");
                assert_eq!(i18n.t("de", key), v_de, "DE mismatch in build_i18n for {key}");
            }
        }
    }

    #[test]
    fn source_keys_exist_and_english_literals_match_catalog() {
        let catalog = catalog_en_map();
        let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        collect_source_files(&src_root, &mut files);
        assert!(files.len() >= 50, "expected full website src scan");

        let mut missing_keys = Vec::new();
        let mut en_mismatches = Vec::new();

        for file in &files {
            let rel = file
                .strip_prefix(env!("CARGO_MANIFEST_DIR"))
                .unwrap_or(file)
                .display()
                .to_string();
            let content = fs::read_to_string(file).expect("read source file");

            // A) Scan attribute i18n: data-i18n-(placeholder|title|aria-label)="key"
            for attr_kind in ["placeholder", "title", "aria-label"] {
                let marker = format!("data-i18n-{attr_kind}=\"");
                let mut pos = 0;
                while let Some(rel_idx) = content[pos..].find(&marker) {
                    let marker_abs = pos + rel_idx;
                    let key_start = marker_abs + marker.len();
                    let Some(key_end_rel) = content[key_start..].find('"') else {
                        break;
                    };
                    let key = &content[key_start..key_start + key_end_rel];
                    pos = key_start + key_end_rel + 1;

                    if !catalog.contains_key(key) {
                        missing_keys.push(format!("{rel}: data-i18n-{attr_kind}=\"{key}\""));
                        continue;
                    }

                    // Walk backwards to '<' of this tag to inspect paired attribute.
                    if let Some(tag_start) = content[..marker_abs].rfind('<') {
                        if let Some(tag_end) = find_tag_end(&content, tag_start + 1) {
                            let tag_slice = &content[tag_start..=tag_end];
                            if let Some(attr_val) = extract_attr_value(tag_slice, attr_kind) {
                                let cat_en = catalog[key];
                                if attr_val != cat_en {
                                    en_mismatches.push(format!(
                                        "{rel} [{key}] ({attr_kind}): src={attr_val:?} != catalog={cat_en:?}"
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            // B) Scan unescaped RSX data-i18n="key"
            let marker = "data-i18n=\"";
            let mut pos = 0;
            while let Some(rel_idx) = content[pos..].find(marker) {
                let marker_abs = pos + rel_idx;
                let key_start = marker_abs + marker.len();
                let Some(key_end_rel) = content[key_start..].find('"') else {
                    break;
                };
                let key = &content[key_start..key_start + key_end_rel];
                pos = key_start + key_end_rel + 1;

                if key == "{key}" {
                    continue;
                }
                if !catalog.contains_key(key) {
                    missing_keys.push(format!("{rel}: data-i18n=\"{key}\""));
                    continue;
                }

                // If simple RSX literal `"..."</tag>` immediately follows closing `>`, verify EN parity.
                if let Some(tag_end) = find_tag_end(&content, pos) {
                    let after = content[tag_end + 1..].trim_start();
                    if let Some(stripped) = after.strip_prefix('"') {
                        if let Some((raw_lit, end_lit)) = parse_quoted_after(stripped, 0, b'"') {
                            let rest = stripped[end_lit..].trim_start();
                            if rest.starts_with("</") {
                                let decoded = decode_rsx_string(&raw_lit);
                                let cat_en = catalog[key];
                                if decoded != cat_en {
                                    en_mismatches.push(format!(
                                        "{rel} [{key}]: src={decoded:?} != catalog={cat_en:?}"
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            // C) Scan Rust escaped HTML strings: data-i18n=\"key\">text</tag>
            let esc_marker = "data-i18n=\\\"";
            let mut pos = 0;
            while let Some(rel_idx) = content[pos..].find(esc_marker) {
                let marker_abs = pos + rel_idx;
                let key_start = marker_abs + esc_marker.len();
                let Some(key_end_rel) = content[key_start..].find("\\\"") else {
                    break;
                };
                let key = &content[key_start..key_start + key_end_rel];
                pos = key_start + key_end_rel + 2;

                if key == "{key}" {
                    continue;
                }
                if !catalog.contains_key(key) {
                    missing_keys.push(format!("{rel}: escaped data-i18n=\"{key}\""));
                    continue;
                }

                if let Some(gt_rel) = content[pos..].find('>') {
                    let text_start = pos + gt_rel + 1;
                    if let Some(lt_rel) = content[text_start..].find("</") {
                        let inner = &content[text_start..text_start + lt_rel];
                        if !inner.contains('{') {
                            let cat_en = catalog[key];
                            if inner != cat_en {
                                en_mismatches.push(format!(
                                    "{rel} [{key}] (raw str): src={inner:?} != catalog={cat_en:?}"
                                ));
                            }
                        }
                    }
                }
            }

            // D) Scan JS i18n helper calls: yavT('key', 'fallback') / t('key', 'fallback')
            for fn_name in ["yavT(", "t("] {
                let mut pos = 0;
                while let Some(rel_idx) = content[pos..].find(fn_name) {
                    let abs = pos + rel_idx;
                    pos = abs + fn_name.len();

                    // Word-boundary check so format(...), get(...), etc. don't match t(.
                    if abs > 0 && is_ident_char(content.as_bytes()[abs - 1]) {
                        continue;
                    }
                    // Skip helper definition signature `function t(k, fb)` or Rust `fn t(`.
                    let mut lo = abs.saturating_sub(24);
                    while lo > 0 && !content.is_char_boundary(lo) {
                        lo -= 1;
                    }
                    let before = content[lo..abs].trim_end();
                    if before.ends_with("function") || before.ends_with("fn") {
                        continue;
                    }

                    let mut arg_start = abs + fn_name.len();
                    let bytes = content.as_bytes();
                    while arg_start < bytes.len() && bytes[arg_start].is_ascii_whitespace() {
                        arg_start += 1;
                    }
                    if arg_start >= bytes.len()
                        || (bytes[arg_start] != b'\'' && bytes[arg_start] != b'"')
                    {
                        continue; // variable key like t(k, fb) or yavT(key, prev)
                    }
                    let q1 = bytes[arg_start];
                    let Some((key, mut after_k)) = parse_quoted_after(&content, arg_start + 1, q1)
                    else {
                        continue;
                    };
                    if !key.contains('.') {
                        continue;
                    }
                    if !catalog.contains_key(key.as_str()) {
                        missing_keys.push(format!("{rel}: {fn_name}'{key}')"));
                        continue;
                    }

                    // Check optional second literal fallback argument
                    while after_k < bytes.len() && bytes[after_k].is_ascii_whitespace() {
                        after_k += 1;
                    }
                    if after_k < bytes.len() && bytes[after_k] == b',' {
                        let mut fb_start = after_k + 1;
                        while fb_start < bytes.len() && bytes[fb_start].is_ascii_whitespace() {
                            fb_start += 1;
                        }
                        if fb_start < bytes.len()
                            && (bytes[fb_start] == b'\'' || bytes[fb_start] == b'"')
                        {
                            let q2 = bytes[fb_start];
                            if let Some((fb_raw, _)) = parse_quoted_after(&content, fb_start + 1, q2)
                            {
                                let fb = decode_js_string(&fb_raw);
                                let cat_en = catalog[key.as_str()];
                                if fb != cat_en {
                                    en_mismatches.push(format!(
                                        "{rel} [{key}] (JS fallback): js={fb:?} != catalog={cat_en:?}"
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Dynamic keys generated/looked up by runtime JS or Rust arrays
        for dyn_key in [
            "meta.title.home",
            "meta.title.about",
            "meta.title.docs",
            "meta.title.benchmarks",
            "meta.title.components",
            "meta.title.showcase",
            "meta.title.faq",
            "meta.title.releases",
            "meta.title.privacy",
            "meta.title.not_found",
            "comp.charts.m0",
            "comp.charts.m1",
            "comp.charts.m2",
            "comp.charts.m3",
            "comp.charts.m4",
            "comp.charts.m5",
            "comp.charts.m6",
            "comp.charts.m7",
            "comp.charts.m8",
            "comp.charts.m9",
            "comp.charts.m10",
            "comp.charts.m11",
            "comp.auth.hide",
        ] {
            if !catalog.contains_key(dyn_key) {
                missing_keys.push(format!("dynamic key missing: {dyn_key}"));
            }
        }

        assert!(
            missing_keys.is_empty(),
            "Missing i18n keys referenced in src:\n{}",
            missing_keys.join("\n")
        );
        assert!(
            en_mismatches.is_empty(),
            "English SSR vs catalog mismatches found ({}):\n{}",
            en_mismatches.len(),
            en_mismatches.join("\n")
        );
    }

    #[test]
    fn no_orphan_catalog_keys() {
        let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        collect_source_files(&src_root, &mut files);

        let mut corpus = String::new();
        for file in files {
            corpus.push_str(&fs::read_to_string(file).expect("read source file"));
            corpus.push('\n');
        }

        let mut orphans = Vec::new();
        for section in all_sections() {
            for &(key, _, _, _, _) in *section {
                if !corpus.contains(key) {
                    orphans.push(key);
                }
            }
        }

        assert!(
            orphans.is_empty(),
            "Unused/orphan catalog keys found: {orphans:?}"
        );
    }

    #[test]
    fn no_leaked_french_or_broken_escapes_in_src() {
        let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        collect_source_files(&src_root, &mut files);

        let forbidden = [
            "Rechercher un component",
            "Composant introuvable",
            "Requête",
            "$O(",
        ];

        for file in files {
            let content = fs::read_to_string(&file).expect("read source file");
            for needle in forbidden {
                assert!(
                    !content.contains(needle),
                    "Forbidden leaked string {needle:?} found in {}",
                    file.display()
                );
            }
        }

        let privacy = fs::read_to_string(src_root.join("routes").join("privacy.rs"))
            .expect("read privacy.rs");
        assert!(
            privacy.contains(r";, \\r, \\n, or \\0."),
            "privacy.rs must double-escape \\r, \\n, \\0 in RSX text"
        );
    }
}
