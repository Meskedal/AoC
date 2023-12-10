import os
import pcre


fn main() {
	if os.args.len < 2 {
		println("Usage: ${os.args[0]} <input.txt>")
		return
	}
	pattern := r'^\D*(\d)\D*.*(\d)\D*$'
	re := pcre.new_regex(pattern, 0)!
	lines := os.read_lines(os.args[1])!
	mut sum := 0
	for line in lines {
		println("Line: ${line}")
		m := re.match_str(line, 0, 0)!
		println(m.get(0)!)
		break
		num := line.int()
		sum += num
	}

	println("Sum: ${sum}")
	// file.
}