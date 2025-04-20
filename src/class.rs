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
    pub fn from_str(s: &str) -> Option<Self> {
        let mut lines = s.lines();
        if !s.lines().nth(1).unwrap().contains("class") {
            return None;
        }

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
                    let split_owned = line
                        .split(" ")
                        .take_while(|word| !word.contains("{"))
                        .collect::<Vec<_>>();

                    let skip_amount = if split_owned[1].contains("(") { 0 } else { 1 };
                    let fbody = split_owned[skip_amount + 1..].join(" ");
                    let split = &split_owned[skip_amount..];
                    let (name, args) = fbody.split_once("(").unwrap();

                    let value = if split[0] == "public" {
                        format!("(constructor)")
                    } else {
                        split[0].to_string()
                    };

                    Some(Method {
                        name: name.to_string(),
                        value,
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

        Some(Class {
            qualified_name: format!("{}.{}", package, name),
            wraps: format!(
                "{}.{}",
                package.replacen("yarnwrap", "net.minecraft", 1),
                name
            ),
            package,
            name,
            methods,
        })
    }
}
