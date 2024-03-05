from pathlib import Path
import subprocess
import re
import logging

logging.getLogger("acquire").setLevel(logging.CRITICAL)

DOCS_REPO = "https://github.com/acquire-project/acquire-docs"
CODE_BLOCK = re.compile(r"```python\n(.*?)```", re.DOTALL)
SKIP = {
    "setup.md",  # has invalid syntax
    "trigger.md",  # has some non-existant paths
}


def pytest_generate_tests(metafunc):
    """This pytest hook will clone the docs and parametrize tests.

    "tutorial" is a fixture name that will be parametrized with tutorials
    """
    if not (docs_path := Path("acquire-docs", "docs")).exists():
        subprocess.check_call(["git", "clone", DOCS_REPO])

    if "tutorial" in metafunc.fixturenames:
        tuts = [docs_path / "get_started.md"]
        tuts.extend(
            [fn for fn in docs_path.glob("tutorials/*.md") if fn.name not in SKIP]
        )
        metafunc.parametrize("tutorial", tuts, ids=lambda p: p.name)


def test_tutorials(tutorial: Path):
    for code_block in CODE_BLOCK.finditer(tutorial.read_text()):
        exec(code_block.group(1))
