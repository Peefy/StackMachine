use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
type KCLObjectRef = Rc<dyn KCLObject>;
pub trait KCLObject: Any {
    fn as_list(&self) -> Option<&KCLListObject>;
    fn as_int(&self) -> Option<&KCLIntObject>;
    fn as_iter(&self) -> Option<&KCLIterObject>;
}
pub struct KCLIntObject {
    value: i128,
}
pub struct KCLFloatObject {
    value: f64,
}
pub struct KCLListObject {
    value: Vec<KCLObjectRef>,
}
pub struct KCLIterObject {
    i: usize,
    value: KCLObjectRef,
}
impl KCLObject for KCLIntObject {
    fn as_int(&self) -> Option<&KCLIntObject> {
        Some(self)
    }
    fn as_list(&self) -> Option<&KCLListObject> {
        None
    }
    fn as_iter(&self) -> Option<&KCLIterObject> {
        None
    }
}
impl KCLObject for KCLFloatObject {
    fn as_int(&self) -> Option<&KCLIntObject> {
        None
    }
    fn as_list(&self) -> Option<&KCLListObject> {
        None
    }
    fn as_iter(&self) -> Option<&KCLIterObject> {
        None
    }
}
impl KCLObject for KCLListObject {
    fn as_int(&self) -> Option<&KCLIntObject> {
        None
    }
    fn as_list(&self) -> Option<&KCLListObject> {
        Some(self)
    }
    fn as_iter(&self) -> Option<&KCLIterObject> {
        None
    }
}
impl KCLObject for KCLIterObject {
    fn as_int(&self) -> Option<&KCLIntObject> {
        None
    }
    fn as_list(&self) -> Option<&KCLListObject> {
        None
    }
    fn as_iter(&self) -> Option<&KCLIterObject> {
        Some(self)
    }
}
impl KCLIterObject {
    pub fn next(&self, index: usize) -> Option<&KCLObjectRef> {
        match self.value.as_list() {
            Some(list_obj) => {
                if index < list_obj.value.len() {
                    Some(&list_obj.value[index])
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
impl fmt::Display for KCLIntObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KCLObject({})", self.value)
    }
}
impl fmt::Display for KCLFloatObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KCLObject({})", self.value)
    }
}
pub enum Opcode {
    BinaryAdd,
    StoreGlobal,
    StoreLocal,
    LoadConst,
    LoadLocal,
    BuildList,
    ListAppend,
    CallFunction,
    GetIter,
    PopTop,
    JmpAbs,
    ForIter,
}
pub struct Instruction {
    filename: String,
    line: u64,
    column: u64,
    opcode: Opcode,
    operand1: u8,
    operand2: u8,
    operand3: u8,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode({})", self.operand1)
    }
}
pub struct ByteCode {
    instructions: Vec<Instruction>,
    names: Vec<String>,
    constants: Vec<KCLObjectRef>,
}
fn main() {
    println!("Hello, KCLVM Rust!");
    let mut vm = VM {};
    let bytecode = ByteCode {
        constants: vec![
            Rc::from(KCLIntObject { value: 0 }),
            Rc::from(KCLIntObject { value: 0 }),
            Rc::from(KCLIntObject { value: 1 }),
            Rc::from(KCLIntObject { value: 200 }),
        ],
        names: vec![
            String::from("_a"),
            String::from("_b"),
            String::from("_a"),
            String::from("i"),
            String::from("_b"),
        ],
        instructions: vec![
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::LoadConst,
                operand1: 0,
                operand2: 0,
                operand3: 0,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::StoreGlobal,
                operand1: 0,
                operand2: 0,
                operand3: 0,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::LoadConst,
                operand1: 0,
                operand2: 0,
                operand3: 1,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::StoreGlobal,
                operand1: 0,
                operand2: 0,
                operand3: 1,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::LoadConst,
                operand1: 0,
                operand2: 0,
                operand3: 2,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::StoreGlobal,
                operand1: 0,
                operand2: 0,
                operand3: 2,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::BuildList,
                operand1: 0,
                operand2: 0,
                operand3: 0,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::CallFunction,
                operand1: 0,
                operand2: 0,
                operand3: 1,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::GetIter,
                operand1: 0,
                operand2: 0,
                operand3: 1,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::ForIter,
                operand1: 0,
                operand2: 0,
                operand3: 14,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::StoreLocal,
                operand1: 0,
                operand2: 0,
                operand3: 3,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::PopTop,
                operand1: 0,
                operand2: 0,
                operand3: 1,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::LoadLocal,
                operand1: 0,
                operand2: 0,
                operand3: 3,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::ListAppend,
                operand1: 0,
                operand2: 0,
                operand3: 2,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::JmpAbs,
                operand1: 0,
                operand2: 0,
                operand3: 8,
            },
            Instruction {
                filename: String::from("_a"),
                line: 1,
                column: 1,
                opcode: Opcode::StoreGlobal,
                operand1: 0,
                operand2: 0,
                operand3: 4,
            },
        ],
    };
    vm.run(&bytecode);
}
struct VM {}
impl VM {
    pub fn run(self, bytecode: &ByteCode) {
        println!("Run KCLVM Rust!");
        let mut stack: Vec<KCLObjectRef> = vec![];
        let mut globals: HashMap<String, KCLObjectRef> = HashMap::new();
        let mut locals: HashMap<String, KCLObjectRef> = HashMap::new();
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < bytecode.instructions.len() {
            let elem = &bytecode.instructions[i];
            match elem.opcode {
                Opcode::BinaryAdd => {
                    println!("BinaryAdd");
                }
                Opcode::StoreGlobal => {
                    // println!("StoreGlobal");
                    let index = elem.operand3 as usize;
                    let name = bytecode.names[index].clone();
                    let obj = stack.pop().expect("empty stack");
                    globals.insert(name, obj);
                }
                Opcode::StoreLocal => {
                    // println!("StoreLocal");
                    let index = elem.operand3 as usize;
                    let name = bytecode.names[index].clone();
                    let obj = stack.last().expect("last object not found");
                    locals.insert(name, (*obj).clone());
                }
                Opcode::LoadConst => {
                    let index = elem.operand3 as usize;
                    stack.push(bytecode.constants[index].clone());
                    // println!("LoadConst");
                }
                Opcode::LoadLocal => {
                    let index = elem.operand3 as usize;
                    let name = &bytecode.names[index];
                    let item = locals.get(name).expect("not found");
                    stack.push((*item).clone());
                    // println!("LoadLocal");
                }
                Opcode::BuildList => {
                    stack.push(Rc::from(KCLListObject { value: vec![] }));
                    // println!("BuildList");
                }
                Opcode::ListAppend => {
                    let item = stack.pop().expect("empty stack");
                    let index = elem.operand3 as usize;
                    let obj = stack[stack.len() - index].clone();
                    match obj.as_list() {
                        Some(list_obj) => {
                            //list_obj.value.push(item)
                        }
                        None => println!("ListAppend error"),
                    };
                }
                Opcode::CallFunction => {
                    let list_obj = Rc::from(KCLListObject {
                        value: vec![Rc::from(KCLIntObject { value: 0 }); 20000],
                    });
                    stack.push(list_obj);
                    // println!("CallFunction");
                }
                Opcode::GetIter => {
                    let obj = stack.pop().expect("empty stack");
                    stack.push(Rc::from(KCLIterObject { i: 0, value: obj }));
                    // println!("GetIter");
                }
                Opcode::PopTop => {
                    stack.pop().expect("empty stack");
                }
                Opcode::ForIter => {
                    match stack.last().expect("last not found").as_iter() {
                        Some(iter_obj) => match iter_obj.next(j) {
                            Some(o) => {
                                j += 1;
                                stack.push((*o).clone())
                            }
                            None => i = elem.operand3 as usize,
                        },
                        None => {}
                    }
                    // println!("ForIter")
                }
                Opcode::JmpAbs => {
                    i = elem.operand3 as usize;
                }
            }
            i += 1;
        }
    }
}
