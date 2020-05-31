from django.http import HttpResponse, HttpRequest, HttpResponseBadRequest
from django.shortcuts import render
from django.contrib.auth.decorators import login_required
from django.contrib.auth.models import User
from django.views.decorators.csrf import csrf_exempt
import json
from web_map.map_manager import MapPoint, MapManager


def index(req):
    return HttpResponse("Kappa")


def map_view(req):
    return render(req, 'web_map/map_view.html', {'soma_data': 'kappa'})


def registratio(req):
    name = req.GET.get("name", "")
    email = req.GET.get("email", "")
    pas1 = req.GET.get("password1", "")
    pas2 = req.GET.get("password2", "")
    if pas1 == pas2 and len(pas1) > 3 and len(name) > 3:
        if not User.objects.filter(username = name).exists():
            User.objects.create_user(name, email, pas1)
            return HttpResponseRedirect("/accounts/login/")
        else:
            return render(req, "htmlfiles/registration.html", {'message':"this account is exist"})
    elif pas1 != pas2:
        return render(req, "htmlfiles/registration.html", {'message':"passwords don't match"})
    elif 4 > len(name) > 0:
        return render(req, "htmlfiles/registration.html", {'message':"name was too short"})
    else:
        return render(req, "htmlfiles/registration.html", {'message':""})


@login_required
def main(req):
    return render(req, 'htmlfiles/main.html', {'link':'main'})
    
    
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
