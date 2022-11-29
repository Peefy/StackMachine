package main

import "fmt"

type KCLObject interface {
	String() string
}
type KCLIntObject struct {
	value int
}
type KCLFloatObject struct {
	value float64
}
type KCLListObject struct {
	value []KCLObject
}
type KCLIterObject struct {
	i     int
	value KCLObject
}

func (obj *KCLIterObject) next() KCLObject {
	list_obj := obj.value.(*KCLListObject)
	var ro KCLObject = nil
	if obj.i < len(list_obj.value) {
		ro = list_obj.value[obj.i]
	}
	obj.i = obj.i + 1
	return ro
}
func (obj *KCLIntObject) String() string {
	return "KCLIntObject"
}
func (obj *KCLFloatObject) String() string {
	return "KCLFloatObject"
}
func (obj *KCLListObject) String() string {
	return "KCLListObject"
}
func (obj *KCLIterObject) String() string {
	return "KCLIterObject"
}

type Opcode int

const (
	BinaryAdd Opcode = iota
	StoreGlobal
	StoreLocal
	LoadConst
	LoadLocal
	BuildList
	ListAppend
	CallFunction
	GetIter
	PopTop
	JmpAbs
	ForIter
)

type Instruction struct {
	filename string
	line     int
	column   int
	opcode   Opcode
	operand1 int
	operand2 int
	operand3 int
}
type ByteCode struct {
	instructions []Instruction
	names        []string
	constants    []KCLObject
}
type VM struct {
	stack   []KCLObject
	globals map[string]KCLObject
	locals  map[string]KCLObject
}

func (vm *VM) push(obj KCLObject) {
	vm.stack = append(vm.stack, obj)
}
func (vm *VM) pop() KCLObject {
	last := vm.stack[len(vm.stack)-1]
	vm.stack = vm.stack[:len(vm.stack)-1]
	return last
}
func (vm *VM) peek_nth(i int) KCLObject {
	return vm.stack[len(vm.stack)-i]
}
func (vm *VM) peek() KCLObject {
	return vm.stack[len(vm.stack)-1]
}
func (vm *VM) run(bytecode *ByteCode) {
	i := 0
	for i < len(bytecode.instructions) {
		elem := bytecode.instructions[i]
		switch elem.opcode {
		case BinaryAdd:
			{
			}
		case StoreGlobal:
			{
				// fmt.Println("StoreGlobal")
				name := bytecode.names[elem.operand3]
				vm.globals[name] = vm.pop()
			}
		case StoreLocal:
			{
				// fmt.Println("StoreLocal")
				name := bytecode.names[elem.operand3]
				vm.locals[name] = vm.peek()
			}
		case LoadConst:
			{
				// fmt.Println("LoadConst")
				vm.push(bytecode.constants[elem.operand3])
			}
		case LoadLocal:
			{
				// fmt.Println("LoadLocal")
				name := bytecode.names[elem.operand3]
				vm.push(vm.locals[name])
			}
		case BuildList:
			{
				// fmt.Println("BuildList")
				vm.push(&KCLListObject{})
			}
		case ListAppend:
			{
				// fmt.Println("ListAppend", elem.operand3, vm.stack)
				item := vm.pop()
				list_obj := vm.peek_nth(elem.operand3).(*KCLListObject)
				list_obj.value = append(list_obj.value, item)
			}
		case CallFunction:
			{
				// fmt.Println("CallFunction")
				list_obj := &KCLListObject{
					value: make([]KCLObject, 20000),
				}
				for i, _ := range list_obj.value {
					list_obj.value[i] = &KCLIntObject{
						value: i,
					}
				}
				vm.push(list_obj)
				// fmt.Println(vm.stack)
			}
		case GetIter:
			{
				// fmt.Println("GetIter")
				obj := vm.pop()
				vm.push(&KCLIterObject{
					value: obj,
				})
			}
		case PopTop:
			{
				// fmt.Println("PopTop")
				vm.pop()
			}
		case JmpAbs:
			{
				// fmt.Println("JmpAbs")
				i = elem.operand3
			}
		case ForIter:
			{
				// fmt.Println("ForIter")
				obj := vm.peek().(*KCLIterObject)
				obj_next := obj.next()
				if obj_next != nil {
					vm.push(obj_next)
				} else {
					vm.pop()
					i = elem.operand3
				}
			}
		default:
			return
		}
		i = i + 1
	}
}
func main() {
	var vm = VM{
		globals: map[string]KCLObject{},
		locals:  map[string]KCLObject{},
	}
	var bytecode = ByteCode{
		constants: []KCLObject{&KCLIntObject{}, &KCLIntObject{}, &KCLIntObject{1}, &KCLIntObject{200000}},
		names: []string{
			"_a",
			"_b",
			"_a",
			"i",
			"_b",
		},
		instructions: []Instruction{
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   LoadConst,
				operand1: 0,
				operand2: 0,
				operand3: 0,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   StoreGlobal,
				operand1: 0,
				operand2: 0,
				operand3: 0,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   LoadConst,
				operand1: 0,
				operand2: 0,
				operand3: 1,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   StoreGlobal,
				operand1: 0,
				operand2: 0,
				operand3: 1,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   LoadConst,
				operand1: 0,
				operand2: 0,
				operand3: 2,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   StoreGlobal,
				operand1: 0,
				operand2: 0,
				operand3: 2,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   BuildList,
				operand1: 0,
				operand2: 0,
				operand3: 0,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   CallFunction,
				operand1: 0,
				operand2: 0,
				operand3: 1,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   GetIter,
				operand1: 0,
				operand2: 0,
				operand3: 1,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   ForIter,
				operand1: 0,
				operand2: 0,
				operand3: 14,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   StoreLocal,
				operand1: 0,
				operand2: 0,
				operand3: 3,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   PopTop,
				operand1: 0,
				operand2: 0,
				operand3: 1,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   LoadLocal,
				operand1: 0,
				operand2: 0,
				operand3: 3,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   ListAppend,
				operand1: 0,
				operand2: 0,
				operand3: 2,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   JmpAbs,
				operand1: 0,
				operand2: 0,
				operand3: 8,
			},
			{
				filename: "_a",
				line:     1,
				column:   1,
				opcode:   StoreGlobal,
				operand1: 0,
				operand2: 0,
				operand3: 4,
			},
		},
	}
	vm.run(&bytecode)
	fmt.Println(len(vm.globals["_b"].(*KCLListObject).value))
	fmt.Println("Hello KCLVM")
}
