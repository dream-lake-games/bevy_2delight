use syn::*;

macro_rules! find_required_attr {
    ($variant:expr, $attr:literal) => {{
        $variant
            .attrs
            .iter()
            .find(|a| a.path.is_ident($attr))
            .expect(format!("Attribute {} is required", $attr).as_str())
    }};
}
pub(crate) use find_required_attr;

macro_rules! find_optional_attr {
    ($variant:expr, $attr:literal) => {{
        $variant.attrs.iter().find(|a| a.path.is_ident($attr))
    }};
}
pub(crate) use find_optional_attr;

/// Matches attributes of the form #[attr(Ident)]
pub(crate) fn get_single_ident(name: &str, attr: &Attribute) -> Ident {
    match attr
        .parse_meta()
        .expect(format!("Cannot parse #[{name}...] attribute").as_str())
    {
        Meta::List(MetaList { nested, .. }) if nested.len() == 1 => match nested.first().unwrap() {
            NestedMeta::Meta(Meta::Path(p)) => p
                .get_ident()
                .expect(format!(r#"#[{name}] should have ident form"#).as_str())
                .clone(),
            _ => panic!("#[{name}...] attribute should take the form #[{name}(ident)]"),
        },
        _ => panic!("#[{name}...] attribute should take the form #[{name}(ident)]"),
    }
}

/// Matches attributes of the form #[attr("lit_str")]
pub(crate) fn get_single_lit_str(name: &str, attr: &Attribute) -> String {
    match attr
        .parse_meta()
        .expect(format!("Cannot parse #[{name}...] attribute").as_str())
    {
        Meta::List(MetaList { nested, .. }) if nested.len() == 1 => match nested.first().unwrap() {
            NestedMeta::Lit(Lit::Str(lit_str)) => lit_str.value(),
            _ => panic!(r#"#[{name}...] attribute should take the form #[{name}("lit_str")]"#),
        },
        _ => {
            panic!(r#"#[{name}...] attribute should take the form #[{name}("lit_str")]"#)
        }
    }
}

/// Matches attributes of the form #[attr(int)]
pub(crate) fn get_single_lit_int<N>(name: &str, attr: &Attribute) -> N
where
    N: std::str::FromStr,
    N::Err: std::fmt::Display,
{
    match attr
        .parse_meta()
        .expect(format!("Cannot parse #[{name}...] attribute").as_str())
    {
        Meta::List(MetaList { nested, .. }) if nested.len() == 1 => match nested.first().unwrap() {
            NestedMeta::Lit(Lit::Int(lit_int)) => lit_int
                .base10_parse::<N>()
                .expect(format!(r#"#[{name}...] attribute cannot be parsed to a number"#).as_str()),
            _ => panic!(r#"#[{name}...] attribute should take the form #[{name}(lit_int)]"#),
        },
        _ => {
            panic!(r#"#[{name}...] attribute should take the form #[{name}(lit_int)]"#)
        }
    }
}

/// Matches attributes of the form #[attr(int)]
pub(crate) fn get_pair_lit_int<N>(name: &str, attr: &Attribute) -> (N, N)
where
    N: std::str::FromStr,
    N::Err: std::fmt::Display,
{
    match attr
        .parse_meta()
        .expect(format!("Cannot parse #[{name}...] attribute").as_str())
    {
        Meta::List(MetaList { nested, .. }) if nested.len() == 2 => {
            let mut nested_iter = nested.iter();
            let thing1 = nested_iter.next().unwrap();
            let thing2 = nested_iter.next().unwrap();
            match (thing1, thing2) {
                (NestedMeta::Lit(Lit::Int(lit_int1)), NestedMeta::Lit(Lit::Int(lit_int2))) => (
                    lit_int1.base10_parse::<N>().expect(
                        format!(r#"#[{name}...] attribute cannot be parsed to a pair of numbers"#)
                            .as_str(),
                    ),
                    lit_int2.base10_parse().expect(
                        format!(r#"#[{name}...] attribute cannot be parsed to a pair of numbers"#)
                            .as_str(),
                    ),
                ),
                _ => panic!(
                    r#"#[{name}...] attribute should take the form #[{name}(lit_int, lit_int)]"#
                ),
            }
        }
        _ => {
            panic!(r#"#[{name}...] attribute should take the form #[{name}(lit_int, lit_int)]"#)
        }
    }
}

/// Matches attributes of the form #[attr(int)]
pub(crate) fn get_single_lit_float<N>(name: &str, attr: &Attribute) -> N
where
    N: std::str::FromStr,
    N::Err: std::fmt::Display,
{
    match attr
        .parse_meta()
        .expect(format!("Cannot parse #[{name}...] attribute").as_str())
    {
        Meta::List(MetaList { nested, .. }) if nested.len() == 1 => match nested.first().unwrap() {
            NestedMeta::Lit(Lit::Float(lit_float)) => lit_float
                .base10_parse::<N>()
                .expect(format!(r#"#[{name}...] attribute cannot be parsed to a float"#).as_str()),
            _ => panic!(r#"#[{name}...] attribute should take the form #[{name}("lit_float")]"#),
        },
        _ => {
            panic!(r#"#[{name}...] attribute should take the form #[{name}("lit_float")]"#)
        }
    }
}
