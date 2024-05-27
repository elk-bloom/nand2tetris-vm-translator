use std::{path::Path, process::Command};

pub fn test_project(project_path_str: &str) {
    let project_path = Path::new(project_path_str);
    if !project_path.is_file() {
        todo!()
    }
    let vm_file_name = project_path;
    let tst_file_name = project_path.with_extension("tst");

    let translation_output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-t")
        .arg(vm_file_name)
        .output()
        .unwrap();

    assert!(
        translation_output.status.success(),
        "VM translation failure"
    );

    let compare_output = if cfg!(target_os = "windows") {
        let bat_file_name = Path::new("tests/tools/CPUEmulator.bat");
        Command::new(bat_file_name)
            .arg(tst_file_name)
            .output()
            .unwrap()
    } else {
        let sh_file_name = Path::new("tests/tools/CPUEmulator.sh");
        Command::new("bash")
            .arg(sh_file_name)
            .arg(tst_file_name)
            .output()
            .unwrap()
    };

    assert!(compare_output.status.success(), "Comparison failed")
}
