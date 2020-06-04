from datetime import datetime

from django.http import HttpResponse, HttpRequest, HttpResponseBadRequest, HttpResponseRedirect, JsonResponse
from django.shortcuts import render
from django.contrib.auth.decorators import login_required
from django.contrib.auth.models import User
from django.views.decorators.csrf import csrf_exempt
import json
from web_map.map_manager import MapPoint, MapManager
from .models import Transport
from web_map.models import PathPoint, UserPath


def index(req):
    return HttpResponse("Kappa")


@login_required
def map_view(req):
    return render(req, 'map_view.html', {'soma_data': 'kappa'})

@csrf_exempt
def registratioт(req):
    name = req.POST.get("name", "")
    email = req.POST.get("email", "")
    pas1 = req.POST.get("password1", "")
    pas2 = req.POST.get("password2", "")
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
@csrf_exempt
def add_transport(req):
    print(req.user.username)
    if req.method == 'POST':
        new_transport = Transport()
        new_transport.model = req.POST.get("model", "")
        new_transport.car_number = req.POST.get("car_number", "")
        new_transport.place = req.POST.get("place", 1)
        if "smoking" in req.POST.get("option", []):
            new_transport.option1 = True
        else:
            new_transport.option1 = False
        if "music" in req.POST.get("option", []):
            new_transport.option2 = True
        else:
            new_transport.option2 = False
        if "dog" in req.POST.get("option", []):
            new_transport.option3 = True
        else:
            new_transport.option3 = False
        new_transport.contact = req.POST.get("contact_data", "")
        new_transport.comment = req.POST.get("comment", "")
        new_transport.user = req.user
        new_transport.save()
        return HttpResponseRedirect("/main")
    return render(req, "htmlfiles/add_transport.html")

@login_required
def show_my_transport(req):
    transport = Transport.objects.all().filter(user=req.user)
    return render(req, "htmlfiles/show_transport.html", {'transports': transport})

@login_required
def main(req):
    return render(req, 'htmlfiles/main.html', {'link':'main'})

    
@csrf_exempt
def path_publish(req: HttpRequest):
    if req.method == 'GET':
        return render(req, 'map_view.html', {'soma_data': 'kappa'})
    elif req.method == 'POST':
        data = json.loads(req.POST['data'])
        path_points = [MapPoint(v.get('id', 0), v['lat'], v['lon']) for v in data]
        if any([p.id == 0 for p in path_points]):
            return HttpResponseBadRequest('osm node id 0 is forbidden!!')
        user_path = UserPath.objects.create(user=req.user, starts_at=datetime.now(), ends_at=datetime.now())
        PathPoint.objects.bulk_create([
            PathPoint(osm_id=p.id, lat=p.lat, lon=p.lon, user_path=user_path) for p in path_points
        ], batch_size=200)

        return HttpResponse()


@csrf_exempt
def build_path(req: HttpRequest):
    """
        Используя опорные точки маршрута, переданные в запросе, строит подробный маршрут и возвращает его.
        !!! Сейчас возвращает первую и послдние точки маршрута, из-за неготовности алгоритма.
    """
    if req.method == 'POST':
        points = [MapPoint(v.get('id', 0), v['lat'], v['lon']) for v in json.loads(req.body)]
        path = MapManager.get_service().build_path(points)
        return HttpResponse(json.dumps([p.to_json() for p in path]))
    return HttpResponseBadRequest()


@login_required
def user_map_view(req: HttpRequest):
    return render(req, 'user_map_view.html')


def user_paths(req: HttpRequest):
    paths = UserPath.objects.all().prefetch_related('points').order_by('id').reverse()[:6]
    path_list = [
        p.to_json() for p in paths.all()
    ]
    return JsonResponse(path_list, safe=False)
