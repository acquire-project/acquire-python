import atexit
from pathlib import Path
import shutil
import subprocess
import re
import logging
import tempfile
import pytest


logging.getLogger("acquire").setLevel(logging.CRITICAL)

DOCS_REPO = "https://github.com/acquire-project/acquire-docs"
CODE_BLOCK = re.compile(r"```python\n(.*?)```", re.DOTALL)
SKIP = {
    "setup.md",  # has invalid syntax
    "trigger.md",  # has some non-existant paths
}


def tutorials():
    tmp_path = Path(tempfile.mkdtemp())
    subprocess.check_call(["git", "clone", DOCS_REPO], cwd=str(tmp_path))
    docs_path = tmp_path / "acquire-docs" / "docs"

    tuts = []
    if (get_started := docs_path / "get_started.md").exists():
        tuts.append(get_started)
    tuts.extend([fn for fn in docs_path.glob("tutorials/*.md") if fn.name not in SKIP])
    tuts.sort()

    @atexit.register
    def cleanup():
        shutil.rmtree(tmp_path, ignore_errors=True)

    return tuts


@pytest.mark.parametrize("tutorial", tutorials(), ids=lambda x: x.name)
def test_tutorials(tutorial: Path):
    for code_block in CODE_BLOCK.finditer(tutorial.read_text()):
        exec(code_block.group(1))
