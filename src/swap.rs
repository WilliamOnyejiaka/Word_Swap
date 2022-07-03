

pub fn swap(word: &str,to_be_swapped_word:&str,swap_with:&str) -> String{
    let word_vector:Vec<char> = (word.to_string().trim()).chars().collect();
    let mut split_word:Vec<String> = split_word(word_vector);

    if split_word.contains(&to_be_swapped_word.to_string()) {
        for index in 0..split_word.len() {
            if split_word[index].eq(&to_be_swapped_word.to_string()) {
                split_word[index] = swap_with.to_string();
            }
        }
    }else {
        return String::from("");
    }

    let mut new_word:String = String::from("");
    for index in 0..split_word.len(){ // find a shortter way and find out about collect()
        // let value:&str = &*(split_word[index]);
        new_word.push_str(&*(split_word[index]));
    }
    return new_word;
}

fn split_word(word_vector:Vec<char>) -> Vec<String>{
    let mut split_word:Vec<String> = vec![];
    let mut temp_str:String = String::from("");
    let skip_symbols:Vec<&str> = vec!["!"," ","^","(",")","?",",","."];

    for index in 0..word_vector.len(){
        let value:&str = &*(word_vector[index].to_string());
        if skip_symbols.contains(&value) {
            split_word.push(temp_str.clone());
            split_word.push(value.to_string());
            temp_str.clear();
        }else {
            temp_str.push_str(value);
            if index == word_vector.len() -1 {
                split_word.push(temp_str.clone());
            }
        }     
    }
    return split_word;
}