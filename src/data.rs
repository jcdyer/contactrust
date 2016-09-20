use postgres::{Connection, SslMode};

use serializers::Jsonable;

pub struct Contact {
    pub name: String,
    pub email: String,
}


impl Jsonable for Contact {
    fn to_json(&self) -> String {
        let mut body = "".to_string();
        body.push_str("{");
        body.push_str("\"name\": \"");
        body.push_str(&self.name);
        body.push_str("\", ");
        body.push_str("\"email\": \"");
        body.push_str(&self.email);
        body.push_str("\"");
        body.push_str("}");
        body
    }
}

pub fn get_contacts() -> Vec<Contact> {
    let dsn = "postgresql://rust:rust@localhost/rust";
    let conn = Connection::connect(dsn, SslMode::None).expect("Connection error");
    let rows = conn.query("SELECT name, email FROM contacts", &[]).expect("Selecting emails failed");
    rows.iter()
        .map(|row| Contact {name: row.get("name"), email: row.get("email")})
        .collect::<Vec<Contact>>()
}

#[cfg(test)]
mod test {
    use super::*;
    use serializers::Jsonable;

    #[test]
    fn test_contact_to_json() {
        let contact = Contact { name: "Tennyson".to_string(), email: "at@lotos.org".to_string() };
        assert_eq!(contact.to_json(), "{\"name\": \"Tennyson\", \"email\": \"at@lotos.org\"}".to_string());
    }

    #[test]
    fn test_contact_vec_to_json() {
        let contacts = vec!(
            Contact { name: "Tennyson".to_string(), email: "at@lotos.org".to_string() },
            Contact { name: "Keats".to_string(), email: "jk@lotos.org".to_string() },
        );
        assert_eq!(
            contacts.to_json(),
            "[{\"name\": \"Tennyson\", \"email\": \"at@lotos.org\"}, {\"name\": \"Keats\", \"email\": \"jk@lotos.org\"}]".to_string()
        );
    }
}
