const CPP_VERSION_FLAG: &'static str = "-std=c++20";

fn main() -> miette::Result<()> {
    let rdkit_path = std::path::PathBuf::from("/opt/homebrew/include/rdkit");
    let homebrew_path = std::path::PathBuf::from("/opt/homebrew/include");
    let mut builder = autocxx_build::Builder::new("src/main.rs", &[&rdkit_path, &homebrew_path])
        .extra_clang_args(&[CPP_VERSION_FLAG])
        .build()
        .unwrap();
    builder.flag(CPP_VERSION_FLAG).compile("rdkit-sys");

    Ok(())
}
