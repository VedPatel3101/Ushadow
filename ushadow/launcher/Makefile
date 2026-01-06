# Ushadow Launcher Makefile

.PHONY: release dev build build-debug clean version help

# Default target
help:
	@echo "Ushadow Launcher"
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@echo "  release      Interactive release workflow (prompts for version, platforms)"
	@echo "  dev          Start development server"
	@echo "  build        Build for current platform (release)"
	@echo "  build-debug  Build for current platform (debug)"
	@echo "  version      Show current version"
	@echo "  clean        Clean build artifacts"
	@echo "  help         Show this help"

# Interactive release - the main workflow
release:
	@./scripts/interactive-release.sh

# Development
dev:
	npm run dev

# Build for current platform
build:
	npm run build

build-debug:
	npm run build:debug

# Version management
version:
	@./scripts/version.sh get

# Clean build artifacts
clean:
	rm -rf src-tauri/target/release/bundle
	rm -rf src-tauri/target/debug/bundle
	rm -rf releases/
	@echo "Build artifacts cleaned"
