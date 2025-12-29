import argparse
from collections import defaultdict
from pathlib import Path
import re
from typing import Dict

from ast_grep_py import SgRoot
from ast_grep_py import Config as SgConfig
from pydantic_yaml import parse_yaml_file_as
import jinja2

from .utils.cpu_data import CpuData

SCRIPT_DIR = Path(__file__).resolve().parent

JINJA_ENV = jinja2.Environment(
    loader=jinja2.FileSystemLoader(SCRIPT_DIR / "templates"),
    trim_blocks=True,
    lstrip_blocks=True,
)

RUST_MEM_CYCLE_NAMES = {
    "inc_read_pc": "IncReadPC",
    "read_pc": "ReadPC",
    "inc_read_tmp": "IncReadTmp",
    "read_tmp": "ReadTmp",
    "inc_write_tmp": "IncWriteTmp",
    "write_tmp": "WriteTmp",
    "inc_read_stk": "IncReadStk",
    "read_stk": "ReadStk",
    "inc_push_stk": "IncPushStk",
    "push_stk": "PushStk",
    "pop_stk": "PopStk",
}


def filter_rust_mem_cycle(s: str) -> str:
    return RUST_MEM_CYCLE_NAMES[s]


JINJA_ENV.filters["rust_mem_cycle"] = filter_rust_mem_cycle  # type: ignore


def get_keyed_comments_for_ops(cpu_data: CpuData) -> Dict[str, Dict[str, str]]:
    """
    Gets a table of comments that should be included on op function implementations
    describing the architectural description of that op
    """
    items: Dict[str, Dict[str, str]] = {}
    for op_name, op_data in cpu_data.mnemonics.items():
        op_items: Dict[str, str] = {}
        if op_data.op is not None:
            op_items["pseudocode"] = op_data.op
        elif op_data.branch_cond is not None:
            op_items["pseudocode"] = f"{{reg}} = {op_data.branch_cond}"
        if op_data.flags is not None:
            op_items["flags"] = op_data.flags

        if op_items:
            items[op_name] = op_items
    return items


def get_keyed_comments_for_actions(cpu_data: CpuData) -> Dict[str, Dict[str, str]]:
    """
    Gets a table of comments that should be included on action function implementations
    describing the architectural description of that action
    """
    return {
        action_name: {"pseudocode": pseudocode}
        for action_name, pseudocode in cpu_data.actions.items()
    }


def generate_rust_code(cpu_data: CpuData, args: argparse.Namespace):
    """Emit the auto-generated rust files describing the CPU micro-architecture"""

    # Build a table of all sequences that need to be implemented
    sequences = {
        "RESET": cpu_data.reset_seq,
        "IRQ": cpu_data.irq_seq,
        "NMI": cpu_data.nmi_seq,
        "DISPATCH": cpu_data.dispatch_seq,
    }
    for access_mode, access_mode_data in cpu_data.access_modes.items():
        for sequence, cycle_list in access_mode_data.sequences.items():
            sequences[f"{access_mode.upper()}_{sequence.upper()}"] = cycle_list

    # Generate and emit files using the templates
    opcode_table_template = JINJA_ENV.get_template("opcode_table.rs.jinja2")
    sequence_tables_template = JINJA_ENV.get_template("sequence_tables.rs.jinja2")

    opcode_table_content = opcode_table_template.render(cpu_data=cpu_data)
    sequence_table_content = sequence_tables_template.render(
        cpu_data=cpu_data, sequences=sequences
    )

    args.opcode_output.parent.mkdir(parents=True, exist_ok=True)
    with args.opcode_output.open("w") as f:
        f.write(opcode_table_content)

    args.sequence_output.parent.mkdir(parents=True, exist_ok=True)
    with args.sequence_output.open("w") as f:
        f.write(sequence_table_content)


def generate_rust_op_skeleton(cpu_data: CpuData, args: argparse.Namespace):
    """Print a starting point skeleton for Rust implementation of all cpu ops"""
    op_skeleton_template = JINJA_ENV.get_template("op_skeleton.rs.jinja2")
    op_comments = get_keyed_comments_for_ops(cpu_data)
    op_skeleton_content = op_skeleton_template.render(
        cpu_data=cpu_data, op_comments=op_comments
    )

    print(op_skeleton_content)


def generate_rust_action_skeleton(cpu_data: CpuData, args: argparse.Namespace):
    """Print a starting point skeleton for Rust implementation of all cpu actions"""
    action_skeleton_template = JINJA_ENV.get_template("action_skeleton.rs.jinja2")
    action_comments = get_keyed_comments_for_actions(cpu_data)
    action_skeleton_content = action_skeleton_template.render(
        cpu_data=cpu_data, action_comments=action_comments
    )

    print(action_skeleton_content)


def get_keyed_rust_comments(
    file_path: Path, pattern: SgConfig
) -> Dict[str, Dict[str, str]]:
    """
    Gets a table of all comments matching the pattern "// @<key>: <value>" in the given source file

    The pattern is an ast-grep rule definition for matching eligible comments. This function expects
    the rule to include a meta-variable $NAME which maps to the name of the function enclosing the
    comment.

    Returns a dictionary mapping function names (given by the $NAME metavar) to their comment key-value pairs
    """
    with file_path.open("r") as f:
        contents = f.read()
    root = SgRoot(contents, "Rust")
    root_node = root.root()

    items: Dict[str, Dict[str, str]] = defaultdict(dict)
    for match in root_node.find_all(pattern):
        comment_text = match.text()
        name = match["NAME"].text()

        if m := re.search(r"@(?P<key>[^: ]+):\s*(?P<value>.*?)\s*$", comment_text):
            key = m.group("key")
            value = m.group("value")
            if key in items[name]:
                print(f"Warning: Duplicate {key} for {name}")
            items[name][key] = value
    return items


