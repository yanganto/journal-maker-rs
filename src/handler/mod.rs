use gotham::state::State;

const GREETING: &'static str = "Powered by Rust";

pub fn greeting(state: State) -> (State, &'static str) {
    (state, GREETING)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;


    #[test]
    fn receive_greeting_response() {
        let test_server = TestServer::new(|| Ok(greeting)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost:9000")
            .perform()
            .unwrap();

        assert_eq!(response.status(), 200);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Powered by Rust");
    }
}
