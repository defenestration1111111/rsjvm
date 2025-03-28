use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;
use std::str;

#[derive(Clone)]
pub struct JavaCompilerOptions {
    flags: Vec<String>,
}

impl JavaCompilerOptions {
    pub fn new() -> Self {
        Self { flags: Vec::new() }
    }

    pub fn add_flag(&mut self, flag: &str) -> &mut Self {
        self.flags.push(flag.to_string());
        self
    }

    pub fn to_args(&self) -> Vec<String> {
        self.flags.clone()
    }

    pub fn use_g(&mut self) -> &mut Self {
        self.add_flag("-g")
    }

    #[allow(dead_code)]
    pub fn use_classpath(&mut self, classpath: &str) -> &mut Self {
        self.add_flag(&format!("-classpath {}", classpath))
    }

    pub fn use_output_dir(&mut self, output_dir: &str) -> &mut Self {
        self.add_flag("-d").add_flag(output_dir)
    }

    #[allow(dead_code)]
    pub fn custom_flag(&mut self, flag: &str) -> &mut Self {
        self.add_flag(flag)
    }
}

impl Default for JavaCompilerOptions {
    fn default() -> Self {
        Self::new().use_g().use_output_dir("target/classes").clone()
    }
}

pub fn compile_java_file(path: &Path, options: &JavaCompilerOptions) -> io::Result<()> {
    let mut command = Command::new("javac");

    for flag in options.to_args() {
        command.arg(flag);
    }

    command.arg(path);

    let status = command.status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to compile file: {}", path.display()),
        ));
    }

    Ok(())
}

pub fn check_javac_version() -> Result<(), Box<dyn Error>> {
    let output = Command::new("javac").arg("-version").output()?;

    let stderr = str::from_utf8(&output.stderr)?.trim();
    let stdout = str::from_utf8(&output.stdout)?.trim();
    let version_output = if !stderr.is_empty() { stderr } else { stdout };

    let version_part =
        version_output.split_whitespace().nth(1).ok_or("Failed to parse javac version")?;

    let major_version: u32 =
        version_part.split('.').next().ok_or("Invalid javac version format")?.parse()?;

    if major_version < 22 {
        Err(format!("javac version is too old: {} (requires 22 or higher)", major_version).into())
    } else {
        Ok(())
    }
}

pub fn read_class_file(path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub struct CompileConfig {
    java_file_name: String,
    options: JavaCompilerOptions,
}

impl CompileConfig {
    pub fn new(java_file_name: String) -> Self {
        Self { java_file_name, options: JavaCompilerOptions::default() }
    }

    #[allow(dead_code)]
    pub fn with_options(&mut self, options: JavaCompilerOptions) -> &mut Self {
        self.options = options;
        self
    }

    pub fn run(&self) -> io::Result<Vec<u8>> {
        let resources_dir = Path::new("tests/resources");
        let java_file = resources_dir.join(&self.java_file_name);

        compile_java_file(&java_file, &self.options)?;

        let output_dir = Path::new("target/classes");
        let class_file_path = output_dir.join(self.java_file_name.replace(".java", ".class"));
        let bytes = read_class_file(&class_file_path)?;
        Ok(bytes)
    }
}
