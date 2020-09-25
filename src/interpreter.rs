use crate::error::SniprunError;
use crate::DataHolder;
use log::info;
use neovim_lib::NeovimApi;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum SupportLevel {
    ///no support
    Unsupported = 0,
    ///run the code in the line, all is contained within and no variable declaration/initialisation happens before
    Line = 1,
    ///run a bloc of code, same limitations as Line
    Bloc = 2,
    ///support exterior imports
    Import = 5,
    ///run a line/bloc of code, but include variable/functions definitions found in the file
    File = 10,
    ///run a line/bloc of code, but include (only needed) variable/functions found in the project
    Project = 20,
    ///Run a line/bloc of code, but include variable/function from the project and project or system-wide dependencies
    System = 30,

    ///selected: don't use this support level, it is meant to communicate user's config choices
    Selected = 255,
}

///This is the trait all interpreters must implement.
///The launcher run fucntions new() and run() from this trait.
pub trait Interpreter {
    //create
    fn new(data: DataHolder) -> Box<Self> {
        Self::new_with_level(data, Self::get_max_support_level())
    }
    /// This implies your interpreter struct should have a 'data' and a 'support_level' field.
    /// I suggest you also add a 'code' String field to hold the code you want to modify and run
    fn new_with_level(data: DataHolder, level: SupportLevel) -> Box<Self>;

    ///Return the (unique) name of your interpreter.
    fn get_name() -> String;

    /// The languages (as filetype codes) supported by your interpreter; check ':set ft?' in neovim
    /// on a file of your language if you are not sure
    fn get_supported_languages() -> Vec<String>;

    fn get_current_level(&self) -> SupportLevel;
    fn set_current_level(&mut self, level: SupportLevel);
    fn get_data(&self) -> DataHolder;

    /// You should override this method as soon as you wish to test your interpreter.
    fn get_max_support_level() -> SupportLevel {
        //to overwrite in trait impls
        return SupportLevel::Unsupported;
    }

    /// This function should be overwritten if your intepreter cannot run
    /// all the files for the advertised filetypes.
    /// It's up to you to detect it, and initialize (new()) and .run() it and return the result
    fn fallback(&mut self) -> Option<Result<String, SniprunError>> {
        // if incompatible code detected {
        //      let mut good_interpreter =
        //      crate::interpreters::Good_interpreter::new_with_level(&self.data,&self.get_current_level());
        //      return Some(good_interpreter.run());
        //      }
        None
    }

    /// This method should get the needed code from the data struct and eventually the files
    /// of the project
    fn fetch_code(&mut self) -> Result<(), SniprunError>; //mut to allow modification of the current_level

    /// This should add code that does not originate from the project to the code field in the
    /// interpreter
    fn add_boilerplate(&mut self) -> Result<(), SniprunError>;

    /// This should be used to build (compile) the code and produce an executable
    /// this function should be left blank (return Ok(());) for interpreted languages.
    fn build(&mut self) -> Result<(), SniprunError>; //return path to executable

    ///This should be used to execute a binary or execute the script
    ///In case it's successfull, returns Ok( standart_output );
    fn execute(&mut self) -> Result<String, SniprunError>;

    /// set the current support level to the one provided, run fetch(), add_boilerplate(), build() and execute() in order if each step is successfull
    fn run_at_level(&mut self, level: SupportLevel) -> Result<String, SniprunError> {
        self.set_current_level(level);
        if let Some(res) = self.fallback() {
            return res;
        }
        self.fetch_code()
            .and_then(|_| self.add_boilerplate())
            .and_then(|_| self.build())
            .and_then(|_| self.execute())
    }
    /// default run function ran from the launcher (run_at_level(max_level))
    fn run(&mut self) -> Result<String, SniprunError> {
        self.run_at_level(self.get_current_level())
    }

    /// return a Range object containing the start and end/ row and columns of the code that hould
    /// be included
    fn get_code_dependencies(&mut self) -> Option<Vec<Range>> {
        let initial_range = Range {
            start_row: self.get_data().range[0] as usize - 1,
            start_col: 0,
            end_row: self.get_data().range[1] as usize - 1,
            end_col: 0,
        };
        let vrange = vec![initial_range];

        fn walk_up_ranges(vrange: Vec<Range>, data: &DataHolder) -> Option<Vec<Range>> {
            let mut vec_range = vec![];
            for range in vrange.clone() {
                let nir = data
                    .nvim_instance
                    .clone()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .command_output(&format!(
                        "lua require'lua.nvim_treesitter_interface'.list_nodes_in_range({},{})",
                        range.start_row, range.end_row
                    )); //account for 0-based range rather than 1based line number
                if let Ok(nir_unwrapped) = nir {
                    for line in nir_unwrapped.lines() {
                        info!("lines -> {:?}", line);
                        let range: Vec<&str> = line.split(" ").collect();
                        info!("range -> {:?}", range);
                        vec_range.push(Range::from(range));
                    }
                } else {
                    return None;
                }
            }
            info!("initial range : {:?}", vrange);
            info!("next range : {:?}", vec_range.clone());
            vec_range.extend(vrange.clone());
            let future_range = squash_vec_range(vec_range);
            info!("next range confirmed : {:?}", future_range);
            if future_range == vrange {
                return Some(future_range);
            } else {
                info!("searching at superior level");
                return walk_up_ranges(future_range, data);
            }
        }

        if let Some(mut final_deps) = walk_up_ranges(vrange, &self.get_data()) {
            let index = final_deps.iter().position(|x| *x == initial_range).unwrap();
            final_deps.swap_remove(index);
            return Some(final_deps);
        } else {
            return None;
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Range {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

fn included(range: &Range, containing_range: &Range) -> bool {
    (range.start_row > containing_range.start_row
        || (range.start_row == containing_range.start_row
            && range.start_col >= containing_range.start_col))
        && (range.end_row < containing_range.end_row
            || (range.end_row == containing_range.end_row
                && range.end_col <= containing_range.end_col))
}

fn in_vec(range: &Range, v: &Vec<Range>) -> bool {
    for r in v {
        if included(range, r) {
            return true;
        }
    }
    return false;
}

fn squash_vec_range(v: Vec<Range>) -> Vec<Range> {
    let mut nv = Vec::new();

    for &range in &v {
        if !(in_vec(&range, &nv)) {
            nv.push(range);
        }
    }
    return nv;
}

impl From<Vec<&str>> for Range {
    fn from(v: Vec<&str>) -> Self {
        assert!(v.len() >= 4);
        Range {
            start_row: v[0].parse::<usize>().unwrap(),
            start_col: v[1].parse::<usize>().unwrap(),
            end_row: v[2].parse::<usize>().unwrap(),
            end_col: v[3].parse::<usize>().unwrap(),
        }
    }
}
