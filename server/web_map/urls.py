from django.urls import path
from django.contrib import admin
from . import views

urlpatterns = [
    path('', views.index, name='index'),
    path('map', views.map_view, name='map'),
    path('main', views.main, name='main'),
    path('registration', views.registratioт, name='registration'),
    path('path_publish', views.path_publish, name='path_publish'),
    path('build_path', views.build_path, name='build_path'),
    path('add_transport', views.add_transport, name='add_transport'),
    path('my_transport', views.show_my_transport, name='my_transport'),
    path('user_paths', views.user_paths, name='user_paths'),
    path('user_map_view', views.user_map_view, name='user_map_view'),
]
