from typing import List, Dict
from dataclasses import dataclass


class KCLObject:
    pass


@dataclass
class KCLIntObject(KCLObject):
    value: int


@dataclass
class KCLFloatObject(KCLObject):
    value: float


@dataclass
class KCLListObject(KCLObject):
    value: List[KCLObject]


@dataclass
class KCLIterObject(KCLObject):
    i: int
    value: KCLObject

    def next(self):
        list_obj = self.value
        ro = None
        if self.i < len(list_obj.value):
            ro = list_obj.value[self.i]
        self.i = self.i + 1
        return ro


BinaryAdd = 1
StoreGlobal = 2
StoreLocal = 3
LoadConst = 4
LoadLocal = 5
BuildList = 6
ListAppend = 7
CallFunction = 8
GetIter = 9
PopTop = 10
JmpAbs = 11
ForIter = 12


@dataclass
class Instruction:
    filename: str
    line:     int
    column:   int
    opcode:   int
    operand1: int
    operand2: int
    operand3: int


@dataclass
class ByteCode:
    instructions: List[Instruction]
    names:        List[str]
    constants:    List[KCLObject]


@dataclass
class VM:
    stack:   List[KCLObject]
    globals: Dict[str, KCLObject]
    locals:  Dict[str, KCLObject]

    def push(self, obj: KCLObject):
        self.stack.append(obj)

    def pop(self) -> KCLObject:
        last = self.stack[len(self.stack)-1]
        self.stack = self.stack[:len(self.stack)-1]
        return last

    def peek_nth(self, i: int) -> KCLObject:
        return self.stack[len(self.stack)-i]

    def peek(self) -> KCLObject:
        return self.stack[len(self.stack)-1]

    def run(self, bytecode: ByteCode):
        i = 0
        while i < len(bytecode.instructions):
            elem = bytecode.instructions[i]
            if elem.opcode == BinaryAdd:
                pass
            elif elem.opcode == StoreGlobal:
                # print("StoreGlobal")
                name = bytecode.names[elem.operand3]
                self.globals[name] = self.pop()
            elif elem.opcode == StoreLocal:
                # print("StoreLocal")
                name = bytecode.names[elem.operand3]
                self.locals[name] = self.peek()
            elif elem.opcode == LoadConst:
                # print("LoadConst")
                self.push(bytecode.constants[elem.operand3])
            elif elem.opcode == LoadLocal:
                # print("LoadLocal")
                name = bytecode.names[elem.operand3]
                self.push(self.locals[name])
            elif elem.opcode == BuildList:
                # print("BuildList")
                self.push(KCLListObject(value=[]))
            elif elem.opcode == ListAppend:
                # print("ListAppend", elem.operand3, self.stack)
                item = self.pop()
                list_obj = self.peek_nth(elem.operand3)
                list_obj.value.append(item)
            elif elem.opcode == CallFunction:
                # print("CallFunction")
                list_obj = KCLListObject(
                    value=list(range(20000)),
                )
                self.push(list_obj)
            elif elem.opcode == GetIter:
                # print("GetIter")
                obj = self.pop()
                self.push(KCLIterObject(
                    i=0,
                    value=obj,
                ))
            elif elem.opcode == PopTop:
                # print("PopTop")
                self.pop()
            elif elem.opcode == JmpAbs:
                # print("JmpAbs")
                i = elem.operand3
            elif elem.opcode == ForIter:
                # print("ForIter")
                obj = self.peek()
                obj_next = obj.next()
                if obj_next is not None:
                    self.push(obj_next)
                else:
                    self.pop()
                    i = elem.operand3
            i = i + 1


if __name__ == "__main__":
    vm = VM(
        stack=[],
        globals={},
        locals={},
    )
    bytecode = ByteCode(
        constants=[KCLIntObject(0), KCLIntObject(
            0), KCLIntObject(1), KCLIntObject(20000)],
        names=[
            "_a",
            "_b",
            "_a",
            "i",
            "_b",
        ],
        instructions=[
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=LoadConst,
                operand1=0,
                operand2=0,
                operand3=0,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=StoreGlobal,
                operand1=0,
                operand2=0,
                operand3=0,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=LoadConst,
                operand1=0,
                operand2=0,
                operand3=1,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=StoreGlobal,
                operand1=0,
                operand2=0,
                operand3=1,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=LoadConst,
                operand1=0,
                operand2=0,
                operand3=2,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=StoreGlobal,
                operand1=0,
                operand2=0,
                operand3=2,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=BuildList,
                operand1=0,
                operand2=0,
                operand3=0,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=CallFunction,
                operand1=0,
                operand2=0,
                operand3=1,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=GetIter,
                operand1=0,
                operand2=0,
                operand3=1,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=ForIter,
                operand1=0,
                operand2=0,
                operand3=14,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=StoreLocal,
                operand1=0,
                operand2=0,
                operand3=3,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=PopTop,
                operand1=0,
                operand2=0,
                operand3=1,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=LoadLocal,
                operand1=0,
                operand2=0,
                operand3=3,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=ListAppend,
                operand1=0,
                operand2=0,
                operand3=2,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=JmpAbs,
                operand1=0,
                operand2=0,
                operand3=8,
            ),
            Instruction(
                filename="_a",
                line=1,
                column=1,
                opcode=StoreGlobal,
                operand1=0,
                operand2=0,
                operand3=4,
            ),
        ],
    )
    vm.run(bytecode)
    print(len(vm.globals["_b"].value))
    print("Hello KCLVM")
