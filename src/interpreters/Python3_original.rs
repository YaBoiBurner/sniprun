//Interpreter:| Python3_original    | python3     |
//############|_____________________|_____________|________________<- delimiters to help formatting,
//###########| Interpretername      | language    | comment
// Keep (but modify the first line after the :) if you wish to have this interpreter listedvia SnipList
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Python3_original {
    support_level: SupportLevel,
    data: DataHolder,
    code: String,
    imports: String,
    main_file_path: String,
}

fn module_used(line: &str, code: &str) -> bool {
    if line.contains("*") {
        return true;
    }
    if line.contains(" as ") {
        if let Some(name) = line.split(" ").last() {
            return code.contains(name);
        }
    }
    for name in line
        .replace(",", " ")
        .replace("from", " ")
        .replace("import ", " ")
        .split(" ")
    {
        if code.contains(name.trim()) {
            return true;
        }
    }
    return false;
}

impl Python3_original {
    pub fn fetch_imports(&mut self) -> std::io::Result<()> {
        if self.support_level < SupportLevel::Line {
            return Ok(());
        }
        //no matter if it fails, we should try to run the rest
        let mut file = File::open(&self.data.filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for line in contents.lines() {
            // info!("lines are : {}", line);
            if line.contains("import ") //basic selection
                && line.trim().chars().next() != Some('#')
            && module_used(line, &contents)
            {
                // embed in try catch blocs in case uneeded module is unavailable
                self.imports = self.imports.clone()
                    + "\n
try:\n" + "\t" + line
                    + "\nexcept:\n\t"
                    + "print()\n";
            }
        }
        Ok(())
    }
}

impl Interpreter for Python3_original {
    fn new_with_level(data: DataHolder, level: SupportLevel) -> Box<Python3_original> {
        //create a subfolder in the cache folder
        let rwd = data.work_dir.clone() + "/rust_original";
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder
            .create(&rwd)
            .expect("Could not create directory for rust-original");

        //pre-create string pointing to main file's and binary's path
        let mfp = rwd.clone() + "/main.py";

        Box::new(Python3_original {
            data,
            support_level: level,
            code: String::from(""),
            imports: String::from(""),
            main_file_path: mfp,
        })
    }

    fn get_name() -> String {
        String::from("Python3_original")
    }

    fn get_supported_languages() -> Vec<String> {
        vec![
            String::from("python"),
            String::from("python3"),
            String::from("py"),
        ]
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
        SupportLevel::File
    }

    fn fetch_code(&mut self) -> Result<(), SniprunError> {
        let _res = self.fetch_imports();
        if !self
            .data
            .current_bloc
            .replace(&[' ', '\t', '\n', '\r'][..], "")
            .is_empty()
            && self.get_current_level() >= SupportLevel::Bloc
        {
            self.code = self.data.current_bloc.clone();
        } else if !self.data.current_line.replace(" ", "").is_empty()
            && self.get_current_level() >= SupportLevel::Line
        {
            self.code = self.data.current_line.clone();
        } else {
            self.code = String::from("");
        }

        if self.get_current_level() >= SupportLevel::File {
            let mut code_to_add = String::new();
            code_to_add.push_str("\n");
            let ranges = self.get_code_dependencies().unwrap_or(vec![]);
            let mut file = File::open(&self.data.filepath).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            for range in ranges {
                for (i, line) in contents.lines().enumerate() {
                    if i < range.start_row {
                        continue;
                    } else if i == range.start_row {
                        code_to_add.push_str(&line[..]); //should start at range.start_col but this break indentation
                        code_to_add.push_str("\n");
                    } else if i == range.end_row {
                        code_to_add.push_str(&line[..range.end_col]);
                        code_to_add.push_str("\n");
                    } else if i > range.end_row {
                        continue;
                    } else {
                        //is in the middle of the range
                        code_to_add.push_str(&line);
                        code_to_add.push_str("\n");
                    }
                }
                code_to_add.push_str("\n"); //separate ranges by newline
            }
            code_to_add.push_str("\n");
            code_to_add = unindent(&format!("{}{}", "\n", code_to_add));
            info!("code to add :\n {}", code_to_add);
            self.code = code_to_add + &unindent(&format!("{}{}", "\n", self.code));
            self.code.push_str("\n");
        }
        info!("got code: {}", self.code);
        Ok(())
    }
    fn add_boilerplate(&mut self) -> Result<(), SniprunError> {
        self.code = self.imports.clone() + &unindent(&format!("{}{}", "\n", self.code.as_str()));
        Ok(())
    }
    fn build(&mut self) -> Result<(), SniprunError> {
        // info!("python code:\n {}", self.code);
        write(&self.main_file_path, &self.code)
            .expect("Unable to write to file for python3_original");
        Ok(())
    }
    fn execute(&mut self) -> Result<String, SniprunError> {
        //run th binary and get the std output (or stderr)
        let output = Command::new("python")
            .arg(&self.main_file_path)
            .output()
            .expect("Unable to start process");
        if output.status.success() {
            return Ok(String::from_utf8(output.stdout).unwrap());
        } else {
            return Err(SniprunError::RuntimeError(
                String::from_utf8(output.stderr.clone())
                    .unwrap()
                    .lines()
                    .last()
                    .unwrap_or(&String::from_utf8(output.stderr).unwrap())
                    .to_owned(),
            ));
        }
    }
}
