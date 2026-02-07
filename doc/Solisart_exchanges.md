# Solisart exchanges


## lecture_valeurs_donnees.php

### request


* Request URL : http://192.168.0.125/admin/divers/ajax/lecture_valeurs_donnees.php
* Request Method :   POST

Headers:
* cookie : PHPSESSID=4strdulvb7il13o9up0b5mctr1
* host : 192.168.0.125
* origin : http://192.168.0.125
* referer : http://192.168.0.125/admin/?page=installation&id=SC1Z20230801

id=SC1Z20230801 => base64 => id=U0MxWjIwMjMwODAx

```url
id=U0MxWjIwMjMwODAx&heure=1770400586&periode=1
```

### response

Headers:
* keep-alive :   timeout=5, max=97
* pragma :   no-cache

```xml
<?xml version="1.0" encoding="utf-8" ?>
<valeurs statut="succes" age_contact="1" heure_contact="1770400651" age_valeurs="3" heure_valeurs="1770400649"  appli="3.0.15">
    <valeur heure="1770400649" donnee="591" valeur="MzQuMCBkQw==" />
    <valeur heure="1770400649" donnee="981" valeur="NjEzOTY=" />
</valeurs>

```

## admin page (b)

### request

* Request URL : http://192.168.0.125/admin/?page=installation&id=SC1Z20230801
* Request Method : GET

Headers:
* cookie : PHPSESSID=4strdulvb7il13o9up0b5mctr1

### response

Status Code : 200 OK

Headers:




## admin page (a)

### request

* Request URL : http://192.168.0.125/admin/
* Request Method : GET

Headers:
* cookie : PHPSESSID=4strdulvb7il13o9up0b5mctr1

### response

Status Code : 302 Found

Headers:
* location : http://192.168.0.125/admin/?page=installation&id=SC1Z20230801 ==> redirect




## login page

### request

* Request URL : http://192.168.0.125/
* Request Method : POST

Headers:
* cache-control : max-age=0
* connection : keep-alive
* content-length : 50
* content-type : application/x-www-form-urlencoded
* cookie : PHPSESSID=4strdulvb7il13o9up0b5mctr1
* host : 192.168.0.125
* origin : http://192.168.0.125
* referer : http://192.168.0.125/


```url
id=util&pass=util&ihm=admin&connexion=Se+connecter
````

### response : 

Status Code : 302 Found

Headers:
* location :  http://192.168.0.125/admin/  ==> redirect



<details>
    <summary>html</summary>

```html
<!doctype html>
<html lang="fr">
	<head>
		<meta http-equiv="content-type" content="text/html" charset="utf-8" />
		<title>SolisArt - Installation SC1Z20230801 / GATTAZ</title>
		<link rel="stylesheet" type="text/css" href="divers/jquery-ui-1.11.2/redmond/jquery-ui.min.css" />
		<link rel="stylesheet" type="text/css" href="divers/css/jquery.datetimepicker-2.4.1.css" />
		<link rel="stylesheet" type="text/css" href="divers/css/mapbox-gl-1.10.1.css" />
		<link rel="stylesheet" type="text/css" href="divers/css/solisart/commun.1670974072.css" />
		<link rel="shortcut icon" href="/favicon.ico" type="image/x-icon">
		<link rel="icon" href="/favicon.ico" type="image/x-icon">
		<script src="divers/js/jquery-1.11.2.min.js" charset="UTF-8"></script>
		<script src="divers/jquery-ui-1.11.2/jquery-ui.min.js" charset="UTF-8"></script>
		<script src="divers/js/jquery.datetimepicker-2.4.1.js" charset="UTF-8"></script>
		<script src="divers/js/mapbox-gl-1.10.1.js" charset="UTF-8"></script>
		<script src="divers/js/latinise.min.js" charset="UTF-8"></script>
		<script>
			var installation = "SC1Z20230801";
		</script>
		<script src="divers/js/solisart/commun-donnees.1705045295.js" charset="UTF-8"></script>
		<script src="divers/js/solisart/commun-fonctions.1702129241.js" charset="UTF-8"></script>
		<script src="divers/js/solisart/page-installation.1702082376.js" charset="UTF-8"></script>
	</head>
	<body>
		<div id="page">
			<div id="contenu">
				<table class="entete">
					<tr>
						<td width="10%"><a href="http://192.168.0.125/admin/"><img src="image/logo-solisart.png" /></a></td>
						<td width="60%" class="entete_titre">Administration des installations</td>
						<td rowspan="2" width="30%" class="entete_authent">
					<form class="formulaire_deconnecter" action="/admin/index.php" method="post">
						Utilisateur : Propri&#233;taire 						<br />
						<input class="input_authent" type="submit" name="deconnexion" value="D&eacute;connexion" title="D&eacute;connexion" />
						<br />
						<div class="message_erreur">&nbsp;</div>
					</form>
						</td>
					</tr>
					<tr>
						<td width="10%" class="version">3.0.15</td>
						<td width="70%" class="entete_menu">&nbsp;&nbsp;&nbsp;<a class="lien_menu" href="/admin/index.php?page=installation&id=SC1Z20230801">Installation</a>&nbsp;&nbsp;<a class="lien_menu" href="/admin/index.php?page=compte">Votre compte</a></td>
					</tr>
				</table>
				<p />
				<div class="titre1">Installation SC1Z20230801 / GATTAZ</div>
				<p />
				<div id="toolbar" class="ui-widget-header ui-corner-all pages-choix">
					<span class="pages-choix" id="div-pages-base">
						<input type="radio" id="input-pages-info" name="div-pages-base" /><label for="input-pages-info">Informations</label>
						<input type="radio" id="input-pages-acces" name="div-pages-base" /><label for="input-pages-acces">Droit d&apos;acc&egrave;s</label>
					</span>
					<span class="pages-choix" id="div-pages-etendu">
						<input type="radio" id="input-pages-utilisateur" name="div-pages-etendu" /><label for="input-pages-utilisateur">Utilisateur</label>
						<input type="radio" id="input-pages-visualisation" name="div-pages-etendu" /><label for="input-pages-visualisation">Visualisation</label>
						<input type="radio" id="input-pages-installateur" name="div-pages-etendu" /><label for="input-pages-installateur">Installateur</label>
						<input type="radio" id="input-pages-maj" name="div-pages-etendu" /><label for="input-pages-maj">Mise &agrave; jour</label>
						<input type="radio" id="input-pages-configuration" name="div-pages-etendu" /><label for="input-pages-configuration">Configuration</label>
						<input type="radio" id="input-pages-administration" name="div-pages-etendu" /><label for="input-pages-administration">Administration</label>
					</span>
				</div>
				<table class="info">
					<tr>
						<td class="comm_statut"><img id="comm-statut" src="image/bullet_red.png" /></td>
						<td class="comm_heure" id="comm-heure" />
						<td id="comm-message" />
					</tr>
				</table>
				<div id="dialog-message" title="Titre">
					<p />
					<span class="ui-icon ui-icon-circle-check" style="float:left; margin:0 7px 50px 0;"></span>
					<div id="dialog-message-texte">Texte</div>
				</div>
				<div id="pages-contenu">
					<div align="center"><img src="image/attente.gif" /></div>
				</div>
			</div>
			<div id="pied">SolisArt - 218, voie Aristide Berg&egrave;s - 73800 Sainte-H&eacute;l&egrave;ne-du-Lac<br />Site Internet : <a href="http://www.solisart.fr" target="_blank">www.solisart.fr</a> - <a href="mailto:contact@solisart.fr" class="adresse"><u>contact@solisart.fr</u></a> - T&eacute;l. : 04 79 60 42 06 - <a href="/admin/index.php?page=mentions_legales">Mentions l&eacute;gales</a></div>
		</div>
	</body>
