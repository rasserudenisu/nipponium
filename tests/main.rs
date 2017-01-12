extern crate nipponium;

#[cfg(test)]
mod tests {
    use nipponium::kana;
    use nipponium::romaji;

    #[test]
    fn to_hiragana() {
        assert_eq!("あいうえお", kana::to_hiragana("アイウエオ"));
    }

    #[test]
    fn to_katakana() {
        assert_eq!("タベル", kana::to_katakana("たべる"));
    }

    #[test]
    fn from_romaji() {
        assert_eq!("ねこがかわいい", romaji::from_romaji("nekogakawaii"));
        assert_eq!("だいがくいんはすっごくたいへんだった", romaji::from_romaji("daigakuinhasuggokutaihendatta"));
    }

    #[test]
    fn is_xyz() {
        assert_eq!(true, kana::is_hiragana("ねこがかわいい"));
        assert_eq!(true, kana::is_katakana("ネコガカワイイ"));
        assert_eq!(true, kana::is_kanji("勉強"));
    }

    #[test]
    fn to_romaji() {
        assert_eq!("nekowotabetemitai", romaji::to_romaji("ねこをたべてみたい"));
        assert_eq!("daigakuinhasuggokutaihendatta", romaji::to_romaji("だいがくいんはすっごくたいへんだった"));
    }

}
