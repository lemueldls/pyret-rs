macro_rules! search_next_token {
    ($stmt:path |> $token:expr) => {{
        $token.map(|stmt| $stmt(stmt))
    }};
    {$stmt:path |> $token:expr, $($rest:tt)*} => {{
        if let Some(stmt) = search_next_token!($stmt |> $token) {
            Some(stmt)
        } else {
            search_next_token!{$($rest)+}
        }
    }};
}

macro_rules! capture_token {
    // Single
    ($code:expr => {$name:expr => $matches:pat = $token:block}) => (
        Ok(
            #[allow(clippy::manual_map)]
            if let Some($matches) = $name.captures($code).unwrap() {
                Some($token)
            } else {
                None
            }
        )
    );
    // Multiple
    ($code:expr => {$name:expr => $matches:pat = $token:block, $($rest:tt)*}) => {{
        let code = $code;

        if let Some(token) = capture_token!(code => {$name => $matches = $token})? {
            Ok(Some(token))
        } else {
            capture_token!(code => {$($rest)*})
        }
    }};
    // Conditional pattern
    ($code:expr => {$name:expr => $matches:pat if $pattern:literal $token:block else $else:expr, $($rest:tt)*}) => {{
        let code = $code;

        if code.starts_with($pattern) {
            if let Some(token) = capture_token!(code => {$name => $matches = $token})? {
                Ok(Some(token))
            } else {
                $else
            }
        } else {
            capture_token!(code => {$($rest)*})
        }
    }};
}
