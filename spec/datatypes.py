from dataclasses import dataclass


@dataclass
class Field:
    """FIX Field"""
    tag: int
    name: str
    type: str