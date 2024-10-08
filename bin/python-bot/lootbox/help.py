from .utils import box_command


bot_help = box_command(("help",))

BOT_HELP_TEXT = """使用方法：
box <物品名称> <数量>
示例：
box 超级补给箱 100
备注：
有多个相似名称时会出错"""


@bot_help.handle()
async def help_handle():
    await bot_help.finish(BOT_HELP_TEXT)
