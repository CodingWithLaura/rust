//idea is to only define a skeleton of an algorithm, whereas the specific implementation is seperated into different parts
//client can choose different implementation while workflow where algorithm is used remains the same
//the genereal algorithm does not depend on specific implementation -> "Dependency Inversion"

use std::collections::HashMap;

type Data = HashMap<String, u32>;

//trait is a collection of methods defined for an unknown type: Self
//its something between an interface in java and an abstract baseclass

trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}

//this is the strategy struct, which takes a implementation of the Formattter trait, which can be all implementing structs and calls the format function within the generate function
struct Report;

impl Report {
    fn generate<T: Formatter>(g: T, s: &mut String) {
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        g.format(&data, s);
    }
}

struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        //deconstructing hashmap to access values seperately
        for (k, v) in data {
            let entry = format!("{} {}\n", k, v);
            buf.push_str(&entry);
        }
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        buf.pop(); // remove extra , at the end
        buf.push(']');
    }
}

fn main() {
    let mut s = String::from("");
    //which fomatting struct will be called is chosen by argument of generate
    //string argument is empty at the beginning and values get appended also in generate function
    Report::generate(Text, &mut s);
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    s.clear(); //reuse same buffer

    Report::generate(Json, &mut s);
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));
}
