from django.urls import path

from . import views

urlpatterns = [
    path('', views.index, name='index'),
    path('path_publish', views.path_publish, name='path_publish'),
    path('build_path', views.build_path, name='build_path'),
]
