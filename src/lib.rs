use search_query_parser::{parse_query_to_condition, Condition, Operator};
use serde::Serialize;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_SearchQueryParser_parseQueryToCondition<'local>(
    mut env: JNIEnv<'local>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    _class: JClass<'local>,
    input: JString<'local>,
) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let query_string: String = env
        .get_string(&input)
        .expect("Couldn't get query_string from input")
        .into();

    let condition = match parse_query_to_condition(query_string.as_ref()) {
        Ok(condition) => json(condition),
        Err(e) => {
            println!("ERROR: {e}");
            json(Condition::Keyword(query_string))
        }
    };
    let condition_json =
        serde_json::to_string(&condition).expect("Couldn't convert condition json to string");

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(condition_json)
        .expect("Couldn't create condition json string to output");

    // Finally, extract the raw pointer to return.
    output.into_raw()
}

fn json(condition: Condition) -> ConditionJson {
    match condition {
        Condition::Keyword(value) => ConditionJson::keyword(value),
        Condition::PhraseKeyword(value) => ConditionJson::phrase_keyword(value),
        Condition::Not(value) => ConditionJson::not(json(*value)),
        Condition::Operator(operator, value) => match operator {
            Operator::And => ConditionJson::and(value.into_iter().map(|v| json(v)).collect()),
            Operator::Or => ConditionJson::or(value.into_iter().map(|v| json(v)).collect()),
        },
        _ => ConditionJson::none(),
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
struct ConditionJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword: Option<String>,
    #[serde(rename(serialize = "phraseKeyword"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    phrase_keyword: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not: Option<Box<ConditionJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    and: Option<Vec<ConditionJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    or: Option<Vec<ConditionJson>>,
}

impl ConditionJson {
    fn none() -> Self {
        Self {
            keyword: None,
            phrase_keyword: None,
            not: None,
            and: None,
            or: None,
        }
    }
    fn keyword(value: String) -> Self {
        Self {
            keyword: Some(value),
            phrase_keyword: None,
            not: None,
            and: None,
            or: None,
        }
    }

    fn phrase_keyword(value: String) -> Self {
        Self {
            keyword: None,
            phrase_keyword: Some(value),
            not: None,
            and: None,
            or: None,
        }
    }

    fn not(value: ConditionJson) -> Self {
        Self {
            keyword: None,
            phrase_keyword: None,
            not: Some(Box::new(value)),
            and: None,
            or: None,
        }
    }

    fn and(value: Vec<ConditionJson>) -> Self {
        Self {
            keyword: None,
            phrase_keyword: None,
            not: None,
            and: Some(value),
            or: None,
        }
    }

    fn or(value: Vec<ConditionJson>) -> Self {
        Self {
            keyword: None,
            phrase_keyword: None,
            not: None,
            and: None,
            or: Some(value),
        }
    }
}
