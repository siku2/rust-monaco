import dataclasses
import re
from typing import Tuple

from .helpers import MatchError, consume_match, remove_indent


# Matches multiline documentation in the form of
# /**
#  * Hello world!
#  */
_PATTERN_DOC = re.compile(r"^ *\/\*\*\n(?: *\*(?: .*)?$\n)+ *\*\/\n", re.MULTILINE)


@dataclasses.dataclass()
class Documented:
    documentation: str

    @staticmethod
    def consume(s: str) -> Tuple[str, str]:
        try:
            match, s = consume_match(_PATTERN_DOC, s)
        except MatchError:
            return "", s

        lines = remove_indent(match[0]).splitlines()
        doc = "\n".join(line[3:] for line in lines[1:-1])
        return doc, s

    def rust_documentation(self) -> str:
        return "\n".join(f"/// {line}" for line in self.documentation.splitlines())
