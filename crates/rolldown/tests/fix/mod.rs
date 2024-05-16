#[cfg(test)]
mod fix_package_json_test_case {
  use rolldown::Bundler;
  use rolldown_common::BundlerOptions;
  use std::env;
  use std::ffi::OsString;
  use std::path::Path;

  pub fn path_resolve(path: &str) -> String {
    let work_cwd = {
      match env::var("CARGO_MANIFEST_DIR") {
        Ok(_val) => env!("CARGO_MANIFEST_DIR").to_string(),
        Err(_) => match std::env::current_exe() {
          Ok(val) => val.parent().unwrap().to_str().unwrap().to_string(),
          Err(_) => std::env::current_dir().unwrap().to_str().unwrap().to_string(),
        },
      }
    };
    let os_work_cwd = OsString::from(work_cwd);
    Path::new(&os_work_cwd).join(path).into_os_string().into_string().unwrap()
  }

  #[tokio::test(flavor = "multi_thread")]
  async fn test_package_json_browser_index_no_ext() {
    let json_content = r#"
{
    "input": [
      {
        "name": "entry",
        "import": "./src/entry.js"
      }
    ]
}
        "#;
    let mut option: BundlerOptions = serde_json::from_str(json_content).unwrap();
    let dir = path_resolve(r"tests/esbuild/packagejson/test_package_json_exports_alternatives");
    option.cwd = Some(std::path::PathBuf::from(dir));
    // println!("option is ->{:#?}", option);
    let mut bundler = Bundler::new(option);
    bundler.write().await.unwrap();
  }
}
