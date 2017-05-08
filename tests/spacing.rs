// <https://github.com/vinta/pangu/blob/master/pangu_test.go>
// <https://github.com/vinta/pangu.js/blob/master/test/shared/test_core.js>

extern crate pangu;

#[test]
fn test_spacing_text() {
  assert_eq!(pangu::spacing("新八的構造成分有95%是眼鏡、3%是水、2%是垃圾"), "新八的構造成分有 95% 是眼鏡、3% 是水、2% 是垃圾");
  assert_eq!(pangu::spacing("所以,請問Jackey的鼻子有幾個?3.14個!"), "所以, 請問 Jackey 的鼻子有幾個? 3.14 個!");
  assert_eq!(pangu::spacing("JUST WE就是JUST WE，既不偉大也不卑微！"), "JUST WE 就是 JUST WE，既不偉大也不卑微！");
  assert_eq!(pangu::spacing("搭載MP3播放器，連續播放時數最長達到124小時的超強利刃……菊一文字RX-7!"), "搭載 MP3 播放器，連續播放時數最長達到 124 小時的超強利刃…… 菊一文字 RX-7!");
  assert_eq!(pangu::spacing("V"), "V");
}

#[test]
fn test_latin1_supplement() {
  assert_eq!(pangu::spacing("中文Ø漢字"), "中文 Ø 漢字");
  assert_eq!(pangu::spacing("中文 Ø 漢字"), "中文 Ø 漢字");
}

#[test]
fn test_general_punctuation() {
  assert_eq!(pangu::spacing("中文•漢字"), "中文 • 漢字");
  assert_eq!(pangu::spacing("中文 • 漢字"), "中文 • 漢字");
}

#[test]
fn test_number_forms() {
  assert_eq!(pangu::spacing("中文Ⅶ漢字"), "中文 Ⅶ 漢字");
  assert_eq!(pangu::spacing("中文 Ⅶ 漢字"), "中文 Ⅶ 漢字");
}

#[test]
fn test_cjk_radicals_supplement() {
  assert_eq!(pangu::spacing("abc⻤123"), "abc ⻤ 123");
  assert_eq!(pangu::spacing("abc ⻤ 123"), "abc ⻤ 123");
}

#[test]
fn test_kangxi_radicals() {
  assert_eq!(pangu::spacing("abc⾗123"), "abc ⾗ 123");
  assert_eq!(pangu::spacing("abc ⾗ 123"), "abc ⾗ 123");
}

#[test]
fn test_hiragana() {
  assert_eq!(pangu::spacing("abcあ123"), "abc あ 123");
  assert_eq!(pangu::spacing("abc あ 123"), "abc あ 123");
}

#[test]
fn test_katakana() {
  assert_eq!(pangu::spacing("abcア123"), "abc ア 123");
  assert_eq!(pangu::spacing("abc ア 123"), "abc ア 123");
}

#[test]
fn test_bopomofo() {
  assert_eq!(pangu::spacing("abcㄅ123"), "abc ㄅ 123");
  assert_eq!(pangu::spacing("abc ㄅ 123"), "abc ㄅ 123");
}

#[test]
fn test_enclosed_cjk_letters_and_months() {
  assert_eq!(pangu::spacing("abc㈱123"), "abc ㈱ 123");
  assert_eq!(pangu::spacing("abc ㈱ 123"), "abc ㈱ 123");
}

#[test]
fn test_cjk_unified_ideographs_extension_a() {
  assert_eq!(pangu::spacing("abc㐂123"), "abc 㐂 123");
  assert_eq!(pangu::spacing("abc 㐂 123"), "abc 㐂 123");
}

#[test]
fn test_cjk_unified_ideographs() {
  assert_eq!(pangu::spacing("abc丁123"), "abc 丁 123");
  assert_eq!(pangu::spacing("abc 丁 123"), "abc 丁 123");
}

