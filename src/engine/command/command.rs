use regex::Regex;

#[derive(Debug)]
pub struct Command {
    pub prefix: Option<String>,
    pub root: String,
    pub switch: Option<String>,
    pub page: Option<u16>,
    pub args: Vec<String>,
}

impl Command {
    pub fn parse(cmd: &str) -> Result<Self, String> {
        lazy_static! {
            static ref COMMAND_REGEX: Regex = Regex::new(
               r#"^(?P<prefix>[/\+=@&]?)(?P<root>[^\d\s\.]+)(?P<page>[\d]*)?(?P<switch>\.[^\s\d]+)?(?P<args>.+)*"#
            ).unwrap();
        }

        match COMMAND_REGEX.captures(cmd) {
            Some(captures) => {
                let root = captures
                    .name("root")
                    .expect(&format!("Could not parse command: `{}`", cmd))
                    .as_str()
                    .to_string();
                let args: Vec<String> = captures.name("args").map_or(Vec::new(), |m| {
                    m.as_str()
                        .trim()
                        .split(' ')
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                });
                let page =
                    captures
                        .name("page")
                        .map_or(None, |m| match m.as_str().parse::<u16>() {
                            Ok(page) => Some(page),
                            Err(_) => None,
                        });

                Ok(Self {
                    prefix: captures
                        .name("prefix")
                        .map_or(None, |m| Some(m.as_str().to_string())),
                    root,
                    switch: captures
                        .name("switch")
                        .map_or(None, |m| Some(m.as_str().to_string())),
                    page,
                    args,
                })
            }
            None => Err(format!("Could not parse command: `{}`", cmd)),
        }
    }
}
