target/py_unicode_segmentation.so:
	cargo build --release
	cp target/release/libpy_unicode_segmentation.so target/py_unicode_segmentation.so

clean:
	cargo clean