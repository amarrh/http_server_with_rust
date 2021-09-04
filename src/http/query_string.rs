use std::collections::HashMap;
#[derive(Debug)]
pub struct QueryString<'buf>{
    data: HashMap<&'buf str, Value<'buf>>,
}
#[derive(Debug)]
pub enum Value<'buf>{
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf>{
    pub fn get(&self, key: &str) -> Option<&Value>{
        self.data.get(key)
    }
}
// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_string in s.split('&'){
            let mut key = sub_string;
            let mut val = "";
            if let Some(i) = sub_string.find('='){
                key = &sub_string[..i];
                val = &sub_string[i + 1..];
            }
            data.entry(key)
            .and_modify(|existing| match existing{
                Value::Single(prev_value) => {
                    /*let mut vec = Vec::new();
                    vec.push(prev_value);
                    vec.push(val); */ //isto kao iduća linija
                    *existing = Value::Multiple(vec![prev_value,val]); // * - follow the pointer and assign a new value over what it was pointing to
                },
                Value::Multiple(vec) => vec.push(val)
            })
            .or_insert(Value::Single(val));
        }
        QueryString { data }

    }
     
}
