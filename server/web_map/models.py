from django.contrib.auth.models import User
from django.db import models
# from django.contrib.postgres.fields import ArrayField

# Create your models here.
from web_map.map_manager import MapPoint


class UserPath(models.Model):
    user = models.ForeignKey(User, on_delete=models.CASCADE, related_name='paths')

    starts_at = models.DateTimeField()
    ends_at = models.DateTimeField()
    # osm_id нод
    # path = ArrayField(models.BigIntegerField())


class Transport(models.Model):
    model = models.CharField(max_length=41)
    car_number = models.CharField(max_length=13)
    place = models.IntegerField()
    option1 = models.BooleanField()
    option2 = models.BooleanField()
    option3 = models.BooleanField()
    contact = models.CharField(max_length=41)
    comment = models.CharField(max_length=200)
    user = models.ForeignKey(User, on_delete=models.CASCADE)


class PathPoint(models.Model):
    user_path = models.ForeignKey(UserPath, on_delete=models.CASCADE, related_name='points')
    osm_id = models.BigIntegerField()
    lat = models.FloatField()
    lon = models.FloatField()

    def to_map_point(self):
        return MapPoint(self.osm_id, self.lat, self.lon)

    def to_json(self):
        return {
            'id': self.osm_id,
            'lat': self.lat,
            'lon': self.lon
        }