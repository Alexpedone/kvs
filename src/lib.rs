fn main() {
    println!("Hello, world!");
}

pub struct KvStore{
    key: String,
    value: Option<String>,
}

impl KvStore{
    pub fn new() -> Self{
        KvStore{key:"".to_string(), value:Some("".to_string())}
    }
    pub fn set (&mut self, key: String, value: String) {
        self.key = key;
        self.value = Some(value);
    }
    pub fn get(& self, key: String) -> Option<String>{
        if key == self.key{
            self.value.clone()
        } else {None}
    }
    pub fn remove(&mut self, key: String){
        self.set(key, "".to_string());
    }
}