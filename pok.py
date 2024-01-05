import requests
import json

API = "https://pokeapi.co/api/v2/pokemon/"

for i in range(1, 156):
    pokJson = json.loads(requests.get(API + str(i)).text)

    tmpPok = {
        "number": i,
        "name": '|{}|.into()'.format(pokJson["name"].capitalize()),
        "typ": "Type::{}".format(pokJson["types"][0]["type"]["name"].capitalize())

    }

    a = "tmp_pokemons.insert({}, Pokemon {}".format(i, str(json.dumps(tmpPok)).replace('"', "").replace("|", '"') + ");")
    print(a)

