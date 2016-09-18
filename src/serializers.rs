
pub trait Jsonable {
    fn to_json(&self) -> String;
}


impl<T> Jsonable for Vec<T> where T: Jsonable {
    fn to_json(&self) -> String {
        let mut body = "".to_string();
        body.push_str("[");
        for element in self.iter() {
            if body.chars().last() != Some('[') {
                body.push_str(", ");
            };
            body.push_str(&element.to_json())
        }
        body.push_str("]");
        body
    }
}
