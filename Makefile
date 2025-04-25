all:
	cargo build --release

test:
	cargo test

coverage:
	cargo tarpaulin --skip-clean --exclude-files src/utils/ui/*

ext1:
	@if [ "$(word 2,$(MAKECMDGOALS))" = "" ] || [ "$(word 3,$(MAKECMDGOALS))" = "" ]; then \
		echo "Usage: make ext1 <nrows> <ncols>"; \
	elif ! echo "$(word 2,$(MAKECMDGOALS)) $(word 3,$(MAKECMDGOALS))" | grep -E '^[0-9]+[[:space:]][0-9]+$$' > /dev/null; then \
		echo "Error: Arguments must be two integers."; \
	else \
		cargo build --release; \
		./target/release/spreadsheet $(word 2,$(MAKECMDGOALS)) $(word 3,$(MAKECMDGOALS)) --ui; \
	fi


%:
	@:

docs:
	pdflatex report.tex
	cargo doc --no-deps --document-private-items