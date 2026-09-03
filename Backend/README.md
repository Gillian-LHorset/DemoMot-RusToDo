# Mise en place du backend

## Mise en place de l'espace de travail

Installez les outils de développement C++ de Visual Studio (Développement Desktop en C++).

Installer Rust depuis le [site officiel](https://rustup.rs/).

## Créer le .env

Copiez le fichier `.env.exemple` et changer l'url pour correspondre à votre container docker.  
(par défaut : `DATABASE_URL=postgres://postgres_user:postgres_password@localhost:5432/rustodo-db`)

## Installation des dépendances

Installez les outils de développement C++ de Visual Studio
Il suffit de bulid le programme avec `rust build`

OU

Installer sous-système Debian  
Update Debian  
Installer rust dans ce système `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` (instal standar)
Dans RustRover : Ajouter une configuration pour la compilation qui run avec `WSL - Debian`
Executez les commandes `which rustc` et `which cargo`
Ajouter l'executable rust et cargo dans les targets du profil RustRover

## Lancer les migrations

Commande pour créer les migrations :

```sh
sqlx database run
```
