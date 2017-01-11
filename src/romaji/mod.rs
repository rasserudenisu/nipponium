pub mod romaji {

    use regex::Regex;

    const ROMAJI: [&'static str; 136] =  ["@", "a", "i", "u", "e", "o", "ka", "ki", "ku", "ke", "ko", "ga", "gi", "gu", "ge", "go", "sa", "shi", "su", "se", "so", "za", "ji", "zu", "ze", "zo", "ta", "chi", "tsu", "te", "to", "da", "ji", "zu", "de", "do", "na", "ni", "nu", "ne", "no", "ha", "hi", "fu", "he", "ho", "ba", "bi", "bu", "be", "bo", "pa", "pi", "pu", "pe", "po", "ma", "mi", "mu", "me", "mo", "ya", "", "yu", "", "yo", "ra", "ri", "ru", "re", "ro", "wa", "wi", "", "we", "wo", "n", "", "", "", "", "cha", "chi", "chu", "chu", "cho", "sha", "shi", "shu", "she", "sho", "ja", "ji", "ju", "je", "jo", "kya", "kyi", "kyu", "kye", "kyo", "nya", "nyi", "nyu", "nye", "nyo", "hya", "hyi", "hyu", "hye", "hyo", "mya", "myi", "myu", "mye", "myo", "rya", "ryi", "ryu", "rye", "ryo", "gya", "gyi", "gyu", "gye", "gyo", "bya", "byi", "byu", "bye", "byo", "pya", "pyi", "pyu", "pye", "pyo"];
    const HIRAGANA: [&'static str; 136] = ["っ", "あ", "い", "う", "え", "お", "か", "き", "く", "け", "こ", "が", "ぎ", "ぐ", "げ", "ご", "さ", "し", "す", "せ", "そ", "ざ", "じ", "ず", "ぜ", "ぞ", "た", "ち", "つ", "て", "と", "だ", "ぢ", "づ", "で", "ど", "な", "に", "ぬ", "ね", "の", "は", "ひ", "ふ", "へ", "ほ", "ば", "び", "ぶ", "べ", "ぼ", "ぱ", "ぴ", "ぷ", "ぺ", "ぽ", "ま", "み", "む", "め", "も", "や", "", "ゆ", "", "よ", "ら", "り", "る", "れ", "ろ", "わ", "ゐ", "", "ゑ", "を", "ん", "", "", "", "", "ちゃ", "ち", "ちゅ", "ちぇ", "ちょ", "しゃ", "し", "しゅ", "しぇ", "しょ", "じゃ", "じ", "じゅ", "じぇ", "じょ", "きゃ", "きぃ", "きゅ", "きぇ", "きょ", "にゃ", "にぃ", "にゅ", "にぇ", "にょ", "ひゃ", "ひぃ", "ひゅ", "ひぇ", "ひょ", "みゃ", "みぃ", "みゅ", "みぇ", "みょ", "りゃ", "りぃ", "りゅ", "りぇ", "りょ", "ぎゃ", "ぎぃ", "ぎゅ", "ぎぇ", "ぎょ", "びゃ", "びぃ", "びゅ", "びぇ", "びょ", "ぴゃ", "ぴぃ", "ぴゅ", "ぴぇ", "ぴょ"];
    const SOKUON: [&'static str; 16] = ["r", "w", "y", "p", "b", "f", "h", "g", "k", "j", "d", "m", "t", "z", "s", "c"];

    pub fn from_romaji(text: &str) -> String {
        let re = Regex::new("(tsu)|([ztmdkghfbpyrwn](ya|yu|yo))|((sh|ch)?([aeiou]))|(([sztmdjkghfbpyrwn])?([aeioun]))|([rwnypbfhgkjdmtzsc])").unwrap();
        let mut result = "".to_string();
        for captures in re.captures_iter(text) {
            let value = captures.get(0).unwrap().as_str();
            if SOKUON.contains(&value) {
                result.push_str("っ");
            } else {
                let index = ROMAJI.iter().position(|&x| x == value).unwrap();
                result.push_str(HIRAGANA[index]);
            }
        }
        return result;
    }

    pub fn to_romaji(text: &str) -> String {
        let mut result = "".to_string();
        for moji in text.chars() {
            let index = HIRAGANA.iter().position(|&x| x == moji.to_string()).unwrap();
            result.push_str(ROMAJI[index]);
        }
        if result.contains("@") {
            let re = Regex::new("(@.)").unwrap();
            for captures in re.find_iter(&result.clone()) {
                let val: String = result[captures.start()+1..captures.end()].to_string();
                result.remove(captures.start());
                result.insert(captures.start(), val.chars().next().unwrap());
            }
        }
        return result;
    }

}
