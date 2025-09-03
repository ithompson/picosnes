# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "jinja2",
#     "pydantic-yaml",
# ]
# ///

import argparse
from pathlib import Path
from pydantic_yaml import parse_yaml_file_as
import jinja2

from utils.cpu_data import *

SCRIPT_DIR = Path(__file__).resolve().parent

JINJA_ENV = jinja2.Environment(
    loader=jinja2.FileSystemLoader(SCRIPT_DIR / "templates"),
    trim_blocks=True,
    lstrip_blocks=True,
)

RUST_MEM_CYCLE_NAMES = {
    'inc_read_pc': 'IncReadPC',
    'read_pc': 'ReadPC',
    'inc_read_tmp': 'IncReadTmp',
    'read_tmp': 'ReadTmp',
    'inc_write_tmp': 'IncWriteTmp',
    'write_tmp': 'WriteTmp',
    'inc_read_stk': 'IncReadStk',
    'read_stk': 'ReadStk',
    'inc_push_stk': 'IncPushStk',
    'push_stk': 'PushStk',
    'pop_stk': 'PopStk',
}
def filter_rust_mem_cycle(s: str) -> str:
    return RUST_MEM_CYCLE_NAMES[s]
JINJA_ENV.filters['rust_mem_cycle'] = filter_rust_mem_cycle

def generate_rust_code(cpu_data: CpuData, args):
    sequences = {
        'RESET': cpu_data.reset_seq,
        'IRQ': cpu_data.irq_seq,
        'NMI': cpu_data.nmi_seq,
        'DISPATCH': cpu_data.dispatch_seq,
    }
    for access_mode, access_mode_data in cpu_data.access_modes.items():
        for sequence, cycle_list in access_mode_data.sequences.items():
            sequences[f'{access_mode.upper()}_{sequence.upper()}'] = cycle_list

    opcode_table_template = JINJA_ENV.get_template("opcode_table.rs.jinja2")
    sequence_tables_template = JINJA_ENV.get_template("sequence_tables.rs.jinja2")

    opcode_table_content = opcode_table_template.render(cpu_data=cpu_data)
    sequence_table_content = sequence_tables_template.render(cpu_data=cpu_data, sequences=sequences)

    args.opcode_output.parent.mkdir(parents=True, exist_ok=True)
    with args.opcode_output.open('w') as f:
        f.write(opcode_table_content)

    args.sequence_output.parent.mkdir(parents=True, exist_ok=True)
    with args.sequence_output.open('w') as f:
        f.write(sequence_table_content)

def generate_rust_op_skeleton(cpu_data: CpuData, args):
    op_skeleton_template = JINJA_ENV.get_template("op_skeleton.rs.jinja2")
    op_skeleton_content = op_skeleton_template.render(cpu_data=cpu_data)

    print(op_skeleton_content)


def generate_rust_action_skeleton(cpu_data: CpuData, args):
    action_skeleton_template = JINJA_ENV.get_template("action_skeleton.rs.jinja2")
    action_skeleton_content = action_skeleton_template.render(cpu_data=cpu_data)

    print(action_skeleton_content)

def main() -> None:
    parser = argparse.ArgumentParser(description="Generator for files derived from CPU data")
    parser.add_argument('--data_file', type=Path, default=SCRIPT_DIR.parent / 'data' / '6502.yaml', help='Path to CPU data file')

    subparsers = parser.add_subparsers(dest='command', required=True)

    parser_rust_codegen = subparsers.add_parser('rust_codegen', help='Generate Rust code from CPU data')
    parser_rust_codegen.add_argument('--opcode_output', type=Path, default=SCRIPT_DIR.parent / "host/nes_emu/src/components/cpu/opcodes/opcode_table.rs", help="Path to the generated opcode table file")
    parser_rust_codegen.add_argument('--sequence_output', type=Path, default=SCRIPT_DIR.parent / "host/nes_emu/src/components/cpu/sequences/sequence_tables.rs", help="Path to the generated CPU cycle sequence file")

    _parser_rust_gen_op_skeleton = subparsers.add_parser('rust_gen_op_skeleton', help='Generate skeleton Rust code for instruction data ops')
    _parser_rust_gen_action_skeleton = subparsers.add_parser('rust_gen_action_skeleton', help='Generate skeleton Rust code for CPU cycle actions')

    args = parser.parse_args()

    cpu_data: CpuData = parse_yaml_file_as(CpuData, args.data_file)
    cpu_data.validate()

    match args.command:
        case "rust_codegen":
            generate_rust_code(cpu_data, args)
        case "rust_gen_op_skeleton":
            generate_rust_op_skeleton(cpu_data, args)
        case "rust_gen_action_skeleton":
            generate_rust_action_skeleton(cpu_data, args)

if __name__ == "__main__":
    main()
