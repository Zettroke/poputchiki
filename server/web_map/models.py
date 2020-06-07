from django.contrib.auth.models import User
from django.db import models
# from django.contrib.postgres.fields import ArrayField

# Create your models here.
from web_map.map_manager import MapPoint, MapCarPath


class Transport(models.Model):
    name = models.CharField(max_length=255, default='Без названия')
    model = models.CharField(max_length=41)
    car_number = models.CharField(max_length=13)
    place = models.IntegerField()
    can_smoke = models.BooleanField()
    can_play_music = models.BooleanField()
    animals_allowed = models.BooleanField()
    contact = models.CharField(max_length=41)
    comment = models.CharField(max_length=200)
    user = models.ForeignKey(User, on_delete=models.CASCADE)


class UserPath(models.Model):
    user = models.ForeignKey(User, on_delete=models.CASCADE, related_name='paths')

    starts_at = models.DateTimeField()
    ends_at = models.DateTimeField()

    transport = models.ForeignKey(Transport, on_delete=models.CASCADE)

    def to_json(self):
        return {
            'id': self.id,
            'starts_at': self.starts_at,
            'points': [p.to_json() for p in self.points.all()]
        }

    def to_car_path(self):
        return MapCarPath(round(self.starts_at.timestamp()), [p.to_map_point() for p in self.points.all()])


class PathPoint(models.Model):
    user_path = models.ForeignKey(UserPath, on_delete=models.CASCADE, related_name='points')
    osm_id = models.BigIntegerField()
    lat = models.FloatField()
    lon = models.FloatField()

    def to_map_point(self):
        return MapPoint(
            self.osm_id,
            self.lat,
            self.lon,
            path_id=self.user_path_id
        )

    def to_json(self):
        return {
            'id': self.osm_id,
            'lat': self.lat,
            'lon': self.lon
        }
