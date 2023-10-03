const CPP_VERSION_FLAG: &'static str = "-std=c++20";

fn main() -> miette::Result<()> {
    let include_paths: Vec<_> = vec![
        "/usr/local/include",
        "/usr/local/include/rdkit",
        "/usr/include",
        "/usr/include/rdkit",
        "/opt/homebrew/include/rdkit",
        "/opt/homebrew/include",
    ]
    .into_iter()
    .map(|x| std::path::PathBuf::from(x))
    .collect();

    let mut builder = autocxx_build::Builder::new("src/main.rs", &include_paths)
        .extra_clang_args(&[CPP_VERSION_FLAG])
        .build()
        .unwrap();
    builder.flag(CPP_VERSION_FLAG).compile("rdkit-sys");

    Ok(())
}
