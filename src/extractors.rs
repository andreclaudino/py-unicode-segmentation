extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn extract_graphemes_from_text(text: &str) -> Vec<String> {
    let graphemes = UnicodeSegmentation::graphemes(text, true);
    graphemes.map(|grapheme| {
        grapheme.to_owned()
    })
    .collect()
}

pub(crate) fn extract_graphemes(text_batch: Vec<String>) -> Vec<Vec<String>> {
    text_batch.iter().map(|text|{
        extract_graphemes_from_text(text)
    })
    .collect()
}

pub(crate) fn extract_sentences_from_text(text: &str) -> Vec<String> {
    let sentences = UnicodeSegmentation::unicode_sentences(text);
    sentences.map(|sentence|{
        sentence.to_owned()
    })
    .collect()
}

pub(crate) fn extract_sentences(text_batch: Vec<String>) -> Vec<Vec<String>> {
    text_batch.iter().map(|text|{
        extract_sentences_from_text(text)
    })
    .collect()
}


pub(crate) fn extract_words_from_text(text: &str) -> Vec<String> {
    let words = text.unicode_words();
    words.map(|word| {
        word.to_owned()
    })
    .collect()
}

pub(crate) fn extract_words(text_batch: Vec<String>) -> Vec<Vec<String>> {
    text_batch.iter().map(|text|{
        extract_words_from_text(text)
    })
    .collect()
}