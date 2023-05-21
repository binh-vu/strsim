import pkgutil
import strsim
from pathlib import Path
from importlib import import_module


def test_import(pkg=strsim, ignore_deprecated: bool = True):
    stack = [(pkg.__name__, Path(pkg.__file__).parent.absolute())]

    while len(stack) > 0:
        pkgname, pkgpath = stack.pop()
        for m in pkgutil.iter_modules([str(pkgpath)]):
            mname = f"{pkgname}.{m.name}"
            if ignore_deprecated and mname.find("deprecated") != -1:
                continue
            if m.ispkg:
                stack.append((mname, pkgpath / m.name))
            import_module(mname)


def test_levenshtein_similarity():
    tokenizer = strsim.CharacterTokenizer()
    testcases = [("abc", "def", 0.0), ("aaa", "aaa", 1.0)]
    for k, q, sim in testcases:
        assert (
            strsim.levenshtein_similarity(tokenizer.tokenize(k), tokenizer.tokenize(q))
            == sim
        )
