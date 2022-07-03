

pub struct WordSwap{
    to_be_swapped_word:String,
    swap_with:String,
    word_vector:Vec<char>,
    split_word:Vec<String>,
}

impl WordSwap {

    pub fn new(word: &str,to_be_swapped_word:&str,swap_with:&str) -> Self {
        WordSwap{
            to_be_swapped_word:to_be_swapped_word.to_string(),
            swap_with:swap_with.to_string(),
            word_vector:(word.to_string().trim()).chars().collect(),
            split_word:vec![],
        }
    }

    pub fn swap(&mut self) -> String {
        self.set_split_word();
        if self.split_word.contains(&self.to_be_swapped_word) {
            for index in 0..self.split_word.len() {
                if self.split_word[index].eq(&self.to_be_swapped_word) {
                    self.split_word[index] = self.swap_with.clone();
                }
            }
        }else {
            return String::from("");
        }
        return self.new_word();
    }

    fn new_word(&mut self) -> String {
        let mut new_word:String = String::from("");
        for index in 0..self.split_word.len(){ 
            new_word.push_str(&*(self.split_word[index]));
        }
        return new_word;
    }

    fn set_split_word(&mut self){
        let mut temp_str:String = String::from("");
        let skip_symbols:Vec<&str> = vec!["!"," ","^","(",")","?",",","."];
        
        for index in 0..self.word_vector.len(){
            let value:&str = &*(self.word_vector[index].to_string());
            if skip_symbols.contains(&value) {
                self.split_word.push(temp_str.clone());
                self.split_word.push(value.to_string());
                temp_str.clear();
            }else {
                temp_str.push_str(value);
                if index == self.word_vector.len() -1 {
                    self.split_word.push(temp_str.clone());
                }
            }     
        }
    }
}