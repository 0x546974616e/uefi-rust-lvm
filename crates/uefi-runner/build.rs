use std::{
  fs::File,
  io::{BufRead, BufReader},
  path::Path,
};

fn set_uefi_env(path: &Path) {
  let reader = BufReader::new(File::open(path).unwrap());
  for line in reader.lines().map(Result::unwrap) {
    if let Some((key, mut value)) = line.split_once('=') {
      if let Some((actual_value, _)) = value.split_once('#') {
        value = actual_value;
      }

      let (key, value) = (key.trim(), value.trim());
      println!("cargo:rustc-env={key}={value}");
    }
  }
}

fn main() {
  let manifest_dir = env!("CARGO_MANIFEST_DIR");
  let uefi_env = Path::new(manifest_dir).join("uefi.env");
  println!("cargo::rerun-if-changed={}", uefi_env.display());
  set_uefi_env(&uefi_env);

  // On a juste besoin d'un dossier temporaire pour l'ESP.
  let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
  println!("cargo:rustc-env=OUT_DIR={out_dir}");
}
