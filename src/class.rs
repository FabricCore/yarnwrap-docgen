use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Class {
    pub package: String,
    pub name: String,
    pub methods: Vec<Method>,

    pub qualified_name: String,
    pub wraps: String,
}

#[derive(Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub value: String,
    pub args: Vec<Argument>,
}

#[derive(Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub value: String,
}

impl Class {
    pub fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        let package = lines
            .next()
            .unwrap()
            .split_once(" ")
            .unwrap()
            .1
            .trim_end_matches(";")
            .to_string();
        let name = lines.next().unwrap().split(" ").nth(2).unwrap().to_string();

        let methods = lines
            .filter_map(|line| {
                if line.starts_with("//") || line.trim().is_empty() || line == "}" {
                    None
                } else {
                    let split = line
                        .split(" ")
                        .skip(1)
                        .take_while(|word| !word.contains("{"))
                        .collect::<Vec<_>>();
                    let fbody = split[1..].join(" ");
                    let (name, args) = fbody.split_once("(").unwrap();

                    Some(Method {
                        name: name.to_string(),
                        value: split[0].to_string(),
                        args: args
                            .trim_end_matches(")")
                            .split(",")
                            .filter_map(|chunk| {
                                if chunk.is_empty() {
                                    None
                                } else {
                                    let (value, name) = chunk.split_once(" ").unwrap();
                                    Some(Argument {
                                        name: name.trim().to_string(),
                                        value: value.trim().to_string(),
                                    })
                                }
                            })
                            .collect(),
                    })
                }
            })
            .collect();

        Class {
            qualified_name: format!("{}.{}", package, name),
            wraps: format!(
                "{}.{}",
                package.replacen("yarnwrap", "net.minecraft", 1),
                name
            ),
            package,
            name,
            methods,
        }
    }
}
