

pub fn swap(str: &str,to_be_swapped_word:&str,swapped:&str) -> String{
    let str_vector:Vec<char> = (str.to_string().trim()).chars().collect();
    let mut split_word:Vec<String> = split_word(str_vector);

    if to_be_swapped_word_exists(to_be_swapped_word, &split_word) {
        for index in 0..split_word.len() {
            if split_word[index].eq(&to_be_swapped_word.to_string()) {
                split_word[index] = swapped.to_string();
            }
        }
    }

    let mut new_str:String = String::from("");
    for index in 0..split_word.len(){ // find a shortter way and find out about collect()
        let value:&str = &*(split_word[index]);
        new_str.push_str(value);
    }
    // println!("{:?}",split_word);
    new_str
}

fn to_be_swapped_word_exists(to_be_swapped_word:&str,split_word:&Vec<String>) -> bool {
    if split_word.contains(&to_be_swapped_word.to_string()) {
        true
    }else {
        false
    }
}

fn split_word(str_vector:Vec<char>) -> Vec<String>{
    let mut split_word:Vec<String> = vec![];
    let mut temp_str:String = String::from("");
    let skip_symbols:Vec<&str> = vec!["!"," ","^","(",")","?",",","."];

    for index in 0..str_vector.len(){
        let value:&str = &(str_vector[index].to_string());
        if skip_symbols.contains(&value) {
            split_word.push(temp_str.clone());
            split_word.push(value.to_string());
            temp_str.clear();
        }else {
            temp_str.push_str(value);
            if index == str_vector.len() -1 {
                split_word.push(temp_str.clone());
            }
        }     
    }
    split_word
}