#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Rust_original {
    support_level: SupportLevel,
    data: DataHolder,
    code: String,

    ///specific to rust
    rust_work_dir: String,
    bin_path: String,
    main_file_path: String,
}
impl ReplLikeInterpreter for Rust_original {}
impl Interpreter for Rust_original {
    fn new_with_level(data: DataHolder, support_level: SupportLevel) -> Box<Rust_original> {
        //create a subfolder in the cache folder
        let rwd = data.work_dir.clone() + "/rust_original";
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder
            .create(&rwd)
            .expect("Could not create directory for rust-original");

        //pre-create string pointing to main file's and binary's path
        let mfp = rwd.clone() + "/main.rs";
        let bp = String::from(&mfp[..mfp.len() - 3]); // remove extension so binary is named 'main'
        Box::new(Rust_original {
            data,
            support_level,
            code: String::from(""),
            rust_work_dir: rwd,
            bin_path: bp,
            main_file_path: mfp,
        })
    }

    fn get_supported_languages() -> Vec<String> {
        vec![
            String::from("Rust"),
            String::from("rust"),
            String::from("rust-lang"),
            String::from("rs"),
        ]
    }

    fn get_name() -> String {
        String::from("Rust_original")
    }

    fn default_for_filetype() -> bool {
        true
    }
    fn get_current_level(&self) -> SupportLevel {
        self.support_level
    }
    fn set_current_level(&mut self, level: SupportLevel) {
        self.support_level = level;
    }

    fn get_data(&self) -> DataHolder {
        self.data.clone()
    }

    fn get_max_support_level() -> SupportLevel {
        SupportLevel::Bloc
    }

    fn fetch_code(&mut self) -> Result<(), SniprunError> {
        //add code from data to self.code
        if !self
            .data
            .current_bloc
            .replace(&[' ', '\t', '\n', '\r'][..], "")
            .is_empty()
            && self.support_level >= SupportLevel::Bloc
        {
            self.code = self.data.current_bloc.clone();
        } else if !self.data.current_line.replace(" ", "").is_empty()
            && self.support_level >= SupportLevel::Line
        {
            self.code = self.data.current_line.clone();
        } else {
            self.code = String::from("");
        }
        Ok(())
    }

    fn add_boilerplate(&mut self) -> Result<(), SniprunError> {
        self.code = String::from("fn main() {") + &self.code + "}";
        Ok(())
    }

    fn build(&mut self) -> Result<(), SniprunError> {
        //write code to file
        let mut _file =
            File::create(&self.main_file_path).expect("Failed to create file for rust-original");
        write(&self.main_file_path, &self.code).expect("Unable to write to file for rust-original");

        //compile it (to the bin_path that arleady points to the rigth path)
        let output = Command::new("rustc")
            .arg("-O")
            .arg("--out-dir")
            .arg(&self.rust_work_dir)
            .arg(&self.main_file_path)
            .output()
            .expect("Unable to start process");

        //TODO if relevant, return the error number (parse it from stderr)
        if !output.status.success() {
            let error_message = String::from_utf8(output.stderr).unwrap();
            //
            //take first line and remove first 'error' word (redondant)
            let first_line = error_message
                .lines()
                .next()
                .unwrap_or_default()
                .trim_start_matches("error: ")
                .trim_start_matches("error");
            return Err(SniprunError::CompilationError(first_line.to_owned()));
        } else {
            return Ok(());
        }
    }

    fn execute(&mut self) -> Result<String, SniprunError> {
        //run th binary and get the std output (or stderr)
        let output = Command::new(&self.bin_path)
            .output()
            .expect("Unable to start process");
        if output.status.success() {
            return Ok(String::from_utf8(output.stdout).unwrap());
        } else {
            return Err(SniprunError::RuntimeError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }
    }
}

#[cfg(test)]
mod test_rust_original {
    use super::*;
    use crate::error::SniprunError;

    #[test]
    fn run_all() {
        //nececssary to run sequentially
        //because of file access & shared things
        simple_print();
        runtime_error();
    }
    fn simple_print() {
        let mut data = DataHolder::new();
        data.current_bloc = String::from("println!(\"HW, 1+1 = {}\", 1+1);");
        let mut interpreter = Rust_original::new(data);
        let res = interpreter.run();

        // should panic if not an Ok()
        let string_result = res.unwrap();
        assert_eq!(string_result, "HW, 1+1 = 2\n");
    }

    fn runtime_error() {
        let mut data = DataHolder::new();
        data.current_bloc = String::from(
            "
            let mock_input = \"153.2\";
            let parsed = mock_input.parse::<i32>().unwrap();
            ",
        );
        let expected = String::from("ParseIntError { kind: InvalidDigit }");
        let mut interpreter = Rust_original::new(data);
        let res = interpreter.run();

        assert!(res.is_err());
        // should panic if not an Err()
        if let Err(e) = res {
            match e {
                SniprunError::RuntimeError(full_message) => {
                    assert!(full_message.contains(&expected))
                }
                _ => panic!("Not the right error message"),
            }
        }
    }
}