#[test]
fn test_cjk_compatibility_ideographs() {
  assert_eq!(pangu::spacing("abc車123"), "abc 車 123");
  assert_eq!(pangu::spacing("abc 車 123"), "abc 車 123");
}

#[test]
fn test_tilde() {
  assert_eq!(pangu::spacing("前面~後面"), "前面~ 後面");
  assert_eq!(pangu::spacing("前面 ~ 後面"), "前面 ~ 後面");
  assert_eq!(pangu::spacing("前面~ 後面"), "前面~ 後面");
}

#[test]
fn test_back_quote() {
  assert_eq!(pangu::spacing("前面`後面"), "前面 ` 後面");
  assert_eq!(pangu::spacing("前面 ` 後面"), "前面 ` 後面");
  assert_eq!(pangu::spacing("前面` 後面"), "前面 ` 後面");
}

#[test]
fn test_exclamation_mark() {
  assert_eq!(pangu::spacing("前面!後面"), "前面! 後面");
  assert_eq!(pangu::spacing("前面 ! 後面"), "前面 ! 後面");
  assert_eq!(pangu::spacing("前面! 後面"), "前面! 後面");
}

#[test]
fn test_at() {
  // https://twitter.com/vinta
  assert_eq!(pangu::spacing("前面@vinta後面"), "前面 @vinta 後面");
  assert_eq!(pangu::spacing("前面 @vinta 後面"), "前面 @vinta 後面");

  // http://weibo.com/vintalines
  assert_eq!(pangu::spacing("前面@陳上進 後面"), "前面 @陳上進 後面");
  assert_eq!(pangu::spacing("前面 @陳上進 後面"), "前面 @陳上進 後面");
  assert_eq!(pangu::spacing("前面 @陳上進tail"), "前面 @陳上進 tail");
}

#[test]
fn test_hash() {
  assert_eq!(pangu::spacing("前面#H2G2後面"), "前面 #H2G2 後面");
  assert_eq!(pangu::spacing("前面#銀河便車指南 後面"), "前面 #銀河便車指南 後面");
  assert_eq!(pangu::spacing("前面#銀河便車指南tail"), "前面 #銀河便車指南 tail");
  assert_eq!(pangu::spacing("前面#銀河公車指南 #銀河拖吊車指南 後面"), "前面 #銀河公車指南 #銀河拖吊車指南 後面");
  assert_eq!(pangu::spacing("前面#H2G2#後面"), "前面 #H2G2# 後面");
  assert_eq!(pangu::spacing("前面#銀河閃電霹靂車指南#後面"), "前面 #銀河閃電霹靂車指南# 後面");
}

#[test]
fn test_dollar() {
  assert_eq!(pangu::spacing("前面$後面"), "前面 $ 後面");
  assert_eq!(pangu::spacing("前面 $ 後面"), "前面 $ 後面");
  assert_eq!(pangu::spacing("前面$100後面"), "前面 $100 後面");
}

#[test]
fn test_percent() {
  assert_eq!(pangu::spacing("前面%後面"), "前面 % 後面");
  assert_eq!(pangu::spacing("前面 % 後面"), "前面 % 後面");
  assert_eq!(pangu::spacing("前面100%後面"), "前面 100% 後面");
}

#[test]
fn test_carat() {
  assert_eq!(pangu::spacing("前面^後面"), "前面 ^ 後面");
  assert_eq!(pangu::spacing("前面 ^ 後面"), "前面 ^ 後面");
}

#[test]
fn test_ampersand() {
  assert_eq!(pangu::spacing("前面&後面"), "前面 & 後面");
  assert_eq!(pangu::spacing("前面 & 後面"), "前面 & 後面");
  assert_eq!(pangu::spacing("Vinta&Mollie"), "Vinta&Mollie");
  assert_eq!(pangu::spacing("Vinta&陳上進"), "Vinta & 陳上進");
  assert_eq!(pangu::spacing("陳上進&Vinta"), "陳上進 & Vinta");
  assert_eq!(pangu::spacing("得到一個A&B的結果"), "得到一個 A&B 的結果");
}

