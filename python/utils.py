from dataclasses import dataclass
from typing import Any


@dataclass
class TestCase:
    input: Any
    target: Any
    expected: Any
