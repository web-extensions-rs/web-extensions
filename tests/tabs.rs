use web_extensions::tabs::*;

mod util;
use util::*;

#[test]
fn status_serde() {
    assert_json_serde_test_cases(&[
        JSONSerdeTestCase {
            value: Status::Loading,
            json: r#""loading""#,
        },
        JSONSerdeTestCase {
            value: Status::Complete,
            json: r#""complete""#,
        },
    ])
}

#[test]
fn window_type_serialize() {
    assert_json_serialize_test_cases(&[
        JSONSerdeTestCase {
            value: WindowType::Normal,
            json: r#""normal""#,
        },
        JSONSerdeTestCase {
            value: WindowType::Popup,
            json: r#""popup""#,
        },
        JSONSerdeTestCase {
            value: WindowType::Panel,
            json: r#""panel""#,
        },
        JSONSerdeTestCase {
            value: WindowType::Devtools,
            json: r#""devtools""#,
        },
    ])
}
