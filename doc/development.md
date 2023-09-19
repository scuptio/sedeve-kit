
## Prerequisite


We utilize [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) as our parser generator tool for the TLA+ toolbox output dot file.

### Install tree-sitter-cli:

    cargo install tree-sitter-cli

## Build the Project

### Generate Tree-Sitter parser

    cd src/tree-sitter-dot
    tree-sitter generate
    
    cd src/tree-sitter-label-tla
    tree-sitter generate

    cd src/tree-sitter-action
    tree-sitter generate
    
    cd src/tree-sitter-sql
    tree-sitter generate

### Build Rust project

    cargo build