</html>

```
</details>




## home page

### request

Headers:
* Request URL: http://192.168.0.125/
* Request Method: GET
* Status Code: 200 OK
* Remote Address : 192.168.0.125:80

### response

Headers:
* accept : text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
* accept-encoding : gzip, deflate
* accept-language : fr-FR,fr;q=0.9,en-US;q=0.8,en;q=0.7
* cache-control : max-age=0
* connection : keep-alive
* cookie : PHPSESSID=4strdulvb7il13o9up0b5mctr1
* host : 192.168.0.125
* referer : http://192.168.0.125/

<details>
    <summary>html</summary>


```html
<!doctype html>
<html>
	<head>
		<meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
		<link rel="stylesheet" href="css/boilerplate.css">
		<link rel="stylesheet" href="css/index.css">
		<meta name="viewport" content="initial-scale = 1.0,maximum-scale = 1.0">
		<title>SolisArt, le Soleil partout avec vous !</title>
	</head>
	<body>
		<div id="primaryContainer" class="primaryContainer clearfix">
			<div id="box" class="clearfix">
				<a href="http://192.168.0.125/"><img id="logo" src="img/mysolisartblanc.png" class="image" title="My SolisArt" alt="My SolisArt" />
				<p id="slogan">PILOTEZ VOTRE&nbsp;CHAUFFAGE SOLAIRE</p></a>
				<br />
				<div class="cadre clearfix">
					<div class="etape2" align="center">Authentification</div>
				</div>
				<form action="" method="post" id="form_connexion">
					<div class="cadre clearfix">
						<div class="etape">1. Saisissez votre identitfiant et votre mot de passe</div>
						<label class="texte">Identifiant : <input class="input_texte" id="id" type="text" value="" name="id" /></label>
						<label class="texte">Mot de passe : <input class="input_password"  id="pass" type="password" value="" name="pass" /></label>
					</div>
					<div class="cadre clearfix">
						<div class="etape">2. Choisissez votre interface</div>
						<div align="center">
							<label class="radiolabel"><input type="radio" name="ihm" value="admin" checked="checked" /><img src="img/site_admin.jpg" title="Interface d'administration" /></label>
							<label class="radiolabel"><input type="radio" name="ihm" value="client" /><img src="img/site_client.jpg" title="Interface client" /></label>
						</div>
					</div>
					<div class="cadre clearfix">
						<div class="etape">3. Connectez-vous</div>
						<input class="soumettre" id="connexion" type="submit" value="Se connecter" name="connexion" />
					</div>
				</form>
			</div>
		</div>
	</body>
</html>

```
</details>
