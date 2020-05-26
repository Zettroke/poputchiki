from typing import List


class MapPoint:
    lat: float
    lon: float

    def __init__(self, lat, lon):
        self.lat = lat
        self.lon = lon


class MapService:
    def build_path(self, points: List[MapPoint]):
        return [points[0], points[-1]]

