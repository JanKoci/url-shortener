use chrono::{DateTime, Utc};
use rand::Rng;

pub fn generate_code() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

pub fn is_expired(expires_at: Option<DateTime<Utc>>) -> bool {
    expires_at.map(|d| d < Utc::now()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn expired_url_returns_true() {
        let past = Utc::now() - Duration::seconds(10);
        assert!(is_expired(Some(past)));
    }

    #[test]
    fn future_url_returns_false() {
        let future = Utc::now() + Duration::seconds(10);
        assert!(!is_expired(Some(future)));
    }

    #[test]
    fn no_expiry_returns_false() {
        assert!(!is_expired(None));
    }
}
