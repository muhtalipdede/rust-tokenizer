use std::collections::HashMap;

pub fn byte_pair_encofing(text: &str, k: usize) -> (Vec<String>, Vec<usize>) {
    let mut tokens: Vec<String> = text
        .split_whitespace()
        .map(|word| word.to_string())
        .collect();

    let mut vocab: HashMap<String, usize> = HashMap::new();
    let mut pairs: HashMap<(String, String), usize> = HashMap::new();
    let mut token_id_counter = 0;

    // İlk karakter çiftlerini oluştur
    for token in tokens.iter() {
        let chars: Vec<String> = token.chars().map(|c| c.to_string()).collect();
        for i in 0..chars.len() - 1 {
            let pair = (chars[i].clone(), chars[i + 1].clone());
            *pairs.entry(pair).or_insert(0) += 1;
        }
    }

    for _ in 0..k {
        // En sık görülen karakter çiftini bul
        let mut max_pair = ("".to_string(), "".to_string());
        let mut max_count = 0;
        for (pair, count) in pairs.iter() {
            if *count > max_count {
                max_count = *count;
                max_pair = pair.clone();
            }
        }

        // Eğer çift bulunamadıysa işlemi bitir
        if max_count == 0 {
            break;
        }

        // En sık görülen çifti birleştir
        let new_token = format!("{}{}", max_pair.0, max_pair.1);

        // Yeni token'ı vocab'e ekle ve ID ver
        if !vocab.contains_key(&new_token) {
            vocab.insert(new_token.clone(), token_id_counter);
            token_id_counter += 1;
        }

        // Tüm tokenları yeni çift ile güncelle
        tokens = tokens
            .iter()
            .map(|token| token.replace(&format!("{}{}", max_pair.0, max_pair.1), &new_token))
            .collect();

        // Çiftlerin sıklığını tekrar hesapla
        let mut new_pairs: HashMap<(String, String), usize> = HashMap::new();
        for token in tokens.iter() {
            let chars: Vec<String> = token.chars().map(|c| c.to_string()).collect();
            for i in 0..chars.len() - 1 {
                let pair = (chars[i].clone(), chars[i + 1].clone());
                *new_pairs.entry(pair).or_insert(0) += 1;
            }
        }
        pairs = new_pairs;
    }

    // Son tokenların ID'lerini al
    let mut token_ids: Vec<usize> = Vec::new();
    for token in &tokens {
        if let Some(id) = vocab.get(token) {
            token_ids.push(*id);
        } else {
            // Eğer token vocab'te yoksa, onu ekle ve ID ver
            vocab.insert(token.clone(), token_id_counter);
            token_ids.push(token_id_counter);
            token_id_counter += 1;
        }
    }

    (tokens, token_ids)
}

pub fn most_common_token_ids(token_ids: &Vec<usize>, n: usize) -> Vec<(String, usize)> {
    let mut token_counts: HashMap<usize, usize> = HashMap::new();
    for token_id in token_ids.iter() {
        *token_counts.entry(*token_id).or_insert(0) += 1;
    }

    let mut token_counts_vec: Vec<(usize, usize)> = token_counts.into_iter().collect();
    token_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let mut most_common_token_ids: Vec<(String, usize)> = Vec::new();
    for (token_id, count) in token_counts_vec.iter().take(n) {
        most_common_token_ids.push((token_id.to_string(), *count));
    }

    most_common_token_ids
}

pub fn most_common_tokens(tokens: &Vec<String>, token_ids: &Vec<usize>, n: usize) -> Vec<(String, usize)> {
    let mut token_counts: HashMap<usize, usize> = HashMap::new();
    for token_id in token_ids.iter() {
        *token_counts.entry(*token_id).or_insert(0) += 1;
    }

    let mut token_counts_vec: Vec<(usize, usize)> = token_counts.into_iter().collect();
    token_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let mut most_common_tokens: Vec<(String, usize)> = Vec::new();
    for (token_id, count) in token_counts_vec.iter().take(n) {
        most_common_tokens.push((tokens[*token_id].clone(), *count));
    }

    most_common_tokens
}