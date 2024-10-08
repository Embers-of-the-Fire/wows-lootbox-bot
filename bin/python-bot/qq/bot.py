import nonebot
from nonebot.adapters.onebot.v11 import Adapter as OnebotAdapter

# 初始化 NoneBot
nonebot.init(command_start={"/", "", "."}, command_sep={" "})

# 注册适配器
driver = nonebot.get_driver()
driver.register_adapter(OnebotAdapter)

# 在这里加载插件
nonebot.load_builtin_plugins("echo")  # 内置插件
# nonebot.load_plugin("thirdparty_plugin")  # 第三方插件
# nonebot.load_plugins("awesome_bot/plugins")  # 本地插件

import lootbox
import randbox

if __name__ == "__main__":
    nonebot.run()
