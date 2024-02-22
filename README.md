# LLI: Rust Library for Tokenization with OpenAI Models
The LLI library offers a Rust interface for tokenizing text, specifically tailored for use with OpenAI models, through a seamless integration with Go applications. Utilizing the tiktoken_rs library, LLI provides efficient text tokenization capabilities, making it easier to preprocess text for AI-driven analysis and processing.

## Features
- Text tokenization suitable for OpenAI models.
- Integration support for Go applications.
- Token count operations for text input.
- Support for various tokenizer models including Cl100kBase, P50kBase, R50kBase, P50kEdit, and Gpt2.
## Prerequisites
Ensure you have Rust and Go installed on your system. This library is not supported on Windows platforms due to specific dependencies.

- Rust (latest stable version)
- Go (version 1.15 or higher)
Building the Library
To build the Rust library:

```console
❯ go generate
//go:generate bash -c "cd lli && cargo build --release"
```
This command compiles the Rust code into a static library located at `target/release/liblli.a`, which is then used by the Go wrapper.

