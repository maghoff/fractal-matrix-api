extern crate gettextrs;
extern crate regex;
use self::gettextrs::gettext;
use self::regex::Regex;
use self::regex::Captures;


pub fn i18n(format: &str) -> String {
    gettext(format)
}


pub fn i18n_f(format: &str, args: &[&str]) -> String {
    let s = gettext(format);
    let mut parts = s.split("{}");
    let mut output = parts.next().unwrap_or("").to_string();
    for (p, a) in parts.zip(args.iter()) {
        output += &(a.to_string() + &p.to_string());
    }
    output
}


pub fn i18n_k(format: &str, kwargs: &[(&str, &str)]) -> String {
    let mut s = gettext(format);
    for (k, v) in kwargs {
        let re = Regex::new(&format!("\\{{{}\\}}", k)).unwrap();
        let x = v.to_string().clone();
        s = re.replace_all(&s, |_: &Captures| x.clone()).to_string();
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_i18n() {
        let out = i18n("translate1");
        assert_eq!(out, "translate1");
    }

    #[test]
    fn test_i18n_f() {
        let out = i18n_f("{} param", &["one"]);
        assert_eq!(out, "one param");

        let out = i18n_f("middle {} param", &["one"]);
        assert_eq!(out, "middle one param");

        let out = i18n_f("end {}", &["one"]);
        assert_eq!(out, "end one");

        let out = i18n_f("multiple {} and {}", &["one", "two"]);
        assert_eq!(out, "multiple one and two");
    }

    #[test]
    fn test_i18n_k() {
        let out = i18n_k("{one} param", &[("one", "one")]);
        assert_eq!(out, "one param");

        let out = i18n_k("middle {one} param", &[("one", "one")]);
        assert_eq!(out, "middle one param");

        let out = i18n_k("end {one}", &[("one", "one")]);
        assert_eq!(out, "end one");

        let out = i18n_k("multiple {one} and {two}", &[("one", "1"), ("two", "two")]);
        assert_eq!(out, "multiple 1 and two");

        let out = i18n_k("multiple {two} and {one}", &[("one", "1"), ("two", "two")]);
        assert_eq!(out, "multiple two and 1");

        let out = i18n_k("multiple {one} and {one}", &[("one", "1"), ("two", "two")]);
        assert_eq!(out, "multiple 1 and 1");
    }
}