# Pattern for matching comments in Rust op function implementations
RUST_OP_COMMENT_PATTERN: SgConfig = {
    "rule": {
        "kind": "line_comment",
        "regex": "^// @[^ ]+:",
        "inside": {"stopBy": "end", "pattern": "pub fn $NAME($$$) {$$$}"},
    }
}
# Pattern for matching comments in Rust action function implementations
RUST_ACTION_COMMENT_PATTERN: SgConfig = {
    "rule": {
        "kind": "line_comment",
        "regex": "^// @[^ ]+:",
        "pattern": "$TAG",
        # ast-grep sees "raw" macros before expansion, so these rules
        # need to describe the raw token trees
        "inside": {
            # Match the block for the function implementation
            "kind": "token_tree",
            "follows": {
                # Match the function name identifier
                "stopBy": "end",
                "kind": "identifier",
                "not": {
                    # Exclude the identifier for the optional cpu argument
                    # by checking if the preceeding token is the lambda '|'
                    "follows": {"pattern": "|"}
                },
                "pattern": "$NAME",
            },
            "inside": {
                # Match the top-level macro block
                "kind": "token_tree",
                "inside": {
                    # Match the top-level macro invocation node
                    "kind": "macro_invocation"
                },
            },
        },
    }
}


def validate_rust_code(cpu_data: CpuData, args: argparse.Namespace):
    """
    Validate that the Rust implementation of CPU ops and actions matches the architectural
    description given in the CPU data file

    This function checks that the pseudocode and flags comments on all functions are in sync
    with their architectural definitions
    """
    op_comments = get_keyed_rust_comments(args.ops_file, RUST_OP_COMMENT_PATTERN)
    action_comments = get_keyed_rust_comments(
        args.actions_file, RUST_ACTION_COMMENT_PATTERN
    )

    # CPU data has op names in upper case, convert code to match
    op_comments = {k.upper(): v for k, v in op_comments.items()}
    # CPU data has action names in lower case, convert code to match
    action_comments = {k.lower(): v for k, v in action_comments.items()}

    expected_op_comments = get_keyed_comments_for_ops(cpu_data)
    expected_action_comments = get_keyed_comments_for_actions(cpu_data)

    for op, expected_comments in expected_op_comments.items():
        if op not in op_comments:
            print(f"Missing implementation for op {op}")
        else:
            for key, value in expected_comments.items():
                if key not in op_comments[op]:
                    print(f"Missing {key} for op {op}")
                elif op_comments[op][key] != value:
                    print(
                        f"Mismatch for {key} in op {op}: expected '{value}', code currently has '{op_comments[op][key]}'"
                    )

    for action, expected_comments in expected_action_comments.items():
        if action not in action_comments:
            print(f"Missing implementation for action {action}")
        else:
            for key, value in expected_comments.items():
                if key not in action_comments[action]:
                    print(f"Missing {key} for action {action}")
                elif action_comments[action][key] != value:
                    print(
                        f"Mismatch for {key} in action {action}: expected '{value}', code currently has '{action_comments[action][key]}'"
                    )


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Generator for files derived from CPU data"
    )
    parser.add_argument(
        "--data_file",
        type=Path,
        default=SCRIPT_DIR.parent / "data" / "6502.yaml",
        help="Path to CPU data file",
    )

    subparsers = parser.add_subparsers(dest="command", required=True)

    parser_rust_codegen = subparsers.add_parser(
        "rust_codegen", help="Generate Rust code from CPU data"
    )
    parser_rust_codegen.add_argument(
        "--opcode_output",
        type=Path,
        default=SCRIPT_DIR.parent
        / "host/nes_emu/src/components/cpu/opcodes/opcode_table.rs",
        help="Path to the generated opcode table file",
    )
    parser_rust_codegen.add_argument(
        "--sequence_output",
        type=Path,
        default=SCRIPT_DIR.parent
        / "host/nes_emu/src/components/cpu/sequences/sequence_tables.rs",
        help="Path to the generated CPU cycle sequence file",
    )

    _parser_rust_gen_op_skeleton = subparsers.add_parser(
        "rust_gen_op_skeleton",
        help="Generate skeleton Rust code for instruction data ops",
    )
    _parser_rust_gen_action_skeleton = subparsers.add_parser(
        "rust_gen_action_skeleton",
        help="Generate skeleton Rust code for CPU cycle actions",
    )

    parser_rust_validate = subparsers.add_parser(
        "rust_validate",
        help="Validate Rust op and action implementations against the CPU data file",
    )
    parser_rust_validate.add_argument(
        "--ops_file",
        type=Path,
        default=SCRIPT_DIR.parent / "host/nes_emu/src/components/cpu/ops/op_impls.rs",
        help="Path to the Rust ops implementation file",
    )
    parser_rust_validate.add_argument(
        "--actions_file",
        type=Path,
        default=SCRIPT_DIR.parent
        / "host/nes_emu/src/components/cpu/sequences/actions.rs",
        help="Path to the Rust action implementation file",
    )

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
        case "rust_validate":
            validate_rust_code(cpu_data, args)
        case _:
            raise ValueError(f"Unknown command {args.command}")
