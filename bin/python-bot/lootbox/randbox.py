from __future__ import annotations


BOT_HELP_TEXT = """使用方法：
box <物品名称> <数量>
示例：
box 超级补给箱 100
备注：
有多个相似名称时会出错"""

BOX_WRONG_PARAM_TEXT = "参数错误\n" + BOT_HELP_TEXT

BOX_NON_PLAIN_TEXT_PARAM_TEXT = (
    "检测到非纯文本入参，请不要使用表情符号等\n" + BOT_HELP_TEXT
)

BACKEND_URL = "http://localhost:8080/lootbox/rand"

class RandMessage:
    image: str | None
    text: str | None

    def __init__(self, im, t) -> None:
        self.image = im
        self.text = t

    @staticmethod
    def from_image(i: str) -> RandMessage:
        return RandMessage(i, None)

    @staticmethod
    def from_text(i: str) -> RandMessage:
        return RandMessage(None, i)

    @staticmethod
    def parse(i: dict) -> RandMessage | None:
        (ty, v) = next(iter(i.items()))

        match ty.lower():
            case "image":
                return RandMessage.from_image(v)
            case "text":
                return RandMessage.from_text(v)
