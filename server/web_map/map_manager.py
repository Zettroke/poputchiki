try:
    from map_service import MapService, MapPoint
except Exception:
    from web_map.map_service_mock import MapService, MapPoint

MapService = MapService
MapPoint = MapPoint


class MapManager:
    _map_service = None

    @staticmethod
    def get_service() -> MapService:
        if not MapManager._map_service:
            MapManager._map_service = MapService()
            MapManager._map_service.load('./Moscow.osm.gz')
        return MapManager._map_service