#[test]
fn test_asterisk() {
  assert_eq!(pangu::spacing("前面*後面"), "前面 * 後面");
  assert_eq!(pangu::spacing("前面 * 後面"), "前面 * 後面");
  assert_eq!(pangu::spacing("Vinta*Mollie"), "Vinta*Mollie");
  assert_eq!(pangu::spacing("Vinta*陳上進"), "Vinta * 陳上進");
  assert_eq!(pangu::spacing("陳上進*Vinta"), "陳上進 * Vinta");
  assert_eq!(pangu::spacing("得到一個A*B的結果"), "得到一個 A*B 的結果");
}

#[test]
fn test_parenthesis() {
  assert_eq!(pangu::spacing("前面(中文123漢字)後面"), "前面 (中文 123 漢字) 後面");
  assert_eq!(pangu::spacing("前面(中文123)後面"), "前面 (中文 123) 後面");
  assert_eq!(pangu::spacing("前面(123漢字)後面"), "前面 (123 漢字) 後面");
  assert_eq!(pangu::spacing("前面(中文123漢字) tail"), "前面 (中文 123 漢字) tail");
  assert_eq!(pangu::spacing("head (中文123漢字)後面"), "head (中文 123 漢字) 後面");
  assert_eq!(pangu::spacing("head (中文123漢字) tail"), "head (中文 123 漢字) tail");
}

#[test]
fn test_minus() {
  assert_eq!(pangu::spacing("前面-後面"), "前面 - 後面");
  assert_eq!(pangu::spacing("前面 - 後面"), "前面 - 後面");
  assert_eq!(pangu::spacing("Vinta-Mollie"), "Vinta-Mollie");
  assert_eq!(pangu::spacing("Vinta-陳上進"), "Vinta - 陳上進");
  assert_eq!(pangu::spacing("陳上進-Vinta"), "陳上進 - Vinta");
  assert_eq!(pangu::spacing("得到一個A-B的結果"), "得到一個 A-B 的結果");
}

#[test]
fn test_underscore() {
  assert_eq!(pangu::spacing("前面_後面"), "前面_後面");
  assert_eq!(pangu::spacing("前面 _ 後面"), "前面 _ 後面");
}

#[test]
fn test_plus() {
  assert_eq!(pangu::spacing("前面+後面"), "前面 + 後面");
  assert_eq!(pangu::spacing("前面 + 後面"), "前面 + 後面");
  assert_eq!(pangu::spacing("Vinta+Mollie"), "Vinta+Mollie");
  assert_eq!(pangu::spacing("Vinta+陳上進"), "Vinta + 陳上進");
  assert_eq!(pangu::spacing("陳上進+Vinta"), "陳上進 + Vinta");
  assert_eq!(pangu::spacing("得到一個A+B的結果"), "得到一個 A+B 的結果");
  assert_eq!(pangu::spacing("得到一個C++的結果"), "得到一個 C++ 的結果");
}

#[test]
fn test_equal() {
  assert_eq!(pangu::spacing("前面=後面"), "前面 = 後面");
  assert_eq!(pangu::spacing("前面 = 後面"), "前面 = 後面");
  assert_eq!(pangu::spacing("Vinta=Mollie"), "Vinta=Mollie");
  assert_eq!(pangu::spacing("Vinta=陳上進"), "Vinta = 陳上進");
  assert_eq!(pangu::spacing("陳上進=Vinta"), "陳上進 = Vinta");
  assert_eq!(pangu::spacing("得到一個A=B的結果"), "得到一個 A=B 的結果");
}

#[test]
fn test_brace() {
  assert_eq!(pangu::spacing("前面{中文123漢字}後面"), "前面 {中文 123 漢字} 後面");
  assert_eq!(pangu::spacing("前面{中文123}後面"), "前面 {中文 123} 後面");
  assert_eq!(pangu::spacing("前面{123漢字}後面"), "前面 {123 漢字} 後面");
  assert_eq!(pangu::spacing("前面{中文123漢字} tail"), "前面 {中文 123 漢字} tail");
  assert_eq!(pangu::spacing("head {中文123漢字}後面"), "head {中文 123 漢字} 後面");
  assert_eq!(pangu::spacing("head {中文123漢字} tail"), "head {中文 123 漢字} tail");
}

