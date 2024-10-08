from __future__ import annotations

from nonebot.adapters import Message
from nonebot.params import CommandArg
from nonebot.adapters.discord import MessageSegment

import aiohttp
from monad_std.prelude import siter

from lootbox.utils import box_command
from lootbox.randbox import (
    BOX_NON_PLAIN_TEXT_PARAM_TEXT,
    BOX_WRONG_PARAM_TEXT,
    BACKEND_URL,
    RandMessage,
)


rand_box = box_command(("",))


@rand_box.handle()
async def rand_handle(args: Message = CommandArg()):
    if not siter(args).fold(True, lambda acc, x: acc and x.is_text()):
        await rand_box.finish(BOX_NON_PLAIN_TEXT_PARAM_TEXT)

    text = args.extract_plain_text()
    arg = text.split(" ")
    if len(arg) != 2:
        await rand_box.finish(BOX_WRONG_PARAM_TEXT)

    name_pat = str(arg[0])
    try:
        amount = int(arg[1])
    except ValueError:
        await rand_box.finish(BOX_WRONG_PARAM_TEXT)

    param = {
        "lang": "zh-sg",
        "box_name": name_pat,
        "amount": amount,
    }

    async with aiohttp.ClientSession() as session:
        async with session.post(BACKEND_URL, json=param) as response:
            resp = await response.json()
            if resp["status"] == "ok":
                data = (
                    siter(resp["data"])
                    .map(lambda t: message_to_local(RandMessage.parse(t)))
                    .collect_list()
                )
                await rand_box.finish(data)


def message_to_local(msg: RandMessage) -> MessageSegment | None:
    if msg.image:
        with open(msg.image, 'rb') as image:
            return MessageSegment.attachment(file=msg.image, content=image.read())
    elif msg.text:
        return MessageSegment.text(msg.text)
