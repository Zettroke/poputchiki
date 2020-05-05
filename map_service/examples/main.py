from map_service import MapService
import time

st = time.clock()
mp = MapService()
mp.load("Moscow.osm.gz")
en = time.clock()
print("aa:", en - st)
