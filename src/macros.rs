/// Print a line using a [color](console::utils::StyledObject) from the `console` crate
///
/// # Examples
///
/// ```
/// // Normal
/// use crate::macros;
/// color_print!(green, "Hello!");
/// color_print!(green, "Hello, {}", name);
/// ```

#[macro_export]
macro_rules! color_print {
    ( $color:ident, $l:literal, $( $x:expr),+ ) => {
        println!(
            "{}",
            style(format!($l, ($($x),+))).$color()
        )
    };
    ( $color:ident, $l:literal ) => {
        println!(
            "{}",
            style(format!($l)).$color()
        )
    };
}
