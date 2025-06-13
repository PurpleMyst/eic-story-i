# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "argh==0.26.2",
#     "python-dotenv==1.0.0",
#     "requests==2.27.1",
#     "termcolor==1.1.0",
#     "tomlkit==0.12.3",
# ]
# ///
import shlex
import subprocess
import sys
import typing as t
from contextlib import chdir
from datetime import datetime
from functools import partial, wraps
from os import environ
from pathlib import Path

import requests
import tomlkit as toml
from argh import aliases, dispatch_commands, wrap_errors
from dotenv import load_dotenv
from termcolor import colored as c

cb = partial(c, attrs=["bold"])

MAIN = """\
fn main() {{
    let (part1, part2, part3) = {crate}::solve();
    println!("{{part1}}");
    println!("{{part2}}");
    println!("{{part3}}");
}}\
"""

LIB = """\
use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    ("TODO", "TODO", "TODO")
}\
"""

DEFAULT_BASELINE = "previous"

WORKSPACE_MANIFEST_PATH = Path(__file__).parent / "Cargo.toml"

load_dotenv()

session = requests.Session()
session.headers.update({"User-Agent": "PurpleMyst/aoc-template with much love! <3"})


def run(cmd: t.Sequence[str | Path], /, **kwargs) -> subprocess.CompletedProcess:
    check = kwargs.pop("check", True)
    print(
        cb("$", "green"),
        shlex.join(map(str, cmd)),
        c(f"(w/ options {kwargs})", "green") if kwargs else "",
    )
    proc = subprocess.run(cmd, **kwargs)
    if check and proc.returncode != 0:
        print(cb("Failed.", "red"))
        sys.exit(proc.returncode)
    return proc


def add_line(p: Path, l: str) -> None:
    ls = p.read_text().splitlines()
    ls.insert(-1, l)
    if ls[-1] != "":
        # add or keep trailing newline
        ls.append("")
    p.write_text("\n".join(ls), newline="\n")


def in_root_dir(f):
    @wraps(f)
    def inner(*args, **kwargs):
        with chdir(Path(__file__).parent):
            return f(*args, **kwargs)

    return inner


@in_root_dir
@aliases("ss")
@wrap_errors((requests.HTTPError,))
def start_solve(problem_num: int) -> None:
    "Start solving a problem."
    crate = f"problem{problem_num:02}"
    crate_path = Path(crate)

    if crate_path.exists():
        print(f"{crate} already exists.")
        return

    manifest = toml.parse(WORKSPACE_MANIFEST_PATH.read_text())
    if crate not in manifest["workspace"]["members"]:  # type: ignore
        manifest["workspace"]["members"].append(crate)  # type: ignore

    metadata = manifest["workspace"].setdefault("metadata", {})  # type: ignore
    metadata[crate] = {"start_time": datetime.now()}

    with WORKSPACE_MANIFEST_PATH.open("w") as manifest_f:
        toml.dump(manifest, manifest_f)

    run(("cargo", "new", "--bin", crate))
    run(
        (
            "cargo",
            "add",
            "--manifest-path",
            "benchmark/Cargo.toml",
            "--path",
            crate,
            crate,
        )
    )

    src = crate_path / "src"
    (src / "main.rs").write_text(MAIN.format(crate=crate), newline="\n")
    (src / "lib.rs").write_text(LIB, newline="\n")

    benches = Path("benchmark", "benches")
    add_line(benches / "criterion.rs", f"    {crate},")
    add_line(benches / "iai.rs", f"    {crate}: {crate}_solve,")

    run(("git", "add", crate))


@aliases("sb")
@in_root_dir
def set_baseline(day: str, name: str = DEFAULT_BASELINE) -> None:
    "Run a criterion benchmark, setting its results as the new baseline."
    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            day,
            "--save-baseline",
            name,
            "--verbose",
        )
    )


@aliases("cmp")
@in_root_dir
def compare(day: str, name: str = DEFAULT_BASELINE) -> None:
    "Run a criterion benchmark, comparing its results to the saved baseline."
    run(
        (
            "cargo",
            "bench",
            "--bench",
            "criterion",
            "--",
            day,
            "--baseline",
            name,
            "--verbose",
        )
    )


@in_root_dir
@aliases("cmp-stash")
def compare_by_stashing(day: str, name: str = DEFAULT_BASELINE) -> None:
    "Stash the current changes, set the baseline and then compare the new changes."
    run(("git", "stash", "push", "-m", "Stashing for benchmarking"))
    set_baseline(day, name)
    run(("git", "stash", "pop"))
    compare(day, name)


@in_root_dir
def criterion(day: str) -> None:
    "Run a criterion benchmark, without caring about baselines."
    run(("cargo", "bench", "--bench", "criterion", "--", day, "--verbose"))


def main() -> None:
    # environ["RUST_BACKTRACE"] = "1"
    environ["RUSTFLAGS"] = "-C target-cpu=native"
    dispatch_commands(
        (
            set_baseline,
            compare,
            compare_by_stashing,
            criterion,
            start_solve,
        ),
    )


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Bye!")
