.PHONY: publish

MSG ?=
VERSION ?=

publish:
ifeq ($(VERSION),)
	@echo "ERROR: No version specified for publish."
	@exit 1
endif
ifeq ($(MSG),)
	@echo "ERROR: No commit MSG specified for push."
	@exit 1
endif
	@echo "Updating versions to $(VERSION)..."
	sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' ./Cargo.toml && rm -f ./Cargo.toml.bak

	git add -A
	git commit -m "$(VERSION)" || true
	
	@echo "Pushing code to remote..."
	git push origin main
	
	@echo "Publishing rust crate..."
	cargo publish
	
	@echo "Publish task complete"
  
