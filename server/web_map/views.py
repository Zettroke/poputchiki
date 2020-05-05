from django.http import HttpResponse
from django.shortcuts import render


# Create your views here.
def index(req):
    return HttpResponse("Kappa")


def map_view(req):
    return render(req, 'web_map/map_view.html', {'soma_data': 'kappa'})
