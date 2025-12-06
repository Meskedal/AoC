import sys, os, requests

session_cookie = "53616c7465645f5fdc268d26be8ab10971d8cdc4a8dcaf1feb447b30c38e236866d42ac7bdb6e168e52ecd1d313c6ecc5f11ff17bbc58f06ec7061fdc584cade"
def fetch_full_input(year: int, day: int) -> str:
    url = f'https://adventofcode.com/{year}/day/{day}/input'
    cookies = {'session': session_cookie}
    response = requests.get(url, cookies=cookies, verify=False)
    if response.status_code != 200:
        print(f"Failed to fetch input for day {day}")
        return None
    return response.text[:-1] # Cut off the last newline character

def generate_benches(day: int) -> None: 
    path = f"day{day}"
    bench_folder = "benches"
    bench_file = "bench.rs"
    os.makedirs(os.path.join(path, bench_folder))
    with open(os.path.join(path, bench_folder, bench_file), "w") as f:
        f.write(f"""use criterion::{{black_box, criterion_group, criterion_main, Criterion}};
fn benchmark_solve(c: &mut Criterion) {{
\tc.bench_function("Day {day} Input Generator", |b| b.iter(|| day{day}::input_generator(black_box(include_str!("../input/full.txt")))));
\tlet input = day{day}::input_generator(include_str!("../input/full.txt"));  
\tc.bench_function("Day {day} Part 1", |b| b.iter(|| day{day}::part_1(black_box(&input))));
\tc.bench_function("Day {day} Part 2", |b| b.iter(|| day{day}::part_2(black_box(&input))));
}}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);
""")

def generate_input(year: int, day: int) -> None:
    input_folder = f"day{day}/input"
    os.makedirs(input_folder)
    with open(f"{input_folder}/full.txt", "w") as f:
        f.write(fetch_full_input(year, day))
    with open(f"{input_folder}/example.txt", "w") as f:
        pass # Do not populate anything at this time 

def generate_lib(day: int) -> None:
    lib_folder = f"day{day}/src"
    lib_file = "lib.rs"
    os.makedirs(lib_folder, exist_ok=True)
    with open(os.path.join(lib_folder, lib_file), "w") as f:
        f.write(f"""pub fn input_generator(input: &str) -> String {{
\tinput.to_string()
}}

pub fn part_1(input: &str) -> i32 {{
\t0
}}

pub fn part_2(input: &str) -> i32 {{
\t0
}}
""")

def generate_main(day: int) -> None:
    main_folder = f"day{day}/src"
    main_file = "main.rs"
    os.makedirs(main_folder, exist_ok=True)
    with open(os.path.join(main_folder, main_file), "w") as f:
        f.write(f"""fn main() {{
\tlet input  = day{day}::input_generator(include_str!("../input/full.txt"));
\tlet part_1 = day{day}::part_1(&input);
\tlet part_2 = day{day}::part_2(&input);
\tprintln!("Part 1: {{}}", part_1);
\tprintln!("Part 2: {{}}", part_2); 
}}
""")


def generate_tests(day: int) -> None:
    test_folder = f"day{day}/tests"
    test_file = "test.rs"
    os.makedirs(test_folder)
    with open(os.path.join(test_folder, test_file), "w") as f:
        f.write(f"""#[test]
fn test_day_{day}_part_1_example() {{
\tlet input = day{day}::input_generator(include_str!("../input/example.txt"));
\tlet expected = 480;
\tlet result = day{day}::part_1(&input);
\tassert_eq!(result, expected);
}}

#[test]
fn test_day_{day}_part_2_example() {{
\tlet input = day{day}::input_generator(include_str!("../input/example.txt"));
\tlet expected = 0;
\tlet result = day{day}::part_2(&input);
\tassert_eq!(result, expected);
}}
""")

def generate_cargo(day: int) -> None:
    cargo_folder = f"day{day}"
    cargo_file = "Cargo.toml"
    os.makedirs(cargo_folder, exist_ok=True)
    with open(os.path.join(cargo_folder, cargo_file), "w") as f:
        f.write(f"""[package]
name = "day{day}"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
criterion = "0.4"

[[bench]]
name = "bench"
harness = false
""")

def edit_workspace_cargo(day: int) -> None:
    cargo_file = "Cargo.toml"
    # Read the file content
    with open(cargo_file, 'r') as file:
        lines = file.readlines()

    # Find the index of the members list
    members_start_index = None
    members_end_index = None
    for i, line in enumerate(lines):
        if line.strip() == "members = [":
            members_start_index = i
        elif line.strip() == "]" and members_start_index is not None:
            members_end_index = i
            break

    # If members list is found, check if the day is already in the list
    if members_start_index is not None and members_end_index is not None:
        day_str = f'    "day{day}",\n'
        if day_str not in lines[members_start_index:members_end_index]:
            lines.insert(members_end_index, day_str)

    # Write the updated content back to the file
    with open(cargo_file, 'w') as file:
        file.writelines(lines)

        

if __name__ == "__main__":
    # Gen sys arguments 
    args = sys.argv
    # Check if the user has passed the correct number of arguments
    if len(args) != 2:
        print("Usage: python day_generator.py <day>")
        sys.exit(1)
    # Get the template name
    day = args[1]

    if not day.isdigit():
        print("Template name must be a number")
        sys.exit(1)

    os.makedirs(f"day{day}", exist_ok=True)
    generate_cargo(day)
    generate_lib(day)
    generate_main(day)
    generate_input(2025, day)
    generate_benches(day)
    generate_tests(day)
    edit_workspace_cargo(day)