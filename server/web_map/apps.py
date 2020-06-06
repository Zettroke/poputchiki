from threading import Thread

from django.apps import AppConfig

from web_map.map_manager import MapManager


class WebMapConfig(AppConfig):
    name = 'web_map'

    def ready(self):
        pass
        # Thread(target=lambda: MapManager.get_service(), daemon=False).run()

