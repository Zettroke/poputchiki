from django.contrib.auth.models import User
from django.db import models
from django.contrib.postgres.fields import ArrayField

# Create your models here.


class UserPath(models.Model):
    user = models.ForeignKey(User, on_delete=models.CASCADE)

    starts_at = models.DateTimeField()
    ends_at = models.DateTimeField()
    # osm_id нод
    path = ArrayField(models.BigIntegerField())
