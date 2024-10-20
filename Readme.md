# Byte Pair Encoding (BPE) Implementation in Rust

This project implements a simple Byte Pair Encoding (BPE) algorithm in Rust. BPE is commonly used for tokenizing text in natural language processing (NLP) tasks.

## Usage

To run the program, use the following command:

```bash
cargo run
```

```rust
    let text = std::fs::read_to_string("src/text.txt").unwrap();

    let (byte_pair_encoding_tokens, byte_pair_encoding_tokens_ids) = tokenizer::byte_pair_encofing(&text, 10);
    println!("Tokens: {:?}", byte_pair_encoding_tokens);
    println!("Token IDs: {:?}", byte_pair_encoding_tokens_ids);

    let most_common_token_ids = tokenizer::most_common_token_ids(&byte_pair_encoding_tokens_ids, 10);
    println!("Most common tokens: {:?}", most_common_token_ids);

    let most_common_tokens = tokenizer::most_common_tokens(&byte_pair_encoding_tokens, &byte_pair_encoding_tokens_ids, 10);
    println!("Most common tokens: {:?}", most_common_tokens);
```

## Example

The following is an example of the output of the program:

```bash
Tokens: ["merhaba", "ben", "bir", "tokenizer", "uygulamasıyım.", "merhaba", "ben", "rust", "programlama", "dili", "ile", "yazılmış", "bir", "uygulamayım."]
Token IDs: [1, 2, 3, 4, 5, 1, 2, 6, 7, 8, 9, 10, 3, 11]
Most common tokens: [("2", 2), ("3", 2), ("1", 2), ("10", 1), ("7", 1), ("6", 1), ("9", 1), ("8", 1), ("4", 1), ("11", 1)]
Most common tokens: [("bir", 2), ("ben", 2), ("tokenizer", 2), ("ben", 1), ("rust", 1), ("uygulamasıyım.", 1), ("programlama", 1), ("merhaba", 1), ("dili", 1), ("ile", 1)]
```

## Author

- [Muhtalip Dede](https://github.com/muhtalipdede)
