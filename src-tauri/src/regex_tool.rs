use regex::{Regex, RegexBuilder};

pub fn match_text(input: &str, regex_str: &str, options: Vec<&str>) -> Result<String, String> {
    let mut builder = RegexBuilder::new(regex_str);
    let mut options_iter = options.into_iter();
    if options_iter.any(|p| p == "i") {
        builder.case_insensitive(true);
    }
    match builder.build() {
        Ok(re) => {
            // for mat in re.find_iter(input) {
            //     println!("{:?}", mat.as_str());
            // }
            // let result = re
            //     .find_iter(input)
            //     .into_iter()
            //     .map(|p| p.as_str())
            //     // .copied()
            //     .intersperse("\r")
            //     .collect::<String>();

            let mut result: String = "".to_string();
            for (index, item) in re
                .find_iter(input)
                .into_iter()
                .map(|p| p.as_str())
                .enumerate()
            {
                if index > 0 {
                    result.push_str("\r\n");
                }
                result.push_str(item);
            }

            Ok(result)
        }
        Err(err) => Err(format!("{}", err)),
    }
}

pub fn replace_text(
    input: &str,
    regex_str: &str,
    replace_str: &str,
    options: Vec<&str>,
) -> Result<String, String> {
    let mut builder = RegexBuilder::new(regex_str);
    let mut options_iter = options.into_iter();
    if options_iter.any(|p| p == "i") {
        builder.case_insensitive(true);
    }

    match builder.build() {
        Ok(re) => {
            if options_iter.any(|p| p == "g") {
                let result = re.replace_all(input, replace_str);
                Ok(result.into_owned())
            } else {
                let result = re.replace(input, replace_str);
                Ok(result.into_owned())
            }
        }
        Err(err) => Err(format!("{}", err)),
    }
}
