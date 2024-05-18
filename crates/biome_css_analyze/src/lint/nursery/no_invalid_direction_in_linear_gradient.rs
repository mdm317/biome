use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{CssFunction, CssParameter};
use biome_rowan::AstNode;
use lazy_static::lazy_static;
use regex::Regex;

declare_rule! {
    /// Disallow non-standard direction values for linear gradient functions.
    ///
    /// A valid and standard direction value is one of the following:
    /// - an angle
    /// - to plus a side-or-corner (to top, to bottom, to left, to right; to top right, to right top, to bottom left, etc.)
    ///
    /// A common mistake (matching outdated non-standard syntax) is to use just a side-or-corner without the preceding to.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .foo { background: linear-gradient(top, #fff, #000); }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// .foo { background: linear-gradient(45, #fff, #000); }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .foo { background: linear-gradient(to top, #fff, #000); }
    /// ```
    ///
    /// ```css
    /// .foo { background: linear-gradient(45deg, #fff, #000); }
    /// ```
    ///
    pub NoInvalidDirectionInLinearGradient {
        version: "next",
        name: "noInvalidDirectionInLinearGradient",
        language: "css",
        recommended: true,
        sources: &[RuleSource::Stylelint("function-linear-gradient-no-nonstandard-direction")],
    }
}

lazy_static! {
    pub static ref GET_PREFIX_REGEX: Regex = Regex::new(r"^(-\w+-)").unwrap();
    pub static ref LINEAR_GRADIENT_FUNCTION_NAME: Regex =
        Regex::new(r"^(?i)(-webkit-|-moz-|-o-|-ms-)?linear-gradient").unwrap();
    pub static ref IN_KEYWORD: Regex = Regex::new(r"(?i)\bin\b").unwrap();
    pub static ref ANGLE: Regex = Regex::new(r"^[\d.]+(?:deg|grad|rad|turn)$").unwrap();
    pub static ref DIRECTION: Regex = Regex::new(r"(?i)top|left|bottom|right").unwrap();
    pub static ref DIRECTION_WITH_TO: Regex = Regex::new(&format!(
        r"(?i)^to ({})(?: ({}))?$",
        DIRECTION.as_str(),
        DIRECTION.as_str()
    ))
    .unwrap();
    pub static ref DIRECTION_WITHOUT_TO: Regex = Regex::new(&format!(
        r"(?i)^({})(?: ({}))?$",
        DIRECTION.as_str(),
        DIRECTION.as_str()
    ))
    .unwrap();
    pub static ref DIGIT: Regex = Regex::new(r"[\d.]").unwrap();
}

impl Rule for NoInvalidDirectionInLinearGradient {
    type Query = Ast<CssFunction>;
    type State = CssParameter;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let node_name = node.name().ok()?.text();
        let is_linear_gradient = LINEAR_GRADIENT_FUNCTION_NAME.is_match(&node_name);
        if !is_linear_gradient {
            return None;
        }
        let css_parameter = node.items();

        if let Some(Ok(first_css_parameter)) = css_parameter.into_iter().next() {
            let first_css_parameter_text = first_css_parameter.text();
            if IN_KEYWORD.is_match(&first_css_parameter_text) {
                return None;
            }
            if let Some(first_char) = first_css_parameter_text.chars().next() {
                if first_char.is_ascii_digit() {
                    if ANGLE.is_match(&first_css_parameter_text) {
                        return None;
                    }
                    return Some(first_css_parameter);
                }
            }
            if !DIRECTION.is_match(&first_css_parameter_text) {
                return None;
            }
            let has_prefix = LINEAR_GRADIENT_FUNCTION_NAME
                .captures(&node_name)
                .and_then(|caps| caps.get(1))
                .is_some();
            if !is_standdard_direction(&first_css_parameter_text, !has_prefix) {
                return Some(first_css_parameter);
            }
            return None;
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected nonstandard direction"
                },
            ).note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/linear-gradient">"MDN web docs"</Hyperlink>" for more details."
            })
        )
    }
}

fn is_standdard_direction(direction: &str, with_to_prefix: bool) -> bool {
    let matches = match with_to_prefix {
        true => DIRECTION_WITH_TO.captures(direction),
        false => DIRECTION_WITHOUT_TO.captures(direction),
    };
    if let Some(matches) = matches {
        match (matches.get(1), matches.get(2)) {
            (Some(_), None) => {
                return true;
            }
            (Some(first_direction), Some(second_direction)) => {
                if first_direction.as_str() != second_direction.as_str() {
                    return true;
                }
            }
            _ => return true,
        }
    }
    false
}
