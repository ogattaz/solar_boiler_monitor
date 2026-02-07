
# Description de l'automate

Glossaire:
* asap="as soon as possible"
* états : [IDLE], [TESTED] , [INITIALIZED], [CONNECTED], [READY]
* Événements externes : `start`, `pause`, `resume`, `stop`
* Événements interne : asap

Les règles :
- start --> (action: création de l'automate) --> [IDLE] 
- [IDLE] `asap`--> (action: diagnostique du réseau): ok--> [TESTED] | onError--> (action: waiting 30s) --> [IDLE]
- [TESTED] `asap`--> (action: initialise le dialogue): ok--> [INITIALIZED] | onError--> [IDLE]
- [INITIALIZED] `asap`--> (action: login): ok--> [CONNECTED] | onError--> (action: logoff) --> [IDLE]
- [CONNECTED] `asap`--> (action: lecture description): ok--> [READY] | onError--> (action: logoff) --> [IDLE]
- [READY] `every 30 secondes`--> (action: lecture valeurs): ok--> [READY] | onError--> (action: logoff) --> [IDLE]
- [READY] `every 3600 secondes`--> (action: logoff) --> [IDLE]
- [INITIALIZED] or [INITIALIZED] or [INITIALIZED] `stop`--> (action: logoff) --> [IDLE] `asap`--> (action: destruction de l'automate)
- [TESTED] `stop`--> [IDLE] `asap`--> (action: destruction de l'automate)
- [IDLE] `stop`--> (action: destruction de l'automate)
- [IDLE] or [TESTED] or [INITIALIZED] or [INITIALIZED] or [INITIALIZED] `asap`--> (action: l'automate est en pause) --> [same]
- [IDLE] or [TESTED] or [INITIALIZED] or [INITIALIZED] or [INITIALIZED] `asap`--> (action: l'automate redmarre) --> [same]

Données :
* un compteur par action: diagnostique, login, lecture description, lecture valeurs, logoff
* uptime
