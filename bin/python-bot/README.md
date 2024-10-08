# Frontend server for the bot

## Install prerequisites

Run

```bash
pip install poetry     # install poetry
poetry env use python  # create environment
poetry install         # install dependencies
```

## Configure the server

For each target, add a `.env.prod` file to its source directory.

E.g. `console/.env.prod` is the configuration file for `console`.

### Console

```
DRIVER=~fastapi+~websockets
PORT=8002 # any port number
```

### QQ

```
DRIVER=~fastapi+~websockets
PORT=8000 # your frontend server port
```

For QQ User, you have to configure an extra Onebot client.

The client must support Onebot V11, and use reverse websocket at `/onebot/v11/ws`.

Take [Lagrange](https://github.com/LagrangeDev/Lagrange.Core/) as an example:

```json
{
	"Implementations": [
		{
			"Type": "ReverseWebSocket",
			"Host": "127.0.0.1",
			"Port": 8000, # this is your frontend server's port number
			"Suffix": "/onebot/v11/ws",
			"ReconnectInterval": 5000,
			"HeartBeatInterval": 5000,
			"AccessToken": ""
		}
	]
}
```

### Discord

```
DRIVER=~fastapi+~httpx+~websockets
PORT=8001 # any port number you want
DISCORD_BOTS='
[
  {
    "token": "<your discord bot's token>",
    "intent": {
      "guild_messages": true,
      "direct_messages": true
    },
    "application_commands": {"*": ["*"]}
  }
]
'
```
