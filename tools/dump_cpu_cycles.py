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
    
class CpuData(BaseModel):
    access_modes: Dict[str, AccessModeData]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("cpu_data", type=Path, help="CPU data file")
    parser.add_argument("--actions-only", action="store_true", help="Only show actions")

    args = parser.parse_args()

    cpu_data = parse_yaml_file_as(CpuData, args.cpu_data)

    cycles = defaultdict(list)
    for access_mode, data in cpu_data.access_modes.items():
        for subtype, sequence in data.subtypes.items():
            sequence_name = f"{access_mode}.{subtype}"
            for cycle in sequence:
                if args.actions_only:
                    cycle = cycle[0]
                cycles[cycle].append(sequence_name)

    for cycle, sequence_names in sorted(cycles.items()):
        print(f"Cycle: {cycle}, Sequences: {sequence_names}")
    
    print(f"Num unique cycles: {len(cycles)}")

if __name__ == "__main__":
    main()
