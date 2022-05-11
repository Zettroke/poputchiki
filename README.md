# Проект Попутчики

>Сервис делался в панике за 2 недели, был использован отвратительный хак с использованием unsafe в части с графом. Мне за него стыдно. Но за проект мне не стыдно! <sub>учитывая обстаятельства его создания)</sub>

[Презентация](https://docs.google.com/presentation/d/1Pij5r4mVxGOouZMRSBviJjVJjYkjFk7dYo9r_X9FcQU/edit?usp=sharing)

[Видео-презентация(с комментариями)](https://youtu.be/ADRjINyaNrA?t=15688)

[Видео-демо(с комментариями)](https://youtu.be/ADRjINyaNrA?t=16023)

Сервис позволяет сторить маршрут пользователя таким образом, что бы была возможность воспользоваться
автотранспортом других зарегистрированных пользователей при поездке.

Пользователи автомобилисты будут публиковать свои поездки в сервисе.
В последствии когда пользователь пассажир будет искать маршрут для своей поездки,
сервис будет строить маршрут таким образом, что бы его часть проходила попутно
маршрутам пользователей автомобилистов.

### Компоненты проекта:
* База данных 
* Веб-сайт с картой и интерфейсом для взаимодействия
* Модуль реализующий работу с графом дорог

### Технологии
* Язык программирования - `Python`
* БД - `PostgreSQL`
* Сервер - `Flask`/`Django`
* Модуль для работы с графом - `Rust`
