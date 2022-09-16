use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buff> {
    data: HashMap<&'buff str, Value<'buff>>,
}

#[derive(Debug)]
pub enum Value<'buff> {
    Single(&'buff str),
    Multiple(Vec<&'buff str>),
}

impl<'buff> QueryString<'buff> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buff> From<&'buff str> for QueryString<'buff> {
    fn from(s: &'buff str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find("=") {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing_value: &mut Value| match existing_value {
                    Value::Single(prev) => {
                        *existing_value = Value::Multiple(vec![prev, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
