from pathlib import Path
import subprocess
import re
import logging
import pytest
import os

if not os.getenv("CI") or os.getenv("SKIP_DOCS_TEST", False):
    pytest.skip("Skipping docs test", allow_module_level=True)


logging.getLogger("acquire").setLevel(logging.CRITICAL)

DOCS_REPO = "https://github.com/acquire-project/acquire-docs"
CODE_BLOCK = re.compile(r"```python\n(.*?)```", re.DOTALL)
SKIP = {
    "setup.md",  # has invalid syntax
    "trigger.md",  # has some non-existant paths
}

# NOTE: this clones the repo on import... not the best practice, could be improved
if not (DOCS_PATH := Path("acquire-docs", "docs")).exists():
    subprocess.check_call(["git", "clone", DOCS_REPO])


TUTS = [DOCS_PATH / "get_started.md"]
TUTS.extend([fn for fn in DOCS_PATH.glob("tutorials/*.md") if fn.name not in SKIP])
TUTS.sort()


@pytest.mark.parametrize("tutorial", TUTS, ids=lambda x: x.name)
def test_tutorials(tutorial: Path):
    for code_block in CODE_BLOCK.finditer(tutorial.read_text()):
        exec(code_block.group(1))