#[test]
fn test_bracket() {
  assert_eq!(pangu::spacing("前面[中文123漢字]後面"), "前面 [中文 123 漢字] 後面");
  assert_eq!(pangu::spacing("前面[中文123]後面"), "前面 [中文 123] 後面");
  assert_eq!(pangu::spacing("前面[123漢字]後面"), "前面 [123 漢字] 後面");
  assert_eq!(pangu::spacing("前面[中文123漢字] tail"), "前面 [中文 123 漢字] tail");
  assert_eq!(pangu::spacing("head [中文123漢字]後面"), "head [中文 123 漢字] 後面");
  assert_eq!(pangu::spacing("head [中文123漢字] tail"), "head [中文 123 漢字] tail");
}

#[test]
fn test_pipe() {
  assert_eq!(pangu::spacing("前面|後面"), "前面 | 後面");
  assert_eq!(pangu::spacing("前面 | 後面"), "前面 | 後面");
  assert_eq!(pangu::spacing("Vinta|Mollie"), "Vinta|Mollie");
  assert_eq!(pangu::spacing("Vinta|陳上進"), "Vinta | 陳上進");
  assert_eq!(pangu::spacing("陳上進|Vinta"), "陳上進 | Vinta");
  assert_eq!(pangu::spacing("得到一個A|B的結果"), "得到一個 A|B 的結果");
}

#[test]
fn test_backslash() {
  assert_eq!(pangu::spacing(r"前面\後面"), r"前面 \ 後面");
}

#[test]
fn test_colon() {
  assert_eq!(pangu::spacing("前面:後面"), "前面: 後面");
  assert_eq!(pangu::spacing("前面 : 後面"), "前面 : 後面");
  assert_eq!(pangu::spacing("前面: 後面"), "前面: 後面");
}

#[test]
fn test_semicolon() {
  assert_eq!(pangu::spacing("前面;後面"), "前面; 後面");
  assert_eq!(pangu::spacing("前面 ; 後面"), "前面 ; 後面");
  assert_eq!(pangu::spacing("前面; 後面"), "前面; 後面");
}

#[test]
fn test_quote() {
  assert_eq!(pangu::spacing("前面\"中文123漢字\"後面"), "前面 \"中文 123 漢字\" 後面");
  assert_eq!(pangu::spacing("前面\"中文123\"後面"), "前面 \"中文 123\" 後面");
  assert_eq!(pangu::spacing("前面\"123漢字\"後面"), "前面 \"123 漢字\" 後面");
  assert_eq!(pangu::spacing("前面\"中文123漢字\" tail"), "前面 \"中文 123 漢字\" tail");
  assert_eq!(pangu::spacing("head \"中文123漢字\"後面"), "head \"中文 123 漢字\" 後面");
  assert_eq!(pangu::spacing("head \"中文123漢字\" tail"), "head \"中文 123 漢字\" tail");

  // \u201c and \u201d
  assert_eq!(pangu::spacing("前面“中文123漢字”後面"), "前面 “中文 123 漢字” 後面");
}

#[test]
fn test_single_quote() {
  assert_eq!(pangu::spacing("前面'中文123漢字'後面"), "前面 '中文 123 漢字' 後面");
  assert_eq!(pangu::spacing("前面'中文123'後面"), "前面 '中文 123' 後面");
  assert_eq!(pangu::spacing("前面'123漢字'後面"), "前面 '123 漢字' 後面");
  assert_eq!(pangu::spacing("前面'中文123漢字' tail"), "前面 '中文 123 漢字' tail");
  assert_eq!(pangu::spacing("head '中文123漢字'後面"), "head '中文 123 漢字' 後面");
  assert_eq!(pangu::spacing("head '中文123漢字' tail"), "head '中文 123 漢字' tail");
  assert_eq!(pangu::spacing("陳上進 likes 林依諾's status."), "陳上進 likes 林依諾's status.");
}

