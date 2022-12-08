#![cfg(feature = "firefox")]

use web_extensions::contextual_identities::*;

mod util;
use util::*;

#[test]
fn color_serde() {
    assert_json_serde_test_cases(&[
        JSONSerdeTestCase {
            value: Color::Blue,
            json: r#""blue""#,
        },
        JSONSerdeTestCase {
            value: Color::Turquoise,
            json: r#""turquoise""#,
        },
        JSONSerdeTestCase {
            value: Color::Green,
            json: r#""green""#,
        },
        JSONSerdeTestCase {
            value: Color::Yellow,
            json: r#""yellow""#,
        },
        JSONSerdeTestCase {
            value: Color::Orange,
            json: r#""orange""#,
        },
        JSONSerdeTestCase {
            value: Color::Red,
            json: r#""red""#,
        },
        JSONSerdeTestCase {
            value: Color::Pink,
            json: r#""pink""#,
        },
        JSONSerdeTestCase {
            value: Color::Purple,
            json: r#""purple""#,
        },
        JSONSerdeTestCase {
            value: Color::Toolbar,
            json: r#""toolbar""#,
        },
    ])
}

#[test]
fn icon_serde() {
    assert_json_serde_test_cases(&[
        JSONSerdeTestCase {
            value: Icon::Fingerprint,
            json: r#""fingerprint""#,
        },
        JSONSerdeTestCase {
            value: Icon::Briefcase,
            json: r#""briefcase""#,
        },
        JSONSerdeTestCase {
            value: Icon::Dollar,
            json: r#""dollar""#,
        },
        JSONSerdeTestCase {
            value: Icon::Cart,
            json: r#""cart""#,
        },
        JSONSerdeTestCase {
            value: Icon::Circle,
            json: r#""circle""#,
        },
        JSONSerdeTestCase {
            value: Icon::Gift,
            json: r#""gift""#,
        },
        JSONSerdeTestCase {
            value: Icon::Vacation,
            json: r#""vacation""#,
        },
        JSONSerdeTestCase {
            value: Icon::Food,
            json: r#""food""#,
        },
        JSONSerdeTestCase {
            value: Icon::Fruit,
            json: r#""fruit""#,
        },
        JSONSerdeTestCase {
            value: Icon::Pet,
            json: r#""pet""#,
        },
        JSONSerdeTestCase {
            value: Icon::Tree,
            json: r#""tree""#,
        },
        JSONSerdeTestCase {
            value: Icon::Chill,
            json: r#""chill""#,
        },
        JSONSerdeTestCase {
            value: Icon::Fence,
            json: r#""fence""#,
        },
    ])
}
