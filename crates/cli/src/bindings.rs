pub fn script_origin<'a>(
    scope: &mut v8::HandleScope<'a>,
    name: v8::Local<'a, v8::String>,
) -> v8::ScriptOrigin<'a> {
    let source_map_url = v8::String::new(scope, "").unwrap();

    v8::ScriptOrigin::new(
        scope,
        name.into(),
        0,
        0,
        false,
        123,
        source_map_url.into(),
        true,
        false,
        false,
    )
}
