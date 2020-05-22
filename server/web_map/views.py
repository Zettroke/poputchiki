from django.http import HttpResponse, HttpResponseRedirect
from django.shortcuts import render
from django.contrib.auth.decorators import login_required
from django.contrib.auth.models import User

# Create your views here.
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