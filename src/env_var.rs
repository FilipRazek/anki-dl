use dotenvy::dotenv;
use std::env;

pub fn load_var(key: &str) -> String {
    dotenv().ok();
    env::var(key).expect(&format!("{} not found in .env", key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_var_succeeds() {
        unsafe {
            env::set_var("TEST_VAR", "test_value");
        }
        assert_eq!(load_var("TEST_VAR"), "test_value");
    }

    #[test]
    #[should_panic(expected = "MISSING_TEST_VAR not found in .env")]
    fn load_var_panics_when_missing() {
        unsafe {
            env::remove_var("MISSING_TEST_VAR");
        }
        load_var("MISSING_TEST_VAR");
    }
}
