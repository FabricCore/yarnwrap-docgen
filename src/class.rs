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
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_static: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_constructor: bool
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
        let class_name = lines.next().unwrap().split(" ").nth(2).unwrap().to_string();

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
                    let mut split = &split_owned[skip_amount..];
                    let (name, args) = fbody.split_once("(").unwrap();

                    let mut is_static = false;
                    let mut is_constructor = false;

                    let value = match split[0] {
                        "public" => {
                            is_constructor = true;
                            class_name.clone()
                        }
                        "static" => {
                            is_static = true;
                            split = &split[1..];
                            split[0].to_string()
                        }
                        _ => split[0].to_string()
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
                        is_static,
                        is_constructor,
                    })
                }
            })
            .collect();

        Some(Class {
            qualified_name: format!("{}.{}", package, class_name),
            wraps: format!(
                "{}.{}",
                package.replacen("yarnwrap", "net.minecraft", 1),
                class_name
            ),
            package,
            name: class_name,
            methods,
        })
    }
}
