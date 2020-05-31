from web_map.map_service_mock import MapService, MapPoint


MapPoint = MapPoint


class MapManager:
    _map_service = None

    @staticmethod
    def get_service() -> MapService:
        if not MapManager._map_service:
            MapManager._map_service = MapService()
        return MapManager._map_service
