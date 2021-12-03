use crate::extractors::*;

/// Graphemes

#[test]
fn extract_graphemes_from_text_should_return_graphemes() {
    let input = "a̐éö̲abc12\r\n";

    let expected = ["a̐", "é", "ö̲", "a", "b", "c", "1", "2", "\r\n"].to_vec();
    let result = extract_graphemes_from_text(input);

    assert_eq!(result, expected);
}

#[test]
fn extract_graphemes_from_text_should_return_void_array_for_empty_string() {
    let input = "";

    let expected: Vec<String> = [].to_vec();
    let result = extract_graphemes_from_text(input);

    assert_eq!(result, expected);
}

#[test]
fn extract_graphemes_should_return_graphemes_for_each_entry() {
    let input = ["a̐éö̲abc".to_owned(), "12\r\n".to_owned()].to_vec();

    let expected = [
        ["a̐", "é", "ö̲", "a", "b", "c"].to_vec(),
        ["1", "2", "\r\n"].to_vec()
    ].to_vec();

    let result = extract_graphemes(input);

    assert_eq!(result, expected);
}

/// Sentences

#[test]
fn extract_sentences_from_text_should_return_sentences() {
    let input = "Rã-de-olhos-dourados (Agalychnis annae) em Heredia, Costa Rica. A espécie por muito tempo foi considerada parte da família das pererecas (Hylidae) e conhecida apenas na Costa Rica. A espécie já foi comum nas florestas tropicais da Cordilheira de Tilaran, no Vale Central e na parte nordeste da Cordilheira de Talamanca em altitudes entre 780 e 1 650 metros, mas desde então desapareceu de Monteverde e Tapanti, entre outros.";

    let expected = [
        "Rã-de-olhos-dourados (Agalychnis annae) em Heredia, Costa Rica. ",
        "A espécie por muito tempo foi considerada parte da família das pererecas (Hylidae) e conhecida apenas na Costa Rica. ",
        "A espécie já foi comum nas florestas tropicais da Cordilheira de Tilaran, no Vale Central e na parte nordeste da Cordilheira de Talamanca em altitudes entre 780 e 1 650 metros, mas desde então desapareceu de Monteverde e Tapanti, entre outros."
    ].to_vec();
    let result = extract_sentences_from_text(input);

    assert_eq!(result, expected);
}

#[test]
fn extract_sentences_from_text_should_return_void_array_for_empty_string() {
    let input = "";

    let expected: Vec<String> = [].to_vec();
    let result = extract_sentences_from_text(input);

    assert_eq!(result, expected);
}

#[test]
fn extract_sentences_from_should_return_graphemes_for_each_entry() {
    let input = [
        "Rã-de-olhos-dourados (Agalychnis annae) em Heredia, Costa Rica. A espécie por muito tempo foi considerada parte da família das pererecas (Hylidae) e conhecida apenas na Costa Rica.".to_owned(),
        "Natural Science Comprehensive-Junior High School Science Development Stage: 25% Natural Science Development Stage: 00%".to_owned()
    ].to_vec();

    let expected = [
        [
            "Rã-de-olhos-dourados (Agalychnis annae) em Heredia, Costa Rica. ",
            "A espécie por muito tempo foi considerada parte da família das pererecas (Hylidae) e conhecida apenas na Costa Rica."
        ].to_vec(),
        [
            "Natural Science Comprehensive-Junior High School Science Development Stage: 25% Natural Science Development Stage: 00%"
        ].to_vec()
    ].to_vec();

    let result = extract_sentences(input);

    assert_eq!(result, expected);
}

/// Words

#[test]
fn extract_words_from_text_should_return_sentences() {
    let input = "This is an opening for Information/Techonology&Services";

    let expected = ["This", "is", "an", "opening", "for", "Information", "Techonology", "Services"].to_vec();
    let result = extract_words_from_text(input);

    assert_eq!(result, expected);
}

#[test]
fn extract_words_from_text_should_return_void_array_for_empty_string() {
    let input = "";

    let expected: Vec<String> = [].to_vec();
    let result = extract_words_from_text(input);

    assert_eq!(result, expected);
}

#[test]
fn extract_words_from_should_return_graphemes_for_each_entry() {
    let input = [
            "This is an opening for".to_owned(),
            "Information/Techonology&Services".to_owned()
        ].to_vec();

    let expected = [
        ["This", "is", "an", "opening", "for"].to_vec(),
        ["Information", "Techonology", "Services"].to_vec()
    ].to_vec();

    let result = extract_words(input);

    assert_eq!(result, expected);
}