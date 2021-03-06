from django.urls import path
from django.contrib import admin
from . import views

urlpatterns = [
    path('', views.index, name='index'),
    path('map', views.map_view, name='map'),
    path('main', views.main, name='main'),
    path('registration', views.registration, name='registration'),
    path('path_publish', views.path_publish, name='path_publish'),
    path('build_path', views.build_path, name='build_path'),
    path('add_transport', views.add_transport, name='add_transport'),
    path('my_transport', views.show_my_transport, name='my_transport'),
    path('delete_transport', views.delete_transport, name='delete_transport'),
    path('user_paths', views.user_paths, name='user_paths'),
    path('user_map_view', views.user_map_view, name='user_map_view'),
    path('build_user_path', views.build_user_path, name='build_user_path'),
]
