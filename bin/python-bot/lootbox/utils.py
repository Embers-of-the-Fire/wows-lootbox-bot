from nonebot import on_command
from nonebot.internal.matcher import Matcher
from monad_std.prelude import *


def box_command(args: tuple[str, ...], *aargs, **kwargs) -> type[Matcher]:
    assert len(args) >= 1, "at least one arg"

    base_it = siter(["box", "lootbox"]).flat_map(
        lambda t: siter(args).map(lambda x: (t, x))
    )

    first = base_it.next().unwrap()
    al = set(base_it.to_iter())
    return on_command(first, aliases=al, force_whitespace=False, *aargs, **kwargs)
