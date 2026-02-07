
pour structurer mon projet "home_automation" qui générer une application, peux tu me proposer une arborescence de dossiers/fichiers à créer dans src/ pour isoler les trois pans de cette application: 1)l'automate de collecte , 2) le gestionnaire de séries temporelles 3) le client de victoriametrics.
Pour la partie automate de collecte, je voudrais isoler dans des fichiers source .rs l'automate et les six actions

```txt
src/
├── main.rs                  # Point d'entrée principal de l'application
├── lib.rs                   # Déclarations publiques et réexportations
├── config/                  # Configuration de l'application
│   └── mod.rs
├── automate/                # 1) Automate de collecte
│   ├── mod.rs               # Réexportations et initialisation
│   ├── state.rs             # Définition des états et événements
│   ├── machine.rs           # Logique principale de l'automate
│   ├── actions/             # Actions isolées
│   │   ├── mod.rs
│   │   ├── diagnostic.rs    # Action: diagnostique du réseau
│   │   ├── initialize.rs    # Action: initialisation du dialogue
│   │   ├── login.rs         # Action: login
│   │   ├── read_desc.rs     # Action: lecture description
│   │   ├── read_values.rs   # Action: lecture valeurs
│   │   └── logoff.rs        # Action: logoff
│   └── counters.rs          # Gestion des compteurs et uptime
├── timeseries/              # 2) Gestionnaire de séries temporelles
│   ├── mod.rs
│   ├── storage.rs           # Logique de stockage local (si nécessaire)
│   └── processor.rs         # Traitement des données avant envoi
└── victoriametrics/         # 3) Client VictoriaMetrics
    ├── mod.rs
    ├── client.rs            # Client HTTP pour VictoriaMetrics
    └── models.rs            # Modèles de données pour les métriques
```