use crate::extractors;

use cpython::{PyResult, Python};

pub fn py_extract_graphemes_from_text(_py: Python, text: &str) -> PyResult<Vec<String>> {
    let result = extractors::extract_graphemes_from_text(text);
    Ok(result)
}

pub fn py_extract_graphemes(_py: Python, text_batch: Vec<String>) -> PyResult<Vec<Vec<String>>> {
    let result = extractors::extract_graphemes(text_batch);
    Ok(result)
}


pub fn py_extract_sentences_from_text(_py: Python, text: &str) -> PyResult<Vec<String>> {
    let result = extractors::extract_sentences_from_text(text);
    Ok(result)
}

pub fn py_extract_sentences(_py: Python, text_batch: Vec<String>) -> PyResult<Vec<Vec<String>>> {
    let result = extractors::extract_sentences(text_batch);
    Ok(result)
}


pub fn py_extract_words_from_text(_py: Python, text: &str) -> PyResult<Vec<String>> {
    let result = extractors::extract_words_from_text(text);
    Ok(result)
}

pub fn py_extract_words(_py: Python, text_batch: Vec<String>) -> PyResult<Vec<Vec<String>>> {
    let result = extractors::extract_words(text_batch);
    Ok(result)
}