#[test]
fn test_less_than() {
  assert_eq!(pangu::spacing("前面<後面"), "前面 < 後面");
  assert_eq!(pangu::spacing("前面 < 後面"), "前面 < 後面");
  assert_eq!(pangu::spacing("Vinta<Mollie"), "Vinta<Mollie");
  assert_eq!(pangu::spacing("Vinta<陳上進"), "Vinta < 陳上進");
  assert_eq!(pangu::spacing("陳上進<Vinta"), "陳上進 < Vinta");
  assert_eq!(pangu::spacing("得到一個A<B的結果"), "得到一個 A<B 的結果");
  assert_eq!(pangu::spacing("前面<中文123漢字>後面"), "前面 <中文 123 漢字> 後面");
  assert_eq!(pangu::spacing("前面<中文123>後面"), "前面 <中文 123> 後面");
  assert_eq!(pangu::spacing("前面<123漢字>後面"), "前面 <123 漢字> 後面");
  assert_eq!(pangu::spacing("前面<中文123漢字> tail"), "前面 <中文 123 漢字> tail");
  assert_eq!(pangu::spacing("head <中文123漢字>後面"), "head <中文 123 漢字> 後面");
  assert_eq!(pangu::spacing("head <中文123漢字> tail"), "head <中文 123 漢字> tail");
}

#[test]
fn test_comma() {
  assert_eq!(pangu::spacing("前面,後面"), "前面, 後面");
  assert_eq!(pangu::spacing("前面 , 後面"), "前面 , 後面");
  assert_eq!(pangu::spacing("前面, 後面"), "前面, 後面");
}

#[test]
fn test_greater_than() {
  assert_eq!(pangu::spacing("前面>後面"), "前面 > 後面");
  assert_eq!(pangu::spacing("前面 > 後面"), "前面 > 後面");
  assert_eq!(pangu::spacing("Vinta>Mollie"), "Vinta>Mollie");
  assert_eq!(pangu::spacing("Vinta>陳上進"), "Vinta > 陳上進");
  assert_eq!(pangu::spacing("陳上進>Vinta"), "陳上進 > Vinta");
  assert_eq!(pangu::spacing("得到一個A>B的結果"), "得到一個 A>B 的結果");
}

#[test]
fn test_period() {
  assert_eq!(pangu::spacing("前面.後面"), "前面. 後面");
  assert_eq!(pangu::spacing("前面 . 後面"), "前面 . 後面");
  assert_eq!(pangu::spacing("前面. 後面"), "前面. 後面");

  // \u2026
  assert_eq!(pangu::spacing("前面…後面"), "前面… 後面");
  assert_eq!(pangu::spacing("前面……後面"), "前面…… 後面");

  // \u2027
  assert_eq!(pangu::spacing("前面‧後面"), "前面 ‧ 後面");
}

#[test]
fn test_question_mark() {
  assert_eq!(pangu::spacing("前面?後面"), "前面? 後面");
  assert_eq!(pangu::spacing("前面 ? 後面"), "前面 ? 後面");
  assert_eq!(pangu::spacing("前面? 後面"), "前面? 後面");
}

#[test]
fn test_slash() {
  assert_eq!(pangu::spacing("前面/後面"), "前面 / 後面");
  assert_eq!(pangu::spacing("前面 / 後面"), "前面 / 後面");
  assert_eq!(pangu::spacing("Vinta/Mollie"), "Vinta/Mollie");
  assert_eq!(pangu::spacing("Vinta/陳上進"), "Vinta / 陳上進");
  assert_eq!(pangu::spacing("陳上進/Vinta"), "陳上進 / Vinta");
  assert_eq!(pangu::spacing("得到一個A/B的結果"), "得到一個 A/B 的結果");
}
