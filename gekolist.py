#from pycoingecko import CoinGeckoAPI

#cg = CoinGeckoAPI()

#coins = cg.get_search_trending()
#print(coins)


import requests

r = requests.get("https://tokens.coingecko.com/uniswap/all.json")
response = r.json()

# coinlist = open("gecko.json","w+")

# coinlist.write(f"{response}")

# coinlist.close()
print(response["tokens"])


        
