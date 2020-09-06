.DEFAULT_GOAL := check

init:
	@echo "Initialising the project"

start:
	@echo "🏃‍♀️ Starting project"
	@cargo run example/random-names

check: --pre_check test build
	@echo "✅"
	@cargo build

docs:
	@doctoc README.md
	@cargo docs
	@echo "📚 Documentation ready!"

clean:
	@echo "🛁 Cleaning..."
	@cargo clean

clean_all: clean
	@echo "🧨 Clean all"

test:
	@echo "Testing..."
	@cargo test

build:
	@echo "👩‍🏭 Building..."
	@cargo build

release_patch: release

release_minor: check
	@.scripts/finish-release minor

release_major: check
	@.scripts/finish-release major

release: check
	@.scripts/finish-release patch

--pre_check:
	@echo "👩‍🏭 Pre-check here!"
