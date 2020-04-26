# Проект Попутчики

Сервис позволяет сторить маршрут пользователя таким образом, что бы была возможность воспользоваться
автотранспортом других зарегистрированных пользователей при поездке.

Пользователи автомобилисты будут публиковать свои поездки в сервисе.
В последствии когда пользователь пассажир будет искать маршрут для своей поездки,
сервис будет строить маршрут таким образом, что бы его часть проходила попутно
маршрутам пользователей автомобилистов.

### Компоненты проекта:
* Сервер на `Flask`/`Django`
* База данных `PostgreSQL`
* Веб-сайт с картой
* Модуль для `Python` реализующий работу с графом дорог