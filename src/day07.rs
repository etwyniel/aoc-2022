use aoc_framework::{anyhow::anyhow, *};

pub struct Day7;

impl_day!(Day7::{Part1, Part2}: 2022[7], r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
");

fn build_fs_map(input: impl Iterator<Item = String>) -> anyhow::Result<Vec<u64>> {
    // current "path"
    let mut stack = Vec::new();
    // sizes of processed directories
    let mut directories = Vec::new();
    for line in input {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            if dir == ".." {
                // pop current directory, add its size to parent
                let size = stack.pop().unwrap();
                if let Some(prev) = stack.last_mut() {
                    *prev += size;
                }
                // push it to the result
                directories.push(size);
            } else {
                stack.push(0);
            }
        } else if line == "$ ls" || line.starts_with("dir") {
            // no-op
        } else {
            // file entry, parse its size
            let size: u64 = line
                .split_once(' ')
                .ok_or_else(|| anyhow!("Invalid input"))?
                .0
                .parse()?;
            // add size to size of current directory
            *stack.last_mut().unwrap() += size;
        }
    }
    // pop remaining directories, add their size to their parent each time
    while let Some(val) = stack.pop() {
        if let Some(prev) = stack.last_mut() {
            *prev += val;
        }
        directories.push(val);
    }
    Ok(directories)
}

pub struct Part1;

impl Part for Part1 {
    type D = Day7;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(95437));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let fs = build_fs_map(input)?;
        Ok(Num(fs.into_iter().filter(|size| *size < 100_000).sum()))
    }
}

pub struct Part2;

impl Part for Part2 {
    type D = Day7;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(24933642));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let fs = build_fs_map(input)?;
        let total_used = fs.last().unwrap();
        let required = 30_000_000 - (70_000_000 - total_used);
        Ok(Num(fs
            .into_iter()
            .filter(|dir| *dir > required)
            .min()
            .unwrap()))
    }
}
