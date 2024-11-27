# Makefile for building the Rust project

# Project variables
PROJECT_NAME := expr-rs

# Build settings
BUILD_DIR := target/release
SRC_DIR := src

COMPLETION_DIR := $(shell brew --prefix)/share/zsh-completions

.PHONY: all build clean

# Default target
all: build

# Build the project in release mode
build:
	@echo "Building the release version of $(PROJECT_NAME)..."
	cargo build --release
	@echo "Build completed. Binary available at $(BUILD_DIR)/$(PROJECT_NAME)"

# Clean the project
clean:
	@echo "Cleaning project..."
	cargo clean
	@echo "Project cleaned."

# Installation might not be commonly used in Rust projects directly,
# but here's a simple example to copy the binary to /usr/local/bin (requires sudo)
install:
	@echo "Installing $(PROJECT_NAME)..."
	sudo cp $(BUILD_DIR)/$(PROJECT_NAME) /usr/local/bin/
	@echo "Installation completed."

install_completion:
	@echo "Installing $(PROJECT_NAME) completion ..."
	$(PROJECT_NAME) completion --shell zsh > $(COMPLETION_DIR)/_$(PROJECT_NAME)
	@echo "Installation completed."
	@echo "Please exec this commands:"
	@echo '	exec $$SHELL -l'

test:
	@echo "Test $(PROJECT_NAME)..."
	cargo test

