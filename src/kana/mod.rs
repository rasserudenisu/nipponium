pub mod kana {

    use std::char;

    const HIRAGANA: (u32, u32) = (0x3041, 0x3096);
    const KATAKANA: (u32, u32) = (0x30A1, 0x30F6);
    const KANJI:    (u32, u32) = (0x4E00, 0x9FAF);

    /// Transmutes katakana to hiragana.
    ///
    /// # Examples
    ///
    /// ```
    /// use nipponium::kana::kana;
    ///
    /// kana::to_hiragana("アイウエオ");
    /// ```
    pub fn to_hiragana(text: &str) -> String {
        let dir: i32 = -96;
        return kana_converter(KATAKANA, text, dir);
    }

    /// Transmutes hiragana to katakana.
    ///
    /// # Examples
    ///
    /// ```
    /// use nipponium::kana::kana;
    ///
    /// kana::to_katakana("あいうえお");
    /// ```
    pub fn to_katakana(text: &str) -> String {
        return kana_converter(HIRAGANA, text, (96 as i32));
    }

    /// Checks whether the entire string is hiragana or not.
    pub fn is_hiragana(text: &str) -> bool {
        for moji in text.chars() {
            if !in_range(HIRAGANA, &moji) {
                return false;
            }
        }
        return true;
    }

    /// Checks whether the entire string is katakana or not.
    pub fn is_katakana(text: &str) -> bool {
        for moji in text.chars() {
            if !in_range(KATAKANA, &moji) {
                return false;
            }
        }
        return true;
    }

    /// Checks whether the entire string is kanji or not.
    pub fn is_kanji(text: &str) -> bool {
        for moji in text.chars() {
            if !in_range(KANJI, &moji) {
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

}
