from typing import List


class MapPoint:
    id: int
    lat: float
    lon: float
    path_id: int

    def __init__(self, id, lat, lon, **kwargs):
        self.id = id
        self.lat = lat
        self.lon = lon
        self.path_id = kwargs.get('path_id')

    def to_json(self):
        return self.__dict__


class MapCarPath:
    start_at: int
    path: List[MapPoint]

    def __init__(self, start_at, path):
        self.start_at = start_at
        self.path = path


class MapService:
    def build_path(self, points: List[MapPoint]):
        return [points[0], points[-1]]

    def build_path_using_cars(self, start_at, points, car_paths):
        return [points[0], points[-1]]

    def load(self, _s: str):
        return

