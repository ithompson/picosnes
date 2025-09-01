# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "pydantic-yaml",
# ]
# ///

import argparse
from collections import defaultdict
from pathlib import Path
from pydantic import BaseModel
from pydantic_yaml import parse_yaml_file_as

from typing import Dict, List, Tuple


class AccessModeData(BaseModel):
    inst_arg: str
    subtypes: Dict[str, List[Tuple[str, str]]]

class MnemonicData(BaseModel):
    description: str
    access_subtype: str
    op: str | None = None
    flags: str | None = None

class InstructionData(BaseModel):
    mnemonic: str
    access_mode: str

class CpuData(BaseModel):
    access_modes: Dict[str, AccessModeData]
    mnemonics: Dict[str, MnemonicData]
    instructions: Dict[int, InstructionData]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("cpu_data", type=Path, help="CPU data file")

    args = parser.parse_args()

    cpu_data: CpuData = parse_yaml_file_as(CpuData, args.cpu_data)

    for opcode, data in cpu_data.instructions.items():
        mnemonic = cpu_data.mnemonics[data.mnemonic]
        access_mode = cpu_data.access_modes[data.access_mode]

        if mnemonic.op is None:
            op = "nop"
        else:
            op = data.mnemonic.lower()
        
        assert mnemonic.access_subtype in access_mode.subtypes, \
            f"Access subtype {mnemonic.access_subtype} not found in access mode {data.access_mode}"

        if access_mode.inst_arg:
            instr_name = f"{data.mnemonic.upper()} {access_mode.inst_arg}"
        else:
            instr_name = f"{data.mnemonic.upper()}"
        sequence_name = f"{data.access_mode.upper()}_{mnemonic.access_subtype.upper()}_SEQUENCE"

        print(f"    opcode!(ops, 0x{opcode:02X}, \"{instr_name}\", {sequence_name}, {op});")

if __name__ == "__main__":
    main()
