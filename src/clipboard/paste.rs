use arboard::Clipboard;
use palette::Hsv;
use regex::Regex;

use crate::utils::hsv_from_rgb;

pub fn clipboard_paste() -> Option<Hsv> {
    let mut clipboard = Clipboard::new().unwrap();
    let content = clipboard.get_text().unwrap();
    validate_pasted_color(&content)
}

fn validate_rgb(s: &str) -> Option<Hsv> {
    static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r"^(?:[rR][gG][bB])?\(? *(\d{1,3}) *, *(\d{1,3}) *, *(\d{1,3}) *\)?$").unwrap()
    });
    let caps = RE.captures(s)?;
    let r: u8 = caps[1].parse().ok()?;
    let g: u8 = caps[2].parse().ok()?;
    let b: u8 = caps[3].parse().ok()?;

    Some(hsv_from_rgb(r, g, b))
}

fn validate_hsv(s: &str) -> Option<Hsv> {
    static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(
            r"(?ix)^(?:[hH][sS][vV])?\(? *\s*(\d{1,3}(?:\.\d+)?)\s*,\s*(\d{1,3}(?:\.\d+)?)%?\s*,\s*(\d{1,3}(?:\.\d+)?)%?\s*\)?$",
        )
        .unwrap()
    });
    let cap = RE.captures(s)?;
    let h: f32 = cap[1].parse().ok()?;
    let mut s = cap[2].parse::<f32>().ok()?;
    let mut v = cap[3].parse::<f32>().ok()?;
    if cap.get(2).unwrap().as_str().ends_with('%') || s > 1.0 {
        s /= 100.0;
    }
    if cap.get(3).unwrap().as_str().ends_with('%') || v > 1.0 {
        v /= 100.0;
    }
    if !(0.0..=360.0).contains(&h) || !(0.0..=1.0).contains(&s) || !(0.0..=1.0).contains(&v) {
        return None;
    }

    Some(Hsv::new(h, s, v))
}

fn validate_hex(s: &str) -> Option<Hsv> {
    static RE: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"^(?:0?[xX]|#)?([0-9a-fA-F]{6})$").unwrap());
    let cap = RE.captures(s)?;
    let hex = &cap[1];
    let rgb = u32::from_str_radix(hex, 16).ok()?;
    let r = ((rgb >> 16) & 0xFF) as u8;
    let g = ((rgb >> 8) & 0xFF) as u8;
    let b = (rgb & 0xFF) as u8;
    Some(hsv_from_rgb(r, g, b))
}

pub fn validate_pasted_color(str: &str) -> Option<Hsv> {
    if let Some(hsv) = validate_hex(str) {
        return Some(hsv);
    }
    if let Some(hsv) = validate_rgb(str) {
        return Some(hsv);
    }
    if let Some(hsv) = validate_hsv(str) {
        return Some(hsv);
    }
    None
}

#[test]
fn test_paste_validation() {
    struct TestCase {
        input: &'static str,
        expected: Option<Hsv>,
    }
    let correct_hsv = hsv_from_rgb(255, 87, 51);
    let test_cases = [
        TestCase {
            input: "#FF5733",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "#ff5733",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "Ff5733",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "0xFf5733",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "xFf5733",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "0XFf5733",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "rgb(255, 87, 51)",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "rgb(  255 ,  87 ,  51  )",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "(255,87,51)",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "(  255 ,  87 ,  51  )",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "255,87,51",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "  255    ,   87    , 51   ",
            expected: Some(correct_hsv),
        },
        TestCase {
            input: "hsv(11, 80%, 100%)",
            expected: Some(Hsv::new(11.0, 0.8, 1.0)),
        },
        TestCase {
            input: "hsv(11, 80, 100)",
            expected: Some(Hsv::new(11.0, 0.8, 1.0)),
        },
        TestCase {
            input: "hsv(  11  ,  80%  ,  100%  )",
            expected: Some(Hsv::new(11.0, 0.8, 1.0)),
        },
        TestCase {
            input: "hsv(11.00, 0.8, 1.0)",
            expected: Some(Hsv::new(11.0, 0.8, 1.0)),
        },
        TestCase {
            input: "invalid string",
            expected: None,
        },
        TestCase {
            input: "#AABBCG",
            expected: None,
        },
        TestCase {
            input: "rgb(12, 34)",
            expected: None,
        },
        TestCase {
            input: "rgb(256, 87, 51)",
            expected: None,
        },
        TestCase {
            input: "#F53",
            expected: None,
        },
        TestCase {
            input: "#FF5733FF",
            expected: None,
        },
        TestCase {
            input: "rgba(255,87,51,0.8)",
            expected: None,
        },
        TestCase {
            input: "hsl(11, 100%, 60%)",
            expected: None,
        },
        TestCase {
            input: "",
            expected: None,
        },
        TestCase {
            input: "  ",
            expected: None,
        },
        TestCase {
            input: "color:#FF5733;",
            expected: None,
        },
    ];
    for case in test_cases.iter() {
        assert_eq!(
            validate_pasted_color(case.input),
            case.expected,
            "Failed for input: {}",
            case.input
        );
    }
}
