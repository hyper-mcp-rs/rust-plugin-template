use anyhow::Result;
use chrono::{DateTime, Utc};
use extism_pdk::*;
use std::{thread, time::Duration};

#[allow(dead_code)]
pub(crate) fn http_request_with_retry(req: &HttpRequest) -> Result<HttpResponse> {
    fn parse_retry_after(value: &str) -> Option<Duration> {
        if let Ok(secs) = value.parse::<u64>() {
            Some(Duration::from_secs(secs))
        } else if let Ok(date) = DateTime::parse_from_rfc2822(value) {
            let target = date.with_timezone(&Utc);
            let now = Utc::now();
            if target > now {
                let delta = target - now;
                delta.to_std().ok()
            } else {
                None
            }
        } else if let Ok(date) =
            chrono::NaiveDateTime::parse_from_str(value, "%a %b %e %H:%M:%S %Y")
        {
            let target = date.and_utc();
            let now = Utc::now();
            if target > now {
                let delta = target - now;
                delta.to_std().ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    const MAX_HTTP_ATTEMPTS: u32 = 3;
    const RETRY_DELAY: Duration = Duration::from_secs(15);

    let mut attempt = 0;

    loop {
        attempt += 1;
        match http::request::<()>(req, None) {
            Ok(res) => {
                let status = res.status_code();

                if attempt < MAX_HTTP_ATTEMPTS && (status == 429 || status >= 500) {
                    thread::sleep(
                        res.header("retry-after")
                            .or_else(|| res.header("Retry-After"))
                            .and_then(parse_retry_after)
                            .unwrap_or(RETRY_DELAY),
                    );
                    continue;
                }
                break Ok(res);
            }
            Err(e) => {
                if attempt < MAX_HTTP_ATTEMPTS {
                    thread::sleep(RETRY_DELAY);
                    continue;
                }
                break Err(e);
            }
        }
    }
}
