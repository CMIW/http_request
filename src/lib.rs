#[macro_use]
extern crate getset;

#[derive(Default, Debug, Getters, PartialEq, Eq)]
pub struct HttpRequest {
    #[getset(get = "pub")]
    uri: String,
    #[getset(get = "pub")]
    method: String,
    #[getset(get = "pub")]
    version: String,
    #[getset(get = "pub")]
    field: Vec<String>,
}

impl HttpRequest {
    pub fn new() -> HttpRequest {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.uri == "" ||
        self.method == "" ||
        self.version == ""
    }

    /// TODO: finish the validation of the http_request
    pub fn is_valid(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        else{
            return true;
        }
    }

    pub fn set_uri(&mut self, uri: &str) -> &mut Self{
        self.uri = String::from(uri);

        self
    }

    pub fn set_method(&mut self, method: &str) -> &mut Self{
        self.method = String::from(method);

        self
    }

    pub fn set_version(&mut self, version: &str) -> &mut Self{
        self.version = String::from(version);

        self
    }

    pub fn push_field_line(&mut self, field_line: &str) -> &mut Self{
        self.field.push(String::from(field_line));

        self
    }

    pub fn append_field(&mut self, field: &mut Vec<String>) -> &mut Self{
        self.field.append(field);

        self
    }

}

impl std::fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result {
        if self.field.len() == 0 && !self.is_empty() {
            write!(f, "{} {} {}\r\n", self.method, self.uri, self.version)
        }
        else {
            let mut request = format!("{} {} {}", self.method, self.uri, self.version);

            for field_line in self.field.clone() {
                request = request + "\r\n" + &field_line;
            }

            write!(f, "{}", request)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_build_http_request(){
        let request_base = "GET / HTTP/1.1\r\nHost: 127.0.0.1:7878";
        let mut request = HttpRequest::new();
        request.set_method("GET")
        .set_uri("/")
        .set_version("HTTP/1.1")
        .push_field_line("Host: 127.0.0.1:7878");

        assert_eq!(request_base, format!("{}",request));
    }
}
