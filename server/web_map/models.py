from django.contrib.auth.models import User
from django.db import models
# from django.contrib.postgres.fields import ArrayField

# Create your models here.


class UserPath(models.Model):
    user = models.ForeignKey(User, on_delete=models.CASCADE)

    starts_at = models.DateTimeField()
    ends_at = models.DateTimeField()
    # osm_id нод
    # path = ArrayField(models.BigIntegerField())

class Transport(models.Model):
    model = models.CharField(max_length=41)
    car_namber = models.CharField(max_length=13)
    place = models.IntegerField()
    option1 = models.BooleanField()
    option2 = models.BooleanField()
    option3 = models.BooleanField()
    conect = models.CharField(max_length=41)
    coment = models.CharField(max_length=200)
    user = models.ForeignKey(User, on_delete=models.CASCADE)