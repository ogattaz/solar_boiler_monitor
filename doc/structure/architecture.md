

La logique est :
* L'automate doit être instancié et doit s'exécuter dans un thread.
* L'action read_values doit empiler des Valeurs (id, value) dans un queue FIFO.
* Le processor de timeseries doit être instancié et doit s'exécuter dans un thread.
* Le processeur dépile les  Valeurs et la met dans la bonne TimeSerie, appelle le client de victoriametrics pour la stocker

