from dataclasses import dataclass

@dataclass
class Field:
    """FIX Field"""
    version: str
    tag: int
    name: str
    type: str