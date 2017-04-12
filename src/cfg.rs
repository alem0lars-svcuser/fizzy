use envy;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct FizzyEnvVars {
    #[serde(rename = "fizzy_cfg")]
    pub cfg_file_path: String,
    #[serde(rename = "fizzy_verbosity")]
    pub verbosity_level: Option<u64>,
    #[serde(rename = "fizzy_simulate")]
    pub simulate: Option<bool>,
}

pub struct FizzyCfg<'a> {
    pub cfg_file_path: Option<&'a Path>,
    pub verbosity_level: Option<u64>,
    pub simulate: Option<bool>,
}

/// Supported file formats (and relative valid extensions).
// const FILE_FORMATS_EXTS: &'static [(FileFormat, &'static [&'static str])] =
//     &[(FileFormat::Json, &["json"]),
//       (FileFormat::Toml, &["toml"]),
//       (FileFormat::Yaml, &["yaml", "yml"])];

//fn read_cfg(file_path: String, env_vars_prefix: String) ->  {
//let mut cfg: Config = Config::new();

//cfg.merge(Environment::new(env_vars_prefix.as_str())).unwrap();

//for &(file_format, exts) in FILE_FORMATS_EXTS {
// for ext in exts {
//     let file_path_str = format!("{}.{}", file_path, ext); // TODO rename
//     let file_path = Path::new(&file_path_str);
//     if file_path.is_file() {
//         //cfg.merge(File::new(&file_path_str, file_format)).unwrap();
//     }
// }
//}

//    return cfg;
//}
/// Read fizzy environment variables.
pub fn read_fizzy_env_vars() -> Option<FizzyEnvVars> {
    return match envy::from_env::<FizzyEnvVars>() {
        Ok(env_vars) => Some(env_vars),
        Err(error) => {
            error!("{:#?}", error);
            None
        }
    };
}

/// Read fizzy configuration from `file_path` (default: guess the file path).
pub fn read_fizzy_cfg<'a>(file_path: Option<&'a Path>) -> FizzyCfg<'a> {
    // let cfg = read_cfg(file_path, env_vars_prefix);
    return FizzyCfg {
        cfg_file_path: None,
        verbosity_level: None,
        simulate: None,
    };
}