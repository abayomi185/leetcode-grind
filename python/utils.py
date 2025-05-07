from dataclasses import dataclass, field
from typing import Any, Optional


@dataclass
class TestCase:
    input: Optional[Any] = field(default=None)
    target: Optional[Any] = field(default=None)
    expected: Optional[Any] = field(default=None)
