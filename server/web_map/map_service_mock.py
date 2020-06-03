from typing import List


class MapPoint:
    id: int
    lat: float
    lon: float

    def __init__(self, lat, lon):
        self.id = 0
        self.lat = lat
        self.lon = lon

    def to_json(self):
        return self.__dict__


class MapService:
    def build_path(self, points: List[MapPoint]):
        return [points[0], points[-1]]

    def load(self, _s: str):
        return

