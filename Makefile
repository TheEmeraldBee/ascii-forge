test:
	cargo test


VERSION_FILE := Cargo.toml

publish:
	sed -i -r "s/version=\"0\.0\.0\"/version=\"${VERSION}\"/g" $(VERSION_FILE) \
	  && cargo publish --allow-dirty
