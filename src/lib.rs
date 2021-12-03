mod extractors_bindings;
mod extractors;

#[cfg(test)]
mod test_extractors;

extern crate cpython;

use extractors_bindings::*;

use cpython::{py_module_initializer, py_fn};

py_module_initializer!(py_unicode_segmentation, |py, m| {
    m.add(py, "__doc__", "Expose functions to extract graphemes, sentences and words from strings or string batches")?;

    m.add(py, "extract_graphemes_from_text", py_fn!(py, py_extract_graphemes_from_text(text: &str)))?;
    m.add(py, "extract_graphemes", py_fn!(py, py_extract_graphemes(text_batch: Vec<String>)))?;

    m.add(py, "extract_sentences_from_text", py_fn!(py, py_extract_sentences_from_text(text: &str)))?;
    m.add(py, "extract_sentences", py_fn!(py, py_extract_sentences(text_batch: Vec<String>)))?;

    m.add(py, "extract_words_from_text", py_fn!(py, py_extract_words_from_text(text: &str)))?;
    m.add(py, "extract_words", py_fn!(py, py_extract_words(text_batch: Vec<String>)))?;

    Ok(())
});

