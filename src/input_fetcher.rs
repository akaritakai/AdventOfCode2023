use reqwest::StatusCode;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

pub struct InputFetcher {
    /// The base URL for Advent of Code (by default 'https://adventofcode.com').
    base_url: String,
    /// The location where the puzzle inputs are locally stored (by default 'puzzle').
    input_path: PathBuf,
    /// The location where the session token is locally stored (by default 'cookie.txt').
    session_token_path: PathBuf,
}

impl InputFetcher {
    /// Creates an InputFetcher using the default values.
    pub fn create() -> Self {
        Self::create_custom(
            "https://adventofcode.com",
            Path::new("puzzle"),
            Path::new("cookie.txt"),
        )
    }

    /// Creates an InputFetcher using the specified values. Used only for testing.
    pub fn create_custom(base_url: &str, input_path: &Path, session_token_path: &Path) -> Self {
        Self {
            base_url: base_url.to_string(),
            input_path: input_path.to_path_buf(),
            session_token_path: session_token_path.to_path_buf(),
        }
    }

    /// Returns the input for the given day. Will try to return it from the local file system first,
    /// and if that fails, will try to fetch it from the Advent of Code website.
    pub fn get_input(&self, day: u8) -> Result<String, Box<dyn Error>> {
        let input_file_path = self.input_path.join(format!("{:02}", day));
        if input_file_path.exists() {
            Ok(fs::read_to_string(input_file_path)?)
        } else {
            let session_token = self.get_session_token()?;
            let input = self.fetch_input(day, &session_token)?;
            fs::write(input_file_path, &input)?;
            Ok(input)
        }
    }

    fn get_session_token(&self) -> Result<String, Box<dyn Error>> {
        let session_token_file = File::open(&self.session_token_path)?;
        let mut session_token_reader = BufReader::new(session_token_file);
        let mut session_token = String::new();
        session_token_reader.read_to_string(&mut session_token)?;
        Ok(session_token)
    }

    fn fetch_input(&self, day: u8, session_token: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, url_path(day));
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .header("Cookie", format!("session={}", session_token))
            .send()?;
        if response.status() == StatusCode::OK {
            Ok(response.text()?)
        } else {
            Err(format!("Failed to fetch input: {}", response.status()).into())
        }
    }
}

fn url_path(day: u8) -> String {
    format!("/2023/day/{}/input", day)
}

#[cfg(test)]
mod tests {
    use crate::input_fetcher::{url_path, InputFetcher};
    use httpmock::prelude::*;
    use httpmock::Mock;
    use std::path::Path;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn local_fetch_succeeds_without_remote_access() {
        let context = TestContext::create();
        let fetcher = context.get_fetcher();
        for day in 1..=25 {
            let mock = context.server_down_mock(day);
            let input = fetcher.get_input(day).unwrap();
            assert_eq!(input, context.get_input(day));
            mock.assert_hits(0);
        }
    }

    #[test]
    fn remote_fetch_succeeds() {
        let context = TestContext::create();
        let fetcher = context.get_fetcher();
        for day in 1..=25 {
            context.delete_puzzle_input_file(day);
            let mock = context.server_up_mock(day);
            let input = fetcher.get_input(day).unwrap();
            assert_eq!(input, context.get_input(day));
            mock.assert();

            // Verify that local files were created.
            let input_file_path = context.input_dir.path().join(format!("{:02}", day));
            let input = std::fs::read_to_string(input_file_path).unwrap();
            assert_eq!(input, context.get_input(day));
        }
    }

    #[test]
    fn fetch_fails_when_all_sources_unavailable() {
        let context = TestContext::create();
        let fetcher = context.get_fetcher();
        for day in 1..=25 {
            context.delete_puzzle_input_file(day);
            let mock = context.server_down_mock(day);
            let result = fetcher.get_input(day);
            assert!(result.is_err());
            mock.assert();
        }
    }

    #[test]
    fn fetch_fails_if_missing_session_token() {
        let context = TestContext::create();
        let session_cookie_path = Path::new("missing_cookie.txt");
        assert!(!session_cookie_path.exists());
        let fetcher = InputFetcher::create_custom(
            context.server.base_url().as_str(),
            context.input_dir.path(),
            session_cookie_path,
        );
        for day in 1..=25 {
            context.delete_puzzle_input_file(day);
            let mock = context.server_up_mock(day);
            let result = fetcher.get_input(day);
            assert!(result.is_err());
            mock.assert_hits(0);
        }
    }

