from pydantic import BaseModel

from typing import Dict, List, NamedTuple

# List of valid types of memory access operations
VALID_MEM_CYCLES = set((
    "inc_read_pc",
    "read_pc",
    "inc_read_tmp",
    "read_tmp",
    "inc_write_tmp",
    "write_tmp",
    "inc_read_stk",
    "read_stk",
    "inc_push_stk",
    "push_stk",
    "pop_stk",
))

class CpuCycle(NamedTuple):
    """Represents a definition of a single CPU cycle"""
    action_name: str
    """Name of the action to perform on core state"""
    mem_cycle: str
    """Name of the memory access cycle to perform"""

class AccessModeData(BaseModel):
    """Represents the definition of a memory access mode"""
    inst_arg: str
    """Suffix to this access mode of a mnemonic"""
    sequences: Dict[str, List[CpuCycle]]
    """Operation sequences for each subtype of this access mode. Keys are the access mode subtype"""

class MnemonicData(BaseModel):
    """Represents the definition of an instruction mnemonic"""
    description: str
    """Description of this mnemonic"""
    access_subtype: str
    """Which subtype of the instruction's access mode is used for this mnemonic"""
    op: str | None = None
    """Optional pseudocode for the data operation attached to this mnemonic"""
    flags: str | None = None
    """Optional pseudocode for flags impacted by this mnemonic"""
    branch_cond: str | None = None
    """For branch type operations, the branch condition"""

class InstructionData(BaseModel):
    """Represents the definition of a CPU instruction"""
    mnemonic: str
    """The instruction's mnemonic"""
    access_mode: str
    """The memory access mode used for this instruction"""

class CpuData(BaseModel):
    """Represents the overall definition of the CPU architecture"""
    actions: Dict[str, str]
    """Table mapping CPU internal action names to their pseudocode definitions"""
    access_modes: Dict[str, AccessModeData]
    """Table of memory access mode definitions"""
    mnemonics: Dict[str, MnemonicData]
    """Table of instruction mnemonic definitions"""
    instructions: Dict[int, InstructionData]
    """Table of instruction opcode definitions"""

    dispatch_seq: List[CpuCycle]
    """The CPU cycle sequence for dispatching an instruction"""
    reset_seq: List[CpuCycle]
    """The CPU cycle sequence executed on reset release"""
    irq_seq: List[CpuCycle]
    """The CPU cycle sequence executed on IRQ"""
    nmi_seq: List[CpuCycle]
    """The CPU cycle sequence executed on NMI"""

    def validate(self):
        """Verify that all data is internally consistent"""

        # Collect a table of all cycle sequences
        all_sequences = {
            "DISPATCH": self.dispatch_seq,
            "RESET": self.reset_seq,
            "IRQ": self.irq_seq,
            "NMI": self.nmi_seq,
        }
        for access_mode, data in self.access_modes.items():
            for subtype_name, sequence in data.sequences.items():
                sequence_name = f"{access_mode}.{subtype_name}"
                all_sequences[sequence_name] = sequence

        # Validate that all actions in all sequences are valid
        for sequence_name, sequence in all_sequences.items():
            for i, cycle in enumerate(sequence):
                assert cycle.action_name in self.actions, f"Invalid action '{cycle.action_name}' in {sequence_name}.{i}"
                assert cycle.mem_cycle in VALID_MEM_CYCLES, f"Invalid memory cycle '{cycle.mem_cycle}' in {sequence_name}.{i}"

        # Validate each instruction
        for opcode, inst_data in self.instructions.items():
            assert opcode >= 0 and opcode <= 0xFF, f"Invalid opcode 0x{opcode:02X}"
            assert inst_data.mnemonic in self.mnemonics, f"Invalid mnemonic '{inst_data.mnemonic}' in opcode 0x{opcode:02X}"
            assert inst_data.access_mode in self.access_modes, f"Invalid access mode '{inst_data.access_mode}' in opcode 0x{opcode:02X}"

            mnemonic_data = self.mnemonics[inst_data.mnemonic]
            assert mnemonic_data.access_subtype in self.access_modes[inst_data.access_mode].sequences, f"Invalid access subtype '{mnemonic_data.access_subtype}' in opcode 0x{opcode:02X}"