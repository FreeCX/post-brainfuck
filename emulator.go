package main

import (
	"flag"
	"fmt"
	"io"
	"os"
)

const MEM_MAX = 30_000

type Emulator struct {
	app   []byte
	mem   [MEM_MAX]byte
	jumps []uint

	ip uint
	mp uint

	input io.Reader
}

func pop(a []uint) (uint, []uint) {
	return a[len(a)-1], a[:len(a)-1]
}

func IsValidToken(t byte) bool {
	return (t == '>') || (t == '<') || (t == '+') || (t == '-') ||
		(t == '.') || (t == ',') || (t == '[') || (t == ']')
}

func FromFile(filename string) Emulator {
	data, err := os.ReadFile(filename)
	if err != nil {
		panic(err)
	}

	var e = Emulator{ip: 0, mp: 0, input: os.Stdin}

	for _, char := range data {
		if IsValidToken(char) {
			e.app = append(e.app, char)
		}
	}

	e.jumps = make([]uint, len(e.app))

	var stack []uint
	for index, char := range e.app {
		if char == '[' {
			stack = append(stack, uint(index))
		}
		if char == ']' {
			var last uint
			last, stack = pop(stack)
			e.jumps[last] = uint(index)
			e.jumps[index] = last
		}
	}

	return e
}

func (e *Emulator) step() {
	switch command := e.app[e.ip]; command {
	case '>':
		if e.mp > MEM_MAX {
			errmsg := fmt.Sprintf("(op >) memory pointer must be <= %d", MEM_MAX)
			panic(errmsg)
		}
		e.mp += 1
	case '<':
		if e.mp == 0 {
			panic("(op <) memory pointer must be >= 0")
		}
		e.mp -= 1
	case '+':
		e.mem[e.mp] += 1
	case '-':
		e.mem[e.mp] -= 1
	case '.':
		fmt.Printf("%c", e.mem[e.mp])
	case ',':
		char := make([]byte, 1)
		_, err := e.input.Read(char)
		if err != nil {
			panic(err)
		}
		e.mem[e.mp] = char[0]
	case '[':
		if e.mem[e.mp] == 0 {
			e.ip = e.jumps[e.ip]
		}
	case ']':
		if e.mem[e.mp] != 0 {
			e.ip = e.jumps[e.ip]
		}
	default:
	}
	e.ip += 1
}

func (e Emulator) execute() {
	for e.ip < uint(len(e.app)) {
		e.step()
	}
}

func main() {
	var input = flag.String("i", "", "input file")
	flag.Parse()

	if len(*input) == 0 {
		fmt.Println("usage: emulator -i <filename>")
		return
	}

	var emulator = FromFile(*input)
	emulator.execute()
}