    #[test]
    fn fetch_fails_if_bad_session_token() {
        let context = TestContext::create();
        let bad_cookie = random_session_token();
        assert_ne!(context.session_token, bad_cookie);
        let bad_cookie_file = NamedTempFile::new().unwrap();
        std::fs::write(bad_cookie_file.path(), bad_cookie.as_bytes()).unwrap();
        let fetcher = InputFetcher::create_custom(
            context.server.base_url().as_str(),
            context.input_dir.path(),
            bad_cookie_file.path(),
        );
        for day in 1..=25 {
            context.delete_puzzle_input_file(day);
            let mock_with_correct_token = context.server_up_mock(day);
            let mock_with_invalid_token = context.server.mock(|when, then| {
                when.method(GET)
                    .path(url_path(day).as_str())
                    .header("Cookie", format!("session={}", bad_cookie));
                then.status(400)
                    .body("Puzzle inputs differ by user.  Please log in to get your puzzle input.");
            });
            let result = fetcher.get_input(day);
            assert!(result.is_err());
            mock_with_correct_token.assert_hits(0);
            mock_with_invalid_token.assert();
        }
    }

    #[test]
    fn fetch_fails_if_puzzle_requested_early() {
        let context = TestContext::create();
        let fetcher = context.get_fetcher();
        for day in 1..=25 {
            context.delete_puzzle_input_file(day);
            let mock = context.server.mock(|when, then| {
                when.method(GET)
                    .path(url_path(day).as_str())
                    .header("Cookie", format!("session={}", context.session_token));
                then.status(404)
                    .body("Please don't repeatedly request this endpoint before it unlocks! \
                           The calendar countdown is synchronized with the server time; \
                           the link will be enabled on the calendar the instant this puzzle becomes available.");
            });
            let result = fetcher.get_input(day);
            assert!(result.is_err());
            mock.assert();
        }
    }

    struct TestContext {
        inputs: Vec<String>,
        input_dir: TempDir,
        session_token: String,
        session_token_file: NamedTempFile,
        server: MockServer,
    }

    impl TestContext {
        pub fn create() -> Self {
            let inputs: Vec<String> = (1..=25).map(|_| random_puzzle()).collect();
            let input_dir = TempDir::new().unwrap();
            for day in 1..=25 {
                let input_file_path = input_dir.path().join(format!("{:02}", day));
                std::fs::write(&input_file_path, &inputs[day - 1]).unwrap();
            }
            let session_token = random_session_token();
            let session_token_file = NamedTempFile::new().unwrap();
            std::fs::write(session_token_file.path(), session_token.as_bytes()).unwrap();
            let server = MockServer::start();
            Self {
                inputs,
                input_dir,
                session_token,
                session_token_file,
                server,
            }
        }

        pub fn server_up_mock(&self, day: u8) -> Mock {
            self.server.mock(|when, then| {
                when.method(GET)
                    .path(url_path(day).as_str())
                    .header("Cookie", format!("session={}", self.session_token));
                then.status(200).body(self.get_input(day));
            })
        }

        pub fn server_down_mock(&self, day: u8) -> Mock {
            self.server.mock(|when, then| {
                when.method(GET)
                    .path(url_path(day).as_str())
                    .header("Cookie", format!("session={}", self.session_token));
                then.status(501);
            })
        }

        pub fn get_input(&self, day: u8) -> &str {
            &self.inputs[day as usize - 1]
        }

        pub fn get_fetcher(&self) -> InputFetcher {
            InputFetcher::create_custom(
                self.server.base_url().as_str(),
                self.input_dir.path(),
                self.session_token_file.path(),
            )
        }

        pub fn delete_puzzle_input_file(&self, day: u8) {
            let input_file_path = self.input_dir.path().join(format!("{:02}", day));
            std::fs::remove_file(input_file_path).unwrap();
        }
    }

    fn random_puzzle() -> String {
        // Puzzle inputs tend to contain a wide variety of ASCII characters including line feed.
        // They can also be fairly large.
        let charset = format!(
            "{}{}{}{}{}{}{}{}",
            "\n",                         // ASCII code 10 (line feed)
            " !\"#$%&'()*+,-./",          // ASCII codes 32-47 (symbols)
            "0123456789",                 // ASCII codes 48-57 (digits)
            ":;<=>?@",                    // ASCII codes 58-64 (symbols)
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ", // ASCII codes 65-90 (uppercase letters)
            "[\\]^_`",                    // ASCII codes 91-96 (symbols)
            "abcdefghijklmnopqrstuvwxyz", // ASCII codes 97-122 (lowercase letters)
            "{|}~"                        // ASCII codes 123-126 (symbols)
        );
        random_string(charset.as_str(), 65535)
    }

    fn random_session_token() -> String {
        // Session tokens appear to be 128 characters of ASCII hex digits
        random_string("0123456789abcdef", 128)
    }

    fn random_string(charset: &str, length: usize) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let index = rng.gen_range(0..charset.len());
                charset.chars().nth(index).unwrap()
            })
            .collect()
    }
}
