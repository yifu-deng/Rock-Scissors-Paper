format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

invoke:
	cargo lambda invoke --remote \
  		--data-ascii '' \
			--output-format json \
			RSP

all: format lint test run
