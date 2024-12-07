mod testing {
    #[test]
    fn run_main_of_all_crates_with_day_in_name() {
        let crates = std::fs::read_dir(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "crates"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_dir())
            .filter(|path| path.file_name().unwrap().to_str().unwrap().contains("day"))
            .collect::<Vec<_>>();
        let time = std::time::Instant::now();
        for crate_path in crates {
            let crate_name = crate_path.file_name().unwrap().to_str().unwrap();
            let main_path = crate_path.join("src/main.rs");
            if main_path.exists() {
                println!("Running main of {}...", crate_name);
                let output = std::process::Command::new("cargo")
                    .arg("run")
                    .arg("--package")
                    .arg(crate_name)
                    .arg("--bin")
                    .arg(crate_name)
                    .output()
                    .unwrap();
                if output.status.success() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    println!("{} failed!", crate_name);
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
        println!("Total time: {:.2?}", time.elapsed());
    }
}
