use std::{cmp::PartialEq, fmt::Debug};

pub fn assert_json_serialize_eq<'a, T>(left: &'a T, right: &'a str)
where
    T: serde::Serialize + Debug,
{
    assert_eq!(
        serde_json::to_string(left).expect(&format!("failed to serialize {:?} to JSON", left)),
        right
    );
}

pub fn assert_json_deserialize_eq<'a, T>(left: &'a str, right: &'a T)
where
    T: serde::Deserialize<'a> + PartialEq + Debug,
{
    assert_eq!(
        &serde_json::from_str::<T>(left).expect(&format!("failed to deserialize JSON {}", left)),
        right
    )
}

pub fn assert_json_serde_eq<'a, T>(left: &'a T, right: &'a str)
where
    T: serde::Serialize + serde::Deserialize<'a> + PartialEq + Debug,
{
    assert_json_serialize_eq(left, right);
    assert_json_deserialize_eq(right, left);
}

pub struct JSONSerdeTestCase<'a, T> {
    pub value: T,
    pub json: &'a str,
}

#[allow(dead_code)]
pub fn assert_json_serialize_test_cases<'a, T, I>(tcs: I)
where
    T: 'a + serde::Serialize + PartialEq + Debug,
    I: 'a + IntoIterator<Item = &'a JSONSerdeTestCase<'a, T>>,
{
    for tc in tcs {
        assert_json_serialize_eq(&tc.value, tc.json);
    }
}

pub fn assert_json_serde_test_cases<'a, T, I>(tcs: I)
where
    T: 'a + serde::Serialize + serde::Deserialize<'a> + PartialEq + Debug,
    I: 'a + IntoIterator<Item = &'a JSONSerdeTestCase<'a, T>>,
{
    for tc in tcs {
        assert_json_serde_eq(&tc.value, tc.json);
    }
}
