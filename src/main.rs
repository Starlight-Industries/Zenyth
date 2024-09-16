use std::fs::read_to_string;
use rayon::prelude::*;
pub mod lexer;
use zenyth::Token;
use logos::Logos;

fn main() {
    let file = get_input_file();
    let chunk_size = 2048 * 999999999 * 999999;

    let execution_time = std::time::Instant::now();

    let chunks: Vec<&str> = split_into_chunks(&file, chunk_size);
    
    let tokens: Vec<Vec<(Token, String)>> = chunks.par_iter()
        .map(|chunk| {
            let mut lex = Token::lexer(chunk);
            let mut chunk_tokens = Vec::with_capacity(chunk.len() / 5); // Estimate based on average token length

            while let Some(Ok(token)) = lex.next() {
                chunk_tokens.push((token, lex.slice().to_string()));
            }
            chunk_tokens
        })
        .collect();

    let all_tokens: Vec<(Token, String)> = tokens.into_iter().flatten().collect();

    for (token, lexeme) in all_tokens {
        println!("{:?}: {}", token, lexeme);
        //todo implement processing lol
    }

    println!("Execution time: {:?}", execution_time.elapsed());
}
fn get_input_file() -> String {
    let file_endings = ["zen", "z", "zenyth", "znyth"];
    let mut file_contents: Result<String, std::io::Error> = Ok("".to_string());
    for ending in file_endings {
        let file_name = format!("input.{}", ending);
        file_contents = read_to_string(file_name);
        if file_contents.is_ok() {
            //println!("{:?}", file_contents);
            break;
        }
    }
    return file_contents.unwrap();
}

fn split_into_chunks(s: &str, chunk_size: usize) -> Vec<&str> {
    let mut chunks = Vec::new();
    let mut start = 0;
    let mut end = chunk_size;

    while start < s.len() {
        if end > s.len() {
            end = s.len();
        } else {
            // this is only to make sure that the chunk doesnt split in the middle of a word
            // imo kind of hacky but it works for now
            while !s.is_char_boundary(end) && end > start {
                end -= 1;
            }
        }

        chunks.push(&s[start..end]);
        start = end;
        end = start + chunk_size;
    }

    return chunks;
}
