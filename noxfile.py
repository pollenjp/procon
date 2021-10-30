# Standard Library
import time
import uuid
from pathlib import Path
from typing import Dict, List

# Third Party Library
import nox

python_code_path_list: List[str] = [
    "abc",
    "arc",
    "contests",
    "template",
    "noxfile.py",
]

env_common: Dict[str, str] = {}

nox_tmp_dir: Path = Path(__file__).parent / ".nox_tmp"


def install_package(session: nox.sessions.Session, dev: bool = False):
    requirements_txt_path: Path = nox_tmp_dir / f"pipenv-requirements-{str(uuid.uuid4())}.txt"
    requirements_txt_path.parent.mkdir(exist_ok=True, parents=True)
    try:
        with open(str(requirements_txt_path), "wt") as f:
            cmd = ["pipenv", "lock", "-r"] + (["--dev"] if dev else [])
            session.run(*cmd, external=True, stdout=f)
        time.sleep(1)  # Installing may starts before requirement's file is created.
        session.install("-r", str(requirements_txt_path))
    except Exception as e:
        requirements_txt_path.unlink(missing_ok=True)
        raise e
    requirements_txt_path.unlink(missing_ok=True)


@nox.session(python=["3.8"])
def lint(session):
    env: Dict[str, str] = {}
    env.update(env_common)

    install_package(session, dev=True)
    session.run("flake8", "--statistics", "--count", "--show-source", *python_code_path_list, env=env)
    # for python_code_path in python_code_path_list:
    #     # Exclude multiple directories on mypy commandline: improve documentation · Issue #10250 · python/mypy
    #     # <https://github.com/python/mypy/issues/10250>
    #     session.run("mypy", python_code_path, env=env)
    session.run("isort", "--check", *python_code_path_list, env=env)
    session.run("black", "--check", *python_code_path_list, env=env)
