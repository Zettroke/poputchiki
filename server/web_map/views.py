from django.http import HttpResponse, HttpRequest, HttpResponseBadRequest, HttpResponseRedirect
from django.shortcuts import render
from django.contrib.auth.decorators import login_required
from django.contrib.auth.models import User
from django.views.decorators.csrf import csrf_exempt
import json
from web_map.map_manager import MapPoint, MapManager
from .models import Transport

def index(req):
    return HttpResponse("Kappa")


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
        new_transport.car_namber = req.POST.get("car_namber", "")
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
        new_transport.conect = req.POST.get("conect_data", "")
        new_transport.coment = req.POST.get("coment", "")
        new_transport.user = req.user
        new_transport.save()
        return HttpResponseRedirect("/main")
    return render(req, "htmlfiles/add_transport.html")

@login_required
def show_my_transport(req):
    transport = Transport.objects.all().filter(user=req.user)
    list = ''
    for element in transport:
        list += '<li>model: {} <br> car namber: {} <br> free place: {:} <br> contact: \
        {} <br> coment: {}</li>\n'.\
        format(element.model, element.car_namber, element.place, element.conect, element.coment)
    return render(req, "htmlfiles/show_transport.html", {'list':list})

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

