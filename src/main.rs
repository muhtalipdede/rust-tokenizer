mod tokenizer;

fn main() {
    let text = std::fs::read_to_string("src/text.txt").unwrap();

    let (byte_pair_encoding_tokens, byte_pair_encoding_tokens_ids) = tokenizer::byte_pair_encofing(&text, 10);
    println!("Tokens: {:?}", byte_pair_encoding_tokens);
    println!("Token IDs: {:?}", byte_pair_encoding_tokens_ids);

    let most_common_token_ids = tokenizer::most_common_token_ids(&byte_pair_encoding_tokens_ids, 10);
    println!("Most common tokens: {:?}", most_common_token_ids);

    let most_common_tokens = tokenizer::most_common_tokens(&byte_pair_encoding_tokens, &byte_pair_encoding_tokens_ids, 10);
    println!("Most common tokens: {:?}", most_common_tokens);
}
