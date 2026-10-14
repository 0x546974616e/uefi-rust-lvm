use std::{
  env,
  ffi::OsStr,
  fs, io,
  path::{Path, PathBuf},
  process::{Command, ExitCode, ExitStatus, Stdio},
};

fn is_uefi_extension(string: &OsStr) -> bool {
  string.eq_ignore_ascii_case("efi")
}

fn get_efi_binary_path() -> io::Result<PathBuf> {
  let mut arguments = env::args_os().skip(1);
  let Some(efi_binary) = arguments.next().map(PathBuf::from) else {
    return Err(io::Error::other("Missing EFI binary argument"));
  };

  if arguments.next().is_some() {
    // NOTE: Pour l'instant on s'attend qu'à ça.
    return Err(io::Error::other("Unexpected extra arguments"));
  }

  if !efi_binary.extension().is_some_and(is_uefi_extension) {
    return Err(io::Error::other(format!(
      "Expected an EFI binary, got: {}",
      efi_binary.display(),
    )));
  }

  Ok(efi_binary)
}

fn prepare_esp_directory(efi_binary: &Path) -> io::Result<PathBuf> {
  let esp_directory = Path::new(env!("OUT_DIR")).join("ESP");
  let boot_directory = esp_directory.join("EFI/BOOT");
  let bootx64 = boot_directory.join("BOOTx64.EFI");

  fs::create_dir_all(boot_directory)?;
  if let Err(error) = fs::copy(efi_binary, &bootx64) {
    return Err(io::Error::new(
      error.kind(),
      format!(
        "Failed to copy {} to {}: {error}",
        efi_binary.display(),
        bootx64.display(),
      ),
    ));
  }

  Ok(esp_directory)
}

fn run_qemu(esp_directory: &Path) -> io::Result<ExitStatus> {
  // TODO: Escape comma with `,,` in QEMU path.
  // TODO(?): QEMU `-blockdev` ? `-device` ?
  Command::new(env!("QEMU"))
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .args([
      // Set the name of the guest.
      "-name",
      "UEFI",
      // The recommended way to use OVMF is with a pflash parameter.
      // (The following is not recommended `-bios /usr/share/ovmf/OVMF.fd`.)
      // TODO(?): OVMF image can be splitted into CODE and VARS section.
      "-drive",
      &format!(
        "if=pflash,format=raw,unit=0,file={},readonly=on",
        Path::new(env!("OVMF")).display()
      ),
      // QEMU can emulate a virtual drive with a FAT filesystem.
      // https://en.wikibooks.org/wiki/QEMU/Devices/Storage#Virtual_FAT_filesystem_(VVFAT)
      "-drive",
      &format!(
        "file=fat:rw:{},media=disk,format=raw",
        esp_directory.display()
      ),
      // Ensure that the mouse cursor is visible when interacting with the
      // virtual machine.
      "-display",
      "default,show-cursor=on",
      // Prevent recent versions of QEMU from attempting a PXE (network)
      // boot when no boot disk is found.
      "-net",
      "none",
    ])
    .status()
}

fn run() -> io::Result<ExitStatus> {
  let efi_binary = get_efi_binary_path()?;
  let esp_directory = prepare_esp_directory(&efi_binary)?;
  run_qemu(&esp_directory)
}

fn main() -> ExitCode {
  match run() {
    Ok(status) if status.success() => ExitCode::SUCCESS,
    Ok(status) => {
      eprintln!("QEMU exited with: {status}");
      ExitCode::FAILURE
    }
    Err(error) => {
      eprintln!("Error: {error}");
      ExitCode::FAILURE
    }
  }
}
