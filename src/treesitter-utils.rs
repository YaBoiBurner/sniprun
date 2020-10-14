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
