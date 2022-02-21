use fancy_regex::Regex;

use const_format::{concatcp, formatcp};
use once_cell::sync::Lazy;

macro_rules! tokens {
    ($(#[$doc:meta])* $name:ident: $re:expr;) => {
        $(#[$doc])*
        pub static $name: Lazy<Regex> = Lazy::new(|| Regex::new(concatcp!("^", $re)).unwrap());
    };

    {$(#[$doc:meta])* $name:ident: $re:expr; $($t:tt)*} => {
        tokens!($(#[$doc])* $name: $re;);

        tokens!($($t)*);
    };
}

/// https://www.pyret.org/docs/latest/s_literals.html#%28part._.Names%29
const NAME: &str = "[_a-zA-Z][_a-zA-Z0-9]*(?:-+[_a-zA-Z0-9]+)*";

// gl
tokens! {
    /// https://www.pyret.org/docs/latest/s_literals.html#%28part._.Names%29
    IDENTIFIER: formatcp!("(?<name>{})", NAME);
    /// https://www.pyret.org/docs/latest/A_Tour_of_Pyret.html#%28part._.Variables%29
    VAR_DECLARATION: formatcp!(r"(?s)(?<capture>(((?<var>var)|(?<rec>rec))\s+)?(?<name>{0})(\s*::\s+(?<type>{0}))?\s*=\s*).*", NAME);
    // TYPE_ANN: formatcp!("(?s)(?<name>{0})(<(?<params>(,?\\s*{0})+)+>)?", NAME);
    /// https://www.pyret.org/docs/latest/s_literals.html#%28part._f~3anumber_literals%29
    NUMBER: r"(?<rough>~)?(?<value>[-+]?[0-9]+(?:\.[0-9]+)?(?:[eE][-+]?[0-9]+)?)";
    /// https://www.pyret.org/docs/latest/s_literals.html#%28part._.String_.Literals%29
    SINGLE_QUOTES: r#"'(?<value>(?:\\[01234567]{1,3}|\\x[0-9a-fA-F]{1,2}|\\u[0-9a-fA-f]{1,4}|\\[\\nrt"']|[^\\'\n])*)'"#;
    /// https://www.pyret.org/docs/latest/s_literals.html#%28part._.String_.Literals%29
    DOUBLE_QUOTES: r#""(?<value>(?:\\[01234567]{1,3}|\\x[0-9a-fA-F]{1,2}|\\u[0-9a-fA-f]{1,4}|\\[\\nrt"']|[^\\"\n])*)""#;
    /// https://www.pyret.org/docs/latest/s_literals.html#%28part._.String_.Literals%29
    MULTILINE_QUOTES: r#"(?s)```(?<value>(?:\\[01234567]{1,3}|\\x[0-9a-fA-F]{1,2}|\\u[0-9a-fA-f]{1,4}|\\[\\nrt"'`]|`{1,2}(?!`)|[^`\\])*)```"#;
    /// https://www.pyret.org/docs/latest/Expressions.html#%28elem._%28bnf-prod._%28.Pyret._paren-expr%29%29%29
    PARENTHESIS: r"(?s)\(.*\)";
    /// https://www.pyret.org/docs/latest/Expressions.html#%28part._s~3abinop-expr%29
    OPERATOR: r"(?<operator>[+\-*\/])";
    /// https://www.pyret.org/docs/latest/A_Tour_of_Pyret.html#%28part._functions-tour%29
    FUNCTION: formatcp!(r"(?s)(?<capture>fun\s+(?<name>{0})\((?<params>(,?\s*({0}(\s*::\s*{0})?)?)*)\s*\)\s*:).*end", NAME);
    /// https://www.pyret.org/docs/latest/Expressions.html#%28part._s~3aapp-expr%29
    CALL: formatcp!(r"(?s)(?<callee>{})\((?<args>.*)\)", NAME);
}
