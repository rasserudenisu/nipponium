/// Provides functions to convert between hiragana and katakana.
/// Includes function to determine if string slice is exclusively a particular character set.

use std::char;

const HIRAGANA: (u32, u32) = (0x3041, 0x3096);
const KATAKANA: (u32, u32) = (0x30A1, 0x30F6);
const KANJI:    (u32, u32) = (0x4E00, 0x9FAF);
const TO_HIRAGANA: i32 = -96;
const TO_KATAKANA: i32 = 96;

/// Transmutes katakana to hiragana.
///
/// # Arguments
///
/// * `text` - A string slice that holds katakana text
///
/// # Example
///
/// ```
/// use nipponium::kana;
///
/// kana::to_hiragana("アイウエオ");
/// ```
pub fn to_hiragana(text: &str) -> String {
    return kana_converter(KATAKANA, text, TO_HIRAGANA);
}

/// Transmutes hiragana to katakana.
///
/// # Arguments
///
/// * `text` - A string slice that holds hiragana text
///
/// # Example
///
/// ```
/// use nipponium::kana;
///
/// kana::to_katakana("あいうえお");
/// ```
pub fn to_katakana(text: &str) -> String {
    return kana_converter(HIRAGANA, text, TO_KATAKANA);
}

/// Checks whether the entire string slice is hiragana or not.
///
/// # Arguments
///
/// * `text` - A string slice that holds target text
pub fn is_hiragana(text: &str) -> bool {
    return is_script(HIRAGANA, text);
}

/// Checks whether the entire string slice is katakana or not.
///
/// # Arguments
///
/// * `text` - A string slice that holds target text
pub fn is_katakana(text: &str) -> bool {
    return is_script(KATAKANA, text);
}

/// Checks whether the entire string slice is kanji or not.
///
/// # Arguments
///
/// * `text` - A string slice that holds target text
pub fn is_kanji(text: &str) -> bool {
    return is_script(KANJI, text);
}

fn is_script(range: (u32, u32), text: &str) -> bool {
    for moji in text.chars() {
        if !in_range(range, &moji) {
            return false;
        }
    }
    return true;
}

fn in_range(range: (u32, u32), moji: &char) -> bool {
    return range.0 < (*moji as u32) && (*moji as u32) < range.1;
}

fn kana_converter(range: (u32, u32), text: &str, direction: i32) -> String
 {
    let mut result = "".to_string();
    for moji in text.chars() {
        if in_range(range, &moji) {
            let mut i_value = moji as i32;
            i_value = i_value + direction;
            let y: char = char::from_u32(i_value as u32).unwrap();
            result.push(y);
        } else {
            result.push(moji);
        }

    }
    return result;
}
