const CPP_VERSION_FLAG: &'static str = "-std=c++17";

fn main() -> miette::Result<()> {
    let mut lib_paths = vec![];
    let mut include_paths = vec![];

    if let Ok(conda_prefix) = std::env::var("CONDA_PREFIX") {
        include_paths.push(format!("{conda_prefix}/include"));
        include_paths.push(format!("{conda_prefix}/include/rdkit"));
        lib_paths.push(format!("{conda_prefix}/lib"));
    }

    // include_paths.push("/usr/include".to_string());
    // include_paths.push("/usr/local/include".to_string());

    let include_paths: Vec<std::path::PathBuf> = include_paths
        .into_iter()
        .map(std::path::PathBuf::from)
        .collect();

    for path in lib_paths {
        println!("cargo:rustc-link-search=native={}", path);
    }

    let mut builder = autocxx_build::Builder::new("src/main.rs", &include_paths)
        .extra_clang_args(&[CPP_VERSION_FLAG])
        .build()
        .unwrap();
    builder.flag(CPP_VERSION_FLAG).compile("rdkit-sys");

    Ok(())
}
