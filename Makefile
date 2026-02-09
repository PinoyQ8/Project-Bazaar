# 🏛️ Project Bazaar | Command Center

all: test build

test:
	@echo "🧪 Running Trust Logic Test Suite..."
	cargo test

build:
	@echo "🔨 Compiling WASM..."
	cargo build --target wasm32-unknown-unknown --release

deploy: build
	@echo "🚀 Deploying..."
	./deploy.sh

clean:
	cargo clean
	rm -f contract_id.txt