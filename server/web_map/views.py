from django.http import HttpResponse, HttpRequest, HttpResponseBadRequest
from django.shortcuts import render
from django.views.decorators.csrf import csrf_exempt
import json

from web_map.map_manager import MapPoint, MapManager


def index(req):
    return HttpResponse("Kappa")


@csrf_exempt
def path_publish(req: HttpRequest):
    if req.method == 'GET':
        return render(req, 'map_view.html', {'soma_data': 'kappa'})
    elif req.method == 'POST':
        o = [MapPoint(v['lat'], v['lon']) for v in json.loads(req.body)]
        # TODO: STORE PATH
        return HttpResponse('')


@csrf_exempt
def build_path(req: HttpRequest):
    """
        Используя опорные точки маршрута, переданные в запросе, строит подробный маршрут и возвращает его.
        !!! Сейчас возвращает первую и послдние точки маршрута, из-за неготовности алгоритма.
    """
    if req.method == 'POST':
        points = [MapPoint(v['lat'], v['lon']) for v in json.loads(req.body)]
        path = MapManager.get_service().build_path(points)
        return HttpResponse(json.dumps([{'lat': p.lat, 'lon': p.lon} for p in path]))
    return HttpResponseBadRequest()
